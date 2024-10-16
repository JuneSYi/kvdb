use std::collections::HashMap;
// use std::error::Error;
use anyhow::{Result, Context};
use crate::error;

#[derive(Debug)]
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.map.insert(key, value);

        Ok(())
    }

    pub fn get(&self, key: String) -> Option<String> {
        // match self.map.get(&key) {
        //     Some(res) => Some(res.to_string()),
        //     None => None
        // }
        self.map.get(&key).map(|res| res.to_string())
    }

    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}
