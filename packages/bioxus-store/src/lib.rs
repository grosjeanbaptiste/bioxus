//! A `Store` port: opaque string documents in collections, each carrying a caller-chosen
//! `stamp` and a tombstone flag so a sync layer can read "what changed". Adapters
//! (memory here, SQLite behind a feature, IndexedDB in its own step) pass one shared
//! contract suite ([`contract::check`]), which third-party adapters can run too.

pub mod contract;
pub mod error;
#[cfg(all(feature = "indexeddb", target_arch = "wasm32"))]
pub mod indexeddb;
pub mod memory;
pub mod record;
#[cfg(feature = "sqlite")]
pub mod sqlite;

use std::future::Future;

pub use error::StoreError;
#[cfg(all(feature = "indexeddb", target_arch = "wasm32"))]
pub use indexeddb::IndexedDbStore;
pub use memory::MemoryStore;
pub use record::Record;
#[cfg(feature = "sqlite")]
pub use sqlite::SqliteStore;

/// Async so a browser backend fits; native adapters simply complete at once. The futures
/// are not required to be `Send` (browser futures are not); implementors may write `async fn`.
/// Methods take `&self`: adapters manage their own interior mutability.
pub trait Store {
    /// The record, tombstones included; `None` when the id was never written.
    fn get(
        &self,
        collection: &str,
        id: &str,
    ) -> impl Future<Output = Result<Option<Record>, StoreError>>;
    /// Insert or replace by (collection, id). A record whose `stamp` is older than the
    /// stored one is ignored, so replaying a sync batch is harmless.
    fn put(&self, record: Record) -> impl Future<Output = Result<(), StoreError>>;
    /// Live (non-deleted) records of a collection, ordered by id.
    fn list(&self, collection: &str) -> impl Future<Output = Result<Vec<Record>, StoreError>>;
    /// Every record, tombstones included, with `stamp` strictly greater than `after`,
    /// ordered by (stamp, collection, id).
    fn changes_since(&self, after: u64) -> impl Future<Output = Result<Vec<Record>, StoreError>>;
}
