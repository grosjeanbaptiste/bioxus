/// One stored document. `value` is opaque text (JSON, usually); `None` is a tombstone.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record {
    pub collection: String,
    pub id: String,
    pub value: Option<String>,
    /// Version chosen by the caller (a logical clock, for sync). Higher wins on `put`.
    pub stamp: u64,
}

impl Record {
    pub fn new(collection: &str, id: &str, value: &str, stamp: u64) -> Self {
        Self {
            collection: collection.into(),
            id: id.into(),
            value: Some(value.into()),
            stamp,
        }
    }

    /// A deletion marker, kept so the deletion can be synced.
    pub fn tombstone(collection: &str, id: &str, stamp: u64) -> Self {
        Self {
            collection: collection.into(),
            id: id.into(),
            value: None,
            stamp,
        }
    }

    #[must_use]
    pub fn is_deleted(&self) -> bool {
        self.value.is_none()
    }
}
