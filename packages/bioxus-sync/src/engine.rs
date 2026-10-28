//! Writes stamp locally; `sync` pushes what changed here, then pulls what changed there.
//! Conflicts resolve by last-write-wins on the stamp, applied by the `Store` itself.

use crate::clock::{node_of, Clock};
use crate::transport::{SyncError, Transport};
use bioxus_store::{Record, Store};

/// Progress markers. Persist them if you want to resume cheaply; losing them only costs
/// a full (idempotent) resync.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SyncState {
    /// Highest local stamp already handed to the transport.
    pub pushed: u64,
    /// Transport arrival token to resume pulling from.
    pub pulled: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Report {
    pub pushed: usize,
    pub pulled: usize,
}

pub struct Engine<S, T> {
    store: S,
    transport: T,
    clock: Clock,
    now_ms: Box<dyn Fn() -> u64>,
    pub state: SyncState,
}

impl<S: Store, T: Transport> Engine<S, T> {
    /// Open on an existing store: the clock is seeded from the newest stamp it holds, so a
    /// device whose wall clock went back never stamps below its own history.
    // ponytail: reads every change to find the newest stamp; add Store::max_stamp if stores get big.
    pub async fn open(
        node: u8,
        store: S,
        transport: T,
        now_ms: impl Fn() -> u64 + 'static,
    ) -> Result<Self, SyncError> {
        let mut clock = Clock::new(node);
        let now = now_ms();
        for r in store.changes_since(0).await? {
            clock.observe(r.stamp, now)?;
        }
        Ok(Self {
            store,
            transport,
            clock,
            now_ms: Box::new(now_ms),
            state: SyncState::default(),
        })
    }

    pub fn store(&self) -> &S {
        &self.store
    }

    /// Give the store back, e.g. to reopen the engine with another transport.
    pub fn into_store(self) -> S {
        self.store
    }

    pub async fn put(
        &mut self,
        collection: &str,
        id: &str,
        value: &str,
    ) -> Result<Record, SyncError> {
        let record = Record::new(collection, id, value, self.clock.tick((self.now_ms)())?);
        self.store.put(record.clone()).await?;
        Ok(record)
    }

    pub async fn delete(&mut self, collection: &str, id: &str) -> Result<Record, SyncError> {
        let record = Record::tombstone(collection, id, self.clock.tick((self.now_ms)())?);
        self.store.put(record.clone()).await?;
        Ok(record)
    }

    /// Push then pull. On any error the markers stay where they were, so calling again
    /// retries exactly what did not finish.
    pub async fn sync(&mut self) -> Result<Report, SyncError> {
        let changes = self.store.changes_since(self.state.pushed).await?;
        let newest = changes.iter().map(|r| r.stamp).max();
        // Only what this node wrote: records pulled from elsewhere are not echoed back.
        let node = self.clock.node();
        let outgoing: Vec<Record> = changes
            .into_iter()
            .filter(|r| node_of(r.stamp) == node)
            .collect();
        let pushed = outgoing.len();
        if !outgoing.is_empty() {
            self.transport.push(outgoing).await?;
        }
        let incoming = self.transport.pull(self.state.pulled).await?;
        let now = (self.now_ms)();
        for r in &incoming.records {
            self.clock.observe(r.stamp, now)?;
        }
        for r in &incoming.records {
            self.store.put(r.clone()).await?;
        }
        self.state.pushed = newest.unwrap_or(self.state.pushed).max(self.state.pushed);
        self.state.pulled = incoming.next;
        Ok(Report {
            pushed,
            pulled: incoming.records.len(),
        })
    }
}
