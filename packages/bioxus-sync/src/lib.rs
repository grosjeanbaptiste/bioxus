//! Offline-first sync on any `bioxus-store::Store`: every write gets a hybrid-logical-clock
//! stamp, devices exchange changes through a [`Transport`], and last-write-wins converges
//! them. The transport is a port; [`MemoryTransport`] is the in-process reference.

pub mod clock;
pub mod engine;
pub mod hub;
pub mod transport;

pub use clock::{Clock, ClockError};
pub use engine::{Engine, Report, SyncState};
pub use hub::MemoryTransport;
pub use transport::{Pulled, SyncError, Transport};

#[cfg(test)]
mod tests;
