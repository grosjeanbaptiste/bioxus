//! Browser adapter on IndexedDB. Object store `records` is keyed by [collection, id].
//! JS numbers hold 53 bits, a stamp needs 64: it is stored as a 20-digit decimal string,
//! which sorts like the number, and the `stamp` index orders `changes_since` with it.

use crate::{Record, Store, StoreError};
use idb::{
    Database, DatabaseEvent, Factory, IndexParams, KeyPath, KeyRange, ObjectStoreParams, Query,
    TransactionMode,
};
use js_sys::Reflect;
use wasm_bindgen::{JsCast, JsValue};

const STORE: &str = "records";

pub struct IndexedDbStore(Database);

fn backend<E: std::fmt::Display>(e: E) -> StoreError {
    StoreError::Backend(e.to_string())
}

fn stamp_key(stamp: u64) -> String {
    format!("{stamp:020}")
}

fn field(obj: &JsValue, name: &str) -> Result<JsValue, StoreError> {
    Reflect::get(obj, &JsValue::from_str(name))
        .map_err(|e| StoreError::Backend(format!("record has no `{name}`: {e:?}")))
}

fn text(obj: &JsValue, name: &str) -> Result<String, StoreError> {
    field(obj, name)?
        .as_string()
        .ok_or_else(|| StoreError::Backend(format!("record field `{name}` is not a string")))
}

fn to_js(r: &Record) -> Result<JsValue, StoreError> {
    let o = js_sys::Object::new();
    let set = |k: &str, v: JsValue| {
        Reflect::set(&o, &JsValue::from_str(k), &v)
            .map(|_| ())
            .map_err(|e| StoreError::Backend(format!("{e:?}")))
    };
    set("collection", JsValue::from_str(&r.collection))?;
    set("id", JsValue::from_str(&r.id))?;
    set(
        "value",
        r.value.as_deref().map_or(JsValue::NULL, JsValue::from_str),
    )?;
    set("stamp", JsValue::from_str(&stamp_key(r.stamp)))?;
    Ok(o.unchecked_into())
}

fn from_js(o: &JsValue) -> Result<Record, StoreError> {
    let stamp = text(o, "stamp")?.parse::<u64>().map_err(backend)?;
    let value = field(o, "value")?.as_string();
    Ok(Record {
        collection: text(o, "collection")?,
        id: text(o, "id")?,
        value,
        stamp,
    })
}

impl IndexedDbStore {
    /// Open (creating on first use) the database called `name`.
    pub async fn open(name: &str) -> Result<Self, StoreError> {
        let mut request = Factory::new()
            .map_err(backend)?
            .open(name, Some(1))
            .map_err(backend)?;
        request.on_upgrade_needed(|event| {
            // An upgrade failure is reported by the open request itself, so it surfaces below.
            if let Ok(db) = event.database() {
                let mut params = ObjectStoreParams::new();
                params.key_path(Some(KeyPath::new_array(["collection", "id"])));
                if let Ok(store) = db.create_object_store(STORE, params) {
                    let _ = store.create_index(
                        "stamp",
                        KeyPath::new_single("stamp"),
                        Some(IndexParams::new()),
                    ); // upgrade errors abort the open request
                }
            }
        });
        Ok(Self(request.await.map_err(backend)?))
    }

    fn tx(&self, mode: TransactionMode) -> Result<idb::ObjectStore, StoreError> {
        self.0
            .transaction(&[STORE], mode)
            .map_err(backend)?
            .object_store(STORE)
            .map_err(backend)
    }
}

impl Store for IndexedDbStore {
    async fn get(&self, collection: &str, id: &str) -> Result<Option<Record>, StoreError> {
        let key = js_sys::Array::of2(&JsValue::from_str(collection), &JsValue::from_str(id));
        let found = self
            .tx(TransactionMode::ReadOnly)?
            .get(Query::Key(key.into()))
            .map_err(backend)?
            .await
            .map_err(backend)?;
        found.as_ref().map(from_js).transpose()
    }

    async fn put(&self, record: Record) -> Result<(), StoreError> {
        // One readwrite transaction: the stamp check and the write cannot interleave.
        let store = self.tx(TransactionMode::ReadWrite)?;
        let key = js_sys::Array::of2(
            &JsValue::from_str(&record.collection),
            &JsValue::from_str(&record.id),
        );
        if let Some(old) = store
            .get(Query::Key(key.into()))
            .map_err(backend)?
            .await
            .map_err(backend)?
        {
            if from_js(&old)?.stamp > record.stamp {
                return Ok(());
            }
        }
        store
            .put(&to_js(&record)?, None)
            .map_err(backend)?
            .await
            .map_err(backend)?;
        Ok(())
    }

    async fn list(&self, collection: &str) -> Result<Vec<Record>, StoreError> {
        let all = self
            .tx(TransactionMode::ReadOnly)?
            .get_all(None, None)
            .map_err(backend)?
            .await
            .map_err(backend)?;
        let mut out = all.iter().map(from_js).collect::<Result<Vec<_>, _>>()?;
        out.retain(|r| r.collection == collection && !r.is_deleted());
        out.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(out)
    }

    async fn changes_since(&self, after: u64) -> Result<Vec<Record>, StoreError> {
        let index = self
            .tx(TransactionMode::ReadOnly)?
            .index("stamp")
            .map_err(backend)?;
        let lower = KeyRange::lower_bound(&JsValue::from_str(&stamp_key(after)), Some(true))
            .map_err(backend)?;
        let rows = index
            .get_all(Some(Query::KeyRange(lower)), None)
            .map_err(backend)?
            .await
            .map_err(backend)?;
        rows.iter().map(from_js).collect()
    }
}
