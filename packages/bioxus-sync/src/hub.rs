//! In-process server: a shared arrival log with last-write-wins. For tests, demos and
//! apps whose devices share a process; also the reference for what a real server does.

use crate::transport::{Pulled, SyncError, Transport};
use bioxus_store::Record;
use std::sync::{Arc, Mutex};

/// Clone it to give each engine a connection to the same hub.
#[derive(Clone, Default)]
pub struct MemoryTransport {
    log: Arc<Mutex<Vec<Record>>>,
    /// Test hook: while true, every call fails like a dropped network.
    pub offline: Arc<Mutex<bool>>,
}

impl MemoryTransport {
    fn check_online(&self) -> Result<(), SyncError> {
        let down = *self
            .offline
            .lock()
            .map_err(|_| SyncError::Transport("hub lock poisoned".into()))?;
        if down {
            Err(SyncError::Transport("offline".into()))
        } else {
            Ok(())
        }
    }
}

impl Transport for MemoryTransport {
    async fn push(&self, records: Vec<Record>) -> Result<(), SyncError> {
        self.check_online()?;
        let mut log = self
            .log
            .lock()
            .map_err(|_| SyncError::Transport("hub lock poisoned".into()))?;
        for r in records {
            let current = log
                .iter()
                .filter(|o| o.collection == r.collection && o.id == r.id)
                .map(|o| o.stamp)
                .max();
            if current.is_none_or(|s| r.stamp > s) {
                log.push(r);
            }
        }
        Ok(())
    }

    async fn pull(&self, after: u64) -> Result<Pulled, SyncError> {
        self.check_online()?;
        let log = self
            .log
            .lock()
            .map_err(|_| SyncError::Transport("hub lock poisoned".into()))?;
        let from = usize::try_from(after).unwrap_or(usize::MAX).min(log.len());
        Ok(Pulled {
            records: log[from..].to_vec(),
            next: log.len() as u64,
        })
    }
}
