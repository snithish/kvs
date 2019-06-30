#![deny(missing_docs)]

//! Crate for fast Key Value Store

use std::collections::HashMap;

/// Used to represent a in-memory key value store
/// # Examples
///
/// ```no_run
/// use kvs::KvStore;
/// fn kv_store_test() {
///     let mut store = KvStore::new();
///     store.set("key".to_owned(), "value".to_owned());
///     store.get("key".to_owned());
/// }
/// ```
#[allow(non_snake_case)]
#[derive(Default)]
pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    /// Instantiates a new store
    pub fn new() -> KvStore {
        Default::default()
    }
    /// Create a mapping between`key` and `value`
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }
    /// Return value associated with `key`, Returns `None` when `key`
    /// not present in store
    pub fn get(&self, key: String) -> Option<String> {
        self.store.get(key.as_str()).cloned()
    }
    /// Delete key denoted `key`
    pub fn remove(&mut self, key: String) {
        self.store.remove(key.as_str());
    }
}
