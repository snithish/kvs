#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[allow(non_snake_case)]
pub struct KvStore {}

impl KvStore {
    pub fn new() -> KvStore { unimplemented!() }
    pub fn set(&self, key: String, value: String) { unimplemented!() }
    pub fn get(&self, key: String) -> Option<String> { unimplemented!() }
    pub fn remove(&self, key: String) { unimplemented!() }
}