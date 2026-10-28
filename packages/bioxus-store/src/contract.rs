//! The behaviour every `Store` adapter must have. Run [`check`] from an adapter's own
//! tests: `block_on(check(&MyStore::new()))`. Panics with the broken rule on failure.

use crate::{Record, Store};
use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

/// Drive a future that never really waits (native adapters). Panics if it does.
pub fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = pin!(fut);
    match fut.as_mut().poll(&mut Context::from_waker(Waker::noop())) {
        Poll::Ready(v) => v,
        Poll::Pending => {
            panic!("block_on: future is pending; use a real executor for this adapter")
        }
    }
}

/// Run the whole contract against an empty store.
pub async fn check<S: Store>(s: &S) {
    assert_eq!(s.get("c", "a").await.unwrap(), None, "unknown id is None");
    s.put(Record::new("c", "a", "1", 5)).await.unwrap();
    assert_eq!(
        s.get("c", "a").await.unwrap().unwrap().value.as_deref(),
        Some("1")
    );

    s.put(Record::new("c", "a", "old", 4)).await.unwrap();
    assert_eq!(
        s.get("c", "a").await.unwrap().unwrap().value.as_deref(),
        Some("1"),
        "older stamp is ignored"
    );
    s.put(Record::new("c", "a", "2", 6)).await.unwrap();
    assert_eq!(
        s.get("c", "a").await.unwrap().unwrap().value.as_deref(),
        Some("2"),
        "newer stamp wins"
    );

    s.put(Record::new("c", "b", "x", 7)).await.unwrap();
    s.put(Record::new("other", "z", "y", 8)).await.unwrap();
    let ids: Vec<_> = s
        .list("c")
        .await
        .unwrap()
        .into_iter()
        .map(|r| r.id)
        .collect();
    assert_eq!(ids, ["a", "b"], "list is per collection and ordered by id");

    s.put(Record::tombstone("c", "a", 9)).await.unwrap();
    assert!(
        s.get("c", "a").await.unwrap().unwrap().is_deleted(),
        "tombstone is readable"
    );
    assert_eq!(s.list("c").await.unwrap().len(), 1, "list hides tombstones");

    let changes = s.changes_since(7).await.unwrap();
    let got: Vec<_> = changes.iter().map(|r| (r.stamp, r.id.as_str())).collect();
    assert_eq!(
        got,
        [(8, "z"), (9, "a")],
        "changes are > after, tombstones included, ordered"
    );
    assert!(s.changes_since(9).await.unwrap().is_empty());

    let same = Record::new("c", "b", "same-stamp", 7);
    s.put(same.clone()).await.unwrap();
    assert_eq!(
        s.get("c", "b").await.unwrap().unwrap(),
        same,
        "equal stamp replaces (idempotent replay)"
    );
}
