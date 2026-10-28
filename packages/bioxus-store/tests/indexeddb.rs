//! Runs the shared Store contract against a real browser IndexedDB:
//! `wasm-pack test --headless --chrome packages/bioxus-store --features indexeddb`.
#![cfg(all(feature = "indexeddb", target_arch = "wasm32"))]

use bioxus_store::{contract, IndexedDbStore, Record, Store};
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

fn fresh_name(tag: &str) -> String {
    format!("bioxus-test-{tag}-{}", js_sys::Date::now())
}

#[wasm_bindgen_test]
async fn passes_the_store_contract() {
    let store = IndexedDbStore::open(&fresh_name("contract")).await.unwrap();
    contract::check(&store).await;
}

#[wasm_bindgen_test]
async fn stamps_beyond_2_pow_53_keep_their_order_and_value() {
    let store = IndexedDbStore::open(&fresh_name("big")).await.unwrap();
    let (low, high) = ((1u64 << 53) + 1, (1u64 << 53) + 2);
    store.put(Record::new("c", "a", "hi", high)).await.unwrap();
    store.put(Record::new("c", "b", "lo", low)).await.unwrap();
    let got: Vec<_> = store
        .changes_since(1 << 53)
        .await
        .unwrap()
        .iter()
        .map(|r| (r.stamp, r.id.clone()))
        .collect();
    assert_eq!(got, [(low, "b".to_string()), (high, "a".to_string())]);
}
