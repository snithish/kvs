use std::collections::HashMap;

#[allow(non_snake_case)]
#[derive(Default)]
pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> KvStore {
        Default::default()
    }
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }
    pub fn get(&self, key: String) -> Option<String> {
        self.store.get(key.as_str()).map(|x| x.to_owned())
    }
    pub fn remove(&mut self, key: String) {
        self.store.remove(key.as_str());
    }
}
