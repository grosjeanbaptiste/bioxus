//! In-memory adapter: tests, previews, and apps that need no persistence.

use crate::{Record, Store, StoreError};
use std::collections::BTreeMap;
use std::sync::{Mutex, MutexGuard};

/// Records keyed by (collection, id).
type Table = BTreeMap<(String, String), Record>;

#[derive(Default)]
pub struct MemoryStore(Mutex<Table>);

impl MemoryStore {
    fn lock(&self) -> Result<MutexGuard<'_, Table>, StoreError> {
        self.0
            .lock()
            .map_err(|_| StoreError::Backend("memory store lock poisoned".into()))
    }
}

impl Store for MemoryStore {
    async fn get(&self, collection: &str, id: &str) -> Result<Option<Record>, StoreError> {
        Ok(self
            .lock()?
            .get(&(collection.to_owned(), id.to_owned()))
            .cloned())
    }

    async fn put(&self, record: Record) -> Result<(), StoreError> {
        let mut map = self.lock()?;
        let key = (record.collection.clone(), record.id.clone());
        if map.get(&key).is_none_or(|old| record.stamp >= old.stamp) {
            map.insert(key, record);
        }
        Ok(())
    }

    async fn list(&self, collection: &str) -> Result<Vec<Record>, StoreError> {
        let map = self.lock()?;
        Ok(map
            .values()
            .filter(|r| r.collection == collection && !r.is_deleted())
            .cloned()
            .collect())
    }

    async fn changes_since(&self, after: u64) -> Result<Vec<Record>, StoreError> {
        let mut out: Vec<Record> = self
            .lock()?
            .values()
            .filter(|r| r.stamp > after)
            .cloned()
            .collect();
        out.sort_by(|a, b| (a.stamp, &a.collection, &a.id).cmp(&(b.stamp, &b.collection, &b.id)));
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passes_the_store_contract() {
        crate::contract::block_on(crate::contract::check(&MemoryStore::default()));
    }

    #[test]
    fn a_poisoned_lock_is_an_error_not_a_panic() {
        let store = MemoryStore::default();
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _guard = store.0.lock().unwrap();
            panic!("poison it");
        }));
        let err = crate::contract::block_on(store.get("c", "i")).unwrap_err();
        assert!(matches!(err, StoreError::Backend(m) if m.contains("poisoned")));
    }
}
