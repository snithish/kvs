#![deny(missing_docs)]
//! Crate for fast Key Value Store

extern crate core;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use super::error::Result;
use crate::kv::Command::{Remove, Set};

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

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "command")]
enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}

impl KvStore {
    /// Instantiates a new store
    pub fn open(file_path: impl Into<PathBuf>) -> Result<KvStore> {
        let log_path = file_path.into().join("temp.log");
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(log_path)?;
        Ok(KvStore {
            file,
            store: HashMap::new(),
        })
    }
    /// Create a mapping between`key` and `value`
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.store.insert(key.clone(), value.clone());
        let set_command: Command = Set { key, value };
        write!(self.file, "{}", serde_json::to_string(&set_command)?)?;
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
        let remove_command: Command = Remove { key };
        write!(self.file, "{}", serde_json::to_string(&remove_command)?)?;
        Ok(())
    }
}
