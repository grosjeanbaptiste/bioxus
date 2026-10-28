//! The wire port. HTTP, WebSocket, p2p or a file drop all fit behind it.

use crate::clock::ClockError;
use bioxus_store::{Record, StoreError};
use std::future::Future;

/// What a pull returned, with the token to resume from. The token is the *server's* own
/// arrival order, not a record stamp: a device that was offline pushes old stamps late,
/// and a stamp-based cursor would skip them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pulled {
    pub records: Vec<Record>,
    pub next: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SyncError {
    Store(StoreError),
    Clock(ClockError),
    /// The transport failed (network, server refusal ...); retrying is safe.
    Transport(String),
}

impl From<StoreError> for SyncError {
    fn from(e: StoreError) -> Self {
        Self::Store(e)
    }
}

impl From<ClockError> for SyncError {
    fn from(e: ClockError) -> Self {
        Self::Clock(e)
    }
}

impl std::fmt::Display for SyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Store(e) => write!(f, "sync: {e}"),
            Self::Clock(e) => write!(f, "sync: {e}"),
            Self::Transport(e) => write!(f, "sync transport failed: {e}"),
        }
    }
}

impl std::error::Error for SyncError {}

/// Both calls must be idempotent: the same batch may arrive twice after a failed sync.
pub trait Transport {
    fn push(&self, records: Vec<Record>) -> impl Future<Output = Result<(), SyncError>>;
    /// Records the server accepted after arrival position `after` (0 = from the start).
    fn pull(&self, after: u64) -> impl Future<Output = Result<Pulled, SyncError>>;
}
