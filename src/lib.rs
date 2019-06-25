#[allow(non_snake_case)]
pub struct KvStore {}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {}
    }
    pub fn set(&self, key: String, value: String) {
        unimplemented!("set")
    }
    pub fn get(&self, key: String) -> Option<String> {
        unimplemented!("get")
    }
    pub fn remove(&self, key: String) {
        unimplemented!("remove")
    }
}
