//! Hybrid logical clock. A stamp is `[44-bit ms | 12-bit counter | 8-bit node]` in a `u64`.
//! Stamps from different nodes never tie (the node id breaks it), so last-write-wins
//! converges everywhere. The wall clock is injected: nothing here reads the time.

const NODE_BITS: u32 = 8;
const COUNTER_BITS: u32 = 12;
const LOGICAL_BITS: u32 = 64 - NODE_BITS;
/// A remote stamp this far ahead of our wall clock is refused, so one broken device
/// cannot drag every clock into the future.
pub const MAX_DRIFT_MS: u64 = 24 * 60 * 60 * 1000;

#[derive(Debug, PartialEq, Eq)]
pub enum ClockError {
    /// The remote stamp is further ahead than [`MAX_DRIFT_MS`].
    Drift { remote_ms: u64, now_ms: u64 },
    /// Wall time or logical time no longer fits in the stamp (year ~2527).
    OutOfRange,
}

impl std::fmt::Display for ClockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Drift { remote_ms, now_ms } => write!(
                f,
                "remote stamp is {} ms ahead of this clock",
                remote_ms - now_ms
            ),
            Self::OutOfRange => write!(f, "clock value does not fit in a stamp"),
        }
    }
}

impl std::error::Error for ClockError {}

/// The node that issued `stamp`.
#[must_use]
pub const fn node_of(stamp: u64) -> u8 {
    (stamp & ((1 << NODE_BITS) - 1)) as u8
}

#[derive(Debug)]
pub struct Clock {
    node: u8,
    /// Highest logical time (ms and counter, without the node) issued or observed.
    last: u64,
}

fn fits(logical: u64) -> Result<u64, ClockError> {
    if logical >> LOGICAL_BITS == 0 {
        Ok(logical)
    } else {
        Err(ClockError::OutOfRange)
    }
}

impl Clock {
    #[must_use]
    pub const fn node(&self) -> u8 {
        self.node
    }

    #[must_use]
    pub fn new(node: u8) -> Self {
        Self { node, last: 0 }
    }

    /// A new stamp, strictly greater than every stamp issued or observed so far.
    pub fn tick(&mut self, now_ms: u64) -> Result<u64, ClockError> {
        let wall = fits(
            now_ms
                .checked_shl(COUNTER_BITS)
                .ok_or(ClockError::OutOfRange)?,
        )?;
        let next = fits(wall.max(self.last.checked_add(1).ok_or(ClockError::OutOfRange)?))?;
        self.last = next;
        Ok(next << NODE_BITS | u64::from(self.node))
    }

    /// Absorb a stamp received from elsewhere, so later ticks order after it.
    pub fn observe(&mut self, remote: u64, now_ms: u64) -> Result<(), ClockError> {
        let logical = remote >> NODE_BITS;
        let remote_ms = logical >> COUNTER_BITS;
        if remote_ms > now_ms.saturating_add(MAX_DRIFT_MS) {
            return Err(ClockError::Drift { remote_ms, now_ms });
        }
        self.last = self.last.max(logical);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ticks_increase_even_when_the_wall_clock_stalls_or_goes_back() {
        let mut c = Clock::new(1);
        let a = c.tick(1_000).unwrap();
        let b = c.tick(1_000).unwrap();
        let back = c.tick(500).unwrap();
        assert!(a < b && b < back);
    }

    #[test]
    fn the_node_id_breaks_ties_between_devices() {
        let (a, b) = (
            Clock::new(1).tick(7).unwrap(),
            Clock::new(2).tick(7).unwrap(),
        );
        assert_ne!(a, b);
        assert_eq!((a >> 8, a & 0xff), (b >> 8, 1));
    }

    #[test]
    fn the_issuing_node_is_recoverable_from_a_stamp() {
        for node in [0, 1, 200, 255] {
            assert_eq!(node_of(Clock::new(node).tick(123_456).unwrap()), node);
        }
    }

    #[test]
    fn observing_a_remote_stamp_orders_later_ticks_after_it() {
        let remote = Clock::new(9).tick(5_000).unwrap();
        let mut c = Clock::new(1);
        c.observe(remote, 1_000).unwrap();
        assert!(c.tick(1_000).unwrap() > remote);
    }

    #[test]
    fn a_stamp_too_far_in_the_future_is_refused_and_changes_nothing() {
        let far = Clock::new(9).tick(1_000 + MAX_DRIFT_MS + 1).unwrap();
        let mut c = Clock::new(1);
        assert!(matches!(
            c.observe(far, 1_000),
            Err(ClockError::Drift { .. })
        ));
        assert!(c.tick(1_000).unwrap() < far);
    }

    #[test]
    fn out_of_range_time_is_an_error_not_a_wrap() {
        assert_eq!(Clock::new(1).tick(1 << 44), Err(ClockError::OutOfRange));
        assert_eq!(Clock::new(1).tick(u64::MAX), Err(ClockError::OutOfRange));
    }
}
