#![deny(missing_docs)]
//! Crate for fast Key Value Store

extern crate core;

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::kv::Command::{Remove, Set};

use super::error::Result;
use crate::KvError;

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

impl From<String> for Command {
    fn from(line_item: String) -> Self {
        serde_json::from_str(&line_item).unwrap()
    }
}

const WAL_FILE_NAME: &str = "wal.log";

impl KvStore {
    /// Instantiates a new store
    pub fn open(file_path: impl Into<PathBuf>) -> Result<KvStore> {
        let log_path = file_path.into().join(WAL_FILE_NAME);
        let constructed_map = KvStore::construct_hash_map(&log_path);
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(log_path)?;
        Ok(KvStore {
            file,
            store: constructed_map,
        })
    }

    fn construct_hash_map(log_path: &PathBuf) -> HashMap<String, String> {
        OpenOptions::new()
            .read(true)
            .open(log_path)
            .map(|file| {
                let reader = BufReader::new(file);
                let commands: Vec<Command> = reader
                    .lines()
                    .map(|line| Command::from(line.unwrap()))
                    .collect();
                let mut map = HashMap::new();
                commands.iter().for_each(|x| match x {
                    Set { key, value } => {
                        map.insert(key.clone(), value.clone());
                    }
                    Remove { key } => {
                        map.remove(key);
                    }
                });
                map
            })
            .unwrap_or_default()
    }

    /// Create a mapping between`key` and `value`
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let command: Command = Set {
            key: key.clone(),
            value: value.clone(),
        };
        self.write_wal_log(command)?;
        self.store.insert(key, value);
        Ok(())
    }

    fn write_wal_log(&mut self, command: Command) -> Result<()> {
        writeln!(self.file, "{}", serde_json::to_string(&command)?)?;
        Ok(())
    }
    /// Return value associated with `key`, Returns `None` when `key`
    /// not present in store
    pub fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self.store.get(key.as_str()).cloned())
    }
    /// Delete key denoted `key`
    pub fn remove(&mut self, key: String) -> Result<()> {
        if !self.store.contains_key(key.as_str()) {
            return Err(KvError::KeyNotFound);
        }
        self.write_wal_log(Remove { key: key.clone() })?;
        self.store.remove(key.as_str());
        Ok(())
    }
}
