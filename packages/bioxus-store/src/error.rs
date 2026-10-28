/// A storage failure. Adapters never swallow one; they return it.
#[derive(Debug, PartialEq, Eq)]
pub enum StoreError {
    /// The backend failed (I/O, SQL, lock poisoned, quota ...).
    Backend(String),
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Backend(e) => write!(f, "store backend failed: {e}"),
        }
    }
}

impl std::error::Error for StoreError {}
