#![deny(missing_docs)]
//! Crate for fast Key Value Store

extern crate core;

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use super::error::Result;

/// Used to represent a in-memory key value store
/// # Examples
///
/// ```no_run
/// use kvs::KvStore;
/// use std::path::Path;
/// fn kv_store_test() {
///     let file_path = Path::new("~/path/to/file");
///     let mut store = KvStore::open(file_path);
///     store.set("key".to_owned(), "value".to_owned());
///     store.get("key".to_owned());
/// }
/// ```
#[allow(non_snake_case)]
pub struct KvStore {
    file: File,
    store: HashMap<String, String>,
}

impl KvStore {
    /// Instantiates a new store
    pub fn open(file_path: impl Into<PathBuf>) -> Result<KvStore> {
        let log_path = file_path.into().join("temp.log");
        let file = File::create(log_path)?;
        Ok(KvStore {
            file,
            store: HashMap::new(),
        })
    }
    /// Create a mapping between`key` and `value`
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        write!(self.file, "some mutation");
        self.store.insert(key, value);
        Ok(())
    }
    /// Return value associated with `key`, Returns `None` when `key`
    /// not present in store
    pub fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self.store.get(key.as_str()).cloned())
    }
    /// Delete key denoted `key`
    pub fn remove(&mut self, key: String) -> Result<()> {
        self.store.remove(key.as_str());
        Ok(())
    }
}
