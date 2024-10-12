#[derive(Debug)]
pub struct KvStore {
    key: String,
    value: String,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore { key: "".to_string(), value: "".to_string() }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.key = key;
        self.value = value;
    }

    pub fn get(&self, key: String) -> Option<String> {
        match key == self.key {
            true => Some(self.value.clone()),
            false => None
        }
    }

    pub fn remove(&mut self, key: String) {
        // self.value.clone()
    }


}