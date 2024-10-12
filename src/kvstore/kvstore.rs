#[derive(Debug)]
pub struct KvStore {
    key: String,
    value: String,
}

impl KvStore {
    pub fn new(key: String, value: String) -> Self {
        KvStore { key, value }
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
}