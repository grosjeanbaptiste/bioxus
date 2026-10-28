//! SQLite adapter (desktop, mobile). Blocking calls run inside the async methods.
// ponytail: blocking in async; wrap in spawn_blocking if an app runs it on a shared executor.

use crate::{Record, Store, StoreError};
use rusqlite::{params, Connection};
use std::path::Path;
use std::sync::{Mutex, MutexGuard};

const SCHEMA: &str = "CREATE TABLE IF NOT EXISTS records (
    collection TEXT NOT NULL, id TEXT NOT NULL, value TEXT, stamp INTEGER NOT NULL,
    PRIMARY KEY (collection, id)) WITHOUT ROWID;
    CREATE INDEX IF NOT EXISTS records_stamp ON records (stamp);";

pub struct SqliteStore(Mutex<Connection>);

fn backend<E: std::fmt::Display>(e: E) -> StoreError {
    StoreError::Backend(e.to_string())
}

fn stamp_i64(stamp: u64) -> Result<i64, StoreError> {
    i64::try_from(stamp).map_err(|_| StoreError::Backend(format!("stamp {stamp} exceeds i64::MAX")))
}

fn row(r: &rusqlite::Row<'_>) -> rusqlite::Result<Record> {
    let stamp: i64 = r.get(3)?;
    // A negative stamp can only come from outside this adapter: surface it, never clamp it.
    let stamp = u64::try_from(stamp).map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(3, rusqlite::types::Type::Integer, Box::new(e))
    })?;
    Ok(Record {
        collection: r.get(0)?,
        id: r.get(1)?,
        value: r.get(2)?,
        stamp,
    })
}

impl SqliteStore {
    pub fn open(path: &Path) -> Result<Self, StoreError> {
        Self::init(Connection::open(path).map_err(backend)?)
    }

    pub fn open_in_memory() -> Result<Self, StoreError> {
        Self::init(Connection::open_in_memory().map_err(backend)?)
    }

    fn init(conn: Connection) -> Result<Self, StoreError> {
        conn.execute_batch(SCHEMA).map_err(backend)?;
        Ok(Self(Mutex::new(conn)))
    }

    fn conn(&self) -> Result<MutexGuard<'_, Connection>, StoreError> {
        self.0
            .lock()
            .map_err(|_| StoreError::Backend("sqlite connection lock poisoned".into()))
    }

    fn query(&self, sql: &str, args: impl rusqlite::Params) -> Result<Vec<Record>, StoreError> {
        let conn = self.conn()?;
        let mut stmt = conn.prepare(sql).map_err(backend)?;
        // Collected into a local so the row iterator, which borrows `stmt`, drops before it.
        let rows = stmt
            .query_map(args, row)
            .map_err(backend)?
            .collect::<Result<Vec<_>, _>>();
        rows.map_err(backend)
    }
}

impl Store for SqliteStore {
    async fn get(&self, collection: &str, id: &str) -> Result<Option<Record>, StoreError> {
        let rows = self.query(
            "SELECT collection, id, value, stamp FROM records WHERE collection = ?1 AND id = ?2",
            params![collection, id],
        )?;
        Ok(rows.into_iter().next())
    }

    async fn put(&self, r: Record) -> Result<(), StoreError> {
        let stamp = stamp_i64(r.stamp)?;
        self.conn()?
            .execute(
                "INSERT INTO records (collection, id, value, stamp) VALUES (?1, ?2, ?3, ?4)
                 ON CONFLICT (collection, id) DO UPDATE SET value = excluded.value, stamp = excluded.stamp
                 WHERE excluded.stamp >= records.stamp",
                params![r.collection, r.id, r.value, stamp],
            )
            .map_err(backend)?;
        Ok(())
    }

    async fn list(&self, collection: &str) -> Result<Vec<Record>, StoreError> {
        self.query("SELECT collection, id, value, stamp FROM records WHERE collection = ?1 AND value IS NOT NULL ORDER BY id", params![collection])
    }

    async fn changes_since(&self, after: u64) -> Result<Vec<Record>, StoreError> {
        self.query("SELECT collection, id, value, stamp FROM records WHERE stamp > ?1 ORDER BY stamp, collection, id", params![stamp_i64(after)?])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract::{block_on, check};

    #[test]
    fn passes_the_store_contract() {
        block_on(check(&SqliteStore::open_in_memory().unwrap()));
    }

    #[test]
    fn data_survives_reopening_the_file() {
        let path = std::env::temp_dir().join(format!("bioxus-store-{}.db", std::process::id()));
        let _ = std::fs::remove_file(&path); // fresh start; a missing file is the normal case
        block_on(
            SqliteStore::open(&path)
                .unwrap()
                .put(Record::new("c", "a", "kept", 1)),
        )
        .unwrap();
        let got = block_on(SqliteStore::open(&path).unwrap().get("c", "a")).unwrap();
        std::fs::remove_file(&path).unwrap();
        assert_eq!(got.unwrap().value.as_deref(), Some("kept"));
    }

    #[test]
    fn a_stamp_beyond_i64_is_an_error_not_a_wrap() {
        let s = SqliteStore::open_in_memory().unwrap();
        let err = block_on(s.put(Record::new("c", "a", "x", u64::MAX))).unwrap_err();
        assert!(matches!(err, StoreError::Backend(m) if m.contains("exceeds")));
        assert!(block_on(s.changes_since(u64::MAX)).is_err());
    }

    #[test]
    fn opening_an_unwritable_path_fails_loudly() {
        assert!(SqliteStore::open(Path::new("/nonexistent-dir/x.db")).is_err());
    }
}
