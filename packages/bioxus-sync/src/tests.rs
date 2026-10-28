use super::*;
use bioxus_store::contract::block_on;
use bioxus_store::{MemoryStore, Store};
use std::cell::Cell;
use std::rc::Rc;

type Dev = Engine<MemoryStore, MemoryTransport>;

fn device(node: u8, hub: &MemoryTransport, clock: &Rc<Cell<u64>>) -> Dev {
    let clock = Rc::clone(clock);
    block_on(Engine::open(
        node,
        MemoryStore::default(),
        hub.clone(),
        move || clock.get(),
    ))
    .unwrap()
}

fn value(d: &Dev, id: &str) -> Option<String> {
    block_on(d.store().get("c", id))
        .unwrap()
        .and_then(|r| r.value)
}

#[test]
fn two_devices_converge_on_each_others_writes() {
    let (hub, now) = (MemoryTransport::default(), Rc::new(Cell::new(1_000)));
    let (mut a, mut b) = (device(1, &hub, &now), device(2, &hub, &now));
    block_on(a.put("c", "x", "from-a")).unwrap();
    block_on(a.sync()).unwrap();
    assert_eq!(
        block_on(b.sync()).unwrap(),
        Report {
            pushed: 0,
            pulled: 1
        }
    );
    assert_eq!(value(&b, "x").as_deref(), Some("from-a"));
}

#[test]
fn concurrent_edits_resolve_the_same_way_on_both_sides() {
    let (hub, now) = (MemoryTransport::default(), Rc::new(Cell::new(1_000)));
    let (mut a, mut b) = (device(1, &hub, &now), device(2, &hub, &now));
    block_on(a.put("c", "x", "a")).unwrap();
    block_on(b.put("c", "x", "b")).unwrap(); // same millisecond: the node id decides
    for _ in 0..2 {
        block_on(a.sync()).unwrap();
        block_on(b.sync()).unwrap();
    }
    assert_eq!(value(&a, "x"), value(&b, "x"));
    assert_eq!(value(&a, "x").as_deref(), Some("b"));
}

#[test]
fn a_late_push_with_an_old_stamp_still_reaches_everyone() {
    let (hub, now) = (MemoryTransport::default(), Rc::new(Cell::new(1_000)));
    let (mut a, mut b, mut c) = (
        device(1, &hub, &now),
        device(2, &hub, &now),
        device(3, &hub, &now),
    );
    block_on(a.put("c", "old", "written-offline")).unwrap();
    now.set(900_000);
    block_on(b.put("c", "new", "later")).unwrap();
    block_on(b.sync()).unwrap();
    block_on(c.sync()).unwrap(); // c's pull marker is now past b's record
    block_on(a.sync()).unwrap(); // a finally pushes its much older stamp
    block_on(c.sync()).unwrap();
    assert_eq!(value(&c, "old").as_deref(), Some("written-offline"));
}

#[test]
fn deletes_propagate_as_tombstones() {
    let (hub, now) = (MemoryTransport::default(), Rc::new(Cell::new(1_000)));
    let (mut a, mut b) = (device(1, &hub, &now), device(2, &hub, &now));
    block_on(a.put("c", "x", "v")).unwrap();
    block_on(a.sync()).unwrap();
    block_on(b.sync()).unwrap();
    now.set(2_000);
    block_on(b.delete("c", "x")).unwrap();
    block_on(b.sync()).unwrap();
    block_on(a.sync()).unwrap();
    assert!(block_on(a.store().list("c")).unwrap().is_empty());
}

#[test]
fn a_failed_sync_changes_nothing_and_the_retry_completes_it() {
    let (hub, now) = (MemoryTransport::default(), Rc::new(Cell::new(1_000)));
    let (mut a, mut b) = (device(1, &hub, &now), device(2, &hub, &now));
    block_on(a.put("c", "x", "v")).unwrap();
    *hub.offline.lock().unwrap() = true;
    assert!(matches!(block_on(a.sync()), Err(SyncError::Transport(_))));
    assert_eq!(a.state, SyncState::default());
    *hub.offline.lock().unwrap() = false;
    block_on(a.sync()).unwrap();
    block_on(b.sync()).unwrap();
    assert_eq!(value(&b, "x").as_deref(), Some("v"));
}

#[test]
fn syncing_twice_is_idempotent() {
    let (hub, now) = (MemoryTransport::default(), Rc::new(Cell::new(1_000)));
    let (mut a, mut b) = (device(1, &hub, &now), device(2, &hub, &now));
    block_on(a.put("c", "x", "v")).unwrap();
    block_on(a.sync()).unwrap();
    block_on(b.sync()).unwrap();
    assert_eq!(block_on(a.sync()).unwrap().pushed, 0);
    assert_eq!(
        block_on(b.sync()).unwrap(),
        Report {
            pushed: 0,
            pulled: 0
        }
    );
}

#[test]
fn a_poisoned_remote_stamp_is_refused_loudly() {
    let (hub, now) = (MemoryTransport::default(), Rc::new(Cell::new(1_000)));
    let mut a = device(1, &hub, &now);
    let far = Clock::new(9).tick(1_000 + clock::MAX_DRIFT_MS + 1).unwrap();
    block_on(hub.push(vec![bioxus_store::Record::new("c", "x", "evil", far)])).unwrap();
    assert!(matches!(block_on(a.sync()), Err(SyncError::Clock(_))));
    assert_eq!(value(&a, "x"), None);
}

#[test]
fn reopening_a_store_keeps_stamps_above_its_history() {
    let (hub, now) = (MemoryTransport::default(), Rc::new(Cell::new(50_000)));
    let mut a = device(1, &hub, &now);
    let first = block_on(a.put("c", "x", "1")).unwrap().stamp;
    now.set(10); // the wall clock jumped far back
    let clock = Rc::clone(&now);
    let mut again = block_on(Engine::open(1, a.into_store(), hub.clone(), move || {
        clock.get()
    }))
    .unwrap();
    assert!(block_on(again.put("c", "x", "2")).unwrap().stamp > first);
}
