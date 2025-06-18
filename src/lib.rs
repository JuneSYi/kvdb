use std::fs::{File, OpenOptions};
use std::{collections::HashMap, path::PathBuf};
use std::io::{BufRead, BufReader, Result, Write};
use std::path::Path;
use serde::{Serialize, Deserialize};
use serde_json;

pub struct KvStore {
    kvs: HashMap<String, String>,
    file_path: PathBuf,
}

#[derive(Serialize, Deserialize)]
enum LogCmd {
    Set { k: String, v: String },
    Remove { k: String },
}

impl KvStore {
    pub fn new(fp: &Path) -> Self {
        KvStore { 
            kvs: HashMap::new(), 
            file_path: fp.to_path_buf(),
        }
    }

    pub fn open(path: &Path) -> Result<KvStore> {
        let mut recreate_kvs = HashMap::new();
        let fp = path.join("log.json");
        if fp.exists() {
            let file = File::open(&fp)?;
            let br = BufReader::new(file);
            for line in br.lines() {
                let line = line?;
                let cmd: LogCmd = serde_json::from_str(&line)?;
                match cmd {
                    LogCmd::Set { k, v } => {
                        recreate_kvs.insert(k, v);
                    },
                    LogCmd::Remove { k } => {
                        recreate_kvs.remove(&k);
                    }
                }
            }
        }
        let kvstore = KvStore { kvs: recreate_kvs, file_path: fp };
        Ok(kvstore)
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd = LogCmd::Set { k: key.clone(), v: value.clone() };
        let serialized_cmd = serde_json::to_string(&cmd)?;
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.file_path)?;
        writeln!(file, "{serialized_cmd}")?;
        self.kvs.insert(key, value);
        Ok(())
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        let log_value = match self.kvs.get(&key).map(|s| s.to_string()) {
            Some(v) => v,
            None => {
                println!("Key not found");
                std::process::exit(0)
            }
        };
        let cmd = LogCmd::Set { k: key.clone(), v: log_value.clone() };
        let serialize_cmd = serde_json::to_string(&cmd)?;
        let file = OpenOptions::new().read(true).open(&self.file_path)?;
        let br = BufReader::new(file);
        let mut found = false;
        let removed_key = LogCmd::Remove { k: key };
        let serialized_removed_key = serde_json::to_string(&removed_key)?;
        for line in br.lines() {
            let line = line?;
            if line == serialize_cmd {
                found = true;
            } else if line == serialized_removed_key {
                found = false;
            }
        }
        match found {
            true => Ok(Some(log_value)),
            false => {
                eprintln!("Key not found");
                std::process::exit(0)
            }
        }
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
       if self.kvs.contains_key(&key) {
           let remove_cmd = LogCmd::Remove { k: key.clone() };
           let serialized_remove_cmd = serde_json::to_string(&remove_cmd)?;
           let mut file = OpenOptions::new()
               .append(true)
               .create(true)
               .open(&self.file_path)?;
           writeln!(file, "{}", serialized_remove_cmd)?;
           self.kvs.remove(&key);
           Ok(())
       } else {
            println!("Key not found");
            std::process::exit(1)
       }
   }
    // pub fn remove(&mut self, key: String) -> Result<()> {
    //     let log_value = match self.kvs.get(&key).map(|s| s.to_string()) {
    //         Some(v) => v,
    //         None => {
    //             println!("Key not found");
    //             std::process::exit(1)
    //         }
    //     };
    //     let cmd = LogCmd::Set { k: key.clone(), v: log_value.clone() };
    //     let serialize_cmd = serde_json::to_string(&cmd)?;
    //     let mut file = OpenOptions::new()
    //         .read(true)
    //         .create(true)
    //         .append(true)
    //         .open(&self.file_path)?;
    //     let br = BufReader::new(file);
    //     let mut exists = false;
    //     let removed_key = LogCmd::Remove { k: key.clone() };
    //     let serialized_removed_key = serde_json::to_string(&removed_key)?;
    //     for line in br.lines() {
    //         let line = line?;
    //         if line == serialize_cmd {
    //             exists = true;
    //         } else if line == serialized_removed_key {
    //             exists = false;
    //         }
    //     }
    //     match exists {
    //         true => {
    //             writeln!(file, "{}", serialized_removed_key)?;
    //             self.kvs.remove(&key);
    //             return Ok(());
    //         }
    //         false => {
    //             self.kvs.remove(&key);
    //             eprintln!("Key not found");
    //             std::process::exit(1)
    //         }
    //     }
    // }
}