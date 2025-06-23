use std::fs::{File, OpenOptions};
use std::{collections::HashMap, path::PathBuf};
use std::io::{BufReader, Error, ErrorKind, Result, SeekFrom};
use std::io::prelude::*;
use std::path::Path;
use serde::{Serialize, Deserialize};
use serde_json;

pub struct KvStore {
    kvs: HashMap<String, LogPointer>,
    file_path: PathBuf,
    stale_log_ct: u8,
}

#[derive(Serialize, Deserialize)]
enum LogCmd {
    Set { k: String, v: String },
    Remove { k: String },
}

struct LogPointer {
    offset: u64,
    len: u64,
}

impl KvStore {
    pub fn new(fp: &Path) -> Self {
        KvStore { 
            kvs: HashMap::new(),
            file_path: fp.to_path_buf(),
            stale_log_ct: 0,
        }
    }

    pub fn open(path: &Path) -> Result<KvStore> {
        let mut recreate_kvs = HashMap::<String, LogPointer>::new();
        let fp = path.join("log.json");
        let mut stale_log_count = 0;
        if fp.exists() {
            let file = File::open(&fp)?;
            let mut br = BufReader::new(file);
            let mut ptr_position: u64 = 0;
            let mut buffer = String::new();
            while br.read_line(&mut buffer)? > 0 {
                let line_size = buffer.len();
                let line_content = buffer.trim_end();
                let cmd: LogCmd = serde_json::from_str(line_content)?;
                match cmd {
                    LogCmd::Set { k, v: _ } => {
                        let log_pointer = LogPointer { offset: ptr_position, len: line_size as u64 };
                        if recreate_kvs.contains_key(&k) {
                            stale_log_count += 1;
                        }
                        recreate_kvs.insert(k, log_pointer);
                    },
                    LogCmd::Remove { k } => {
                        stale_log_count += 1;
                        recreate_kvs.remove(&k);
                    }
                }
                ptr_position += line_size as u64;
                buffer.clear();
            }
        }
        let kvstore = KvStore { kvs: recreate_kvs, file_path: fp, stale_log_ct: stale_log_count };
        Ok(kvstore)
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd = LogCmd::Set { k: key.clone(), v: value.clone() };
        let serialized_cmd = serde_json::to_string(&cmd)?;
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.file_path)?;
        file.seek(SeekFrom::End(0))?;
        let before_position = file.stream_position()?;
        writeln!(file, "{serialized_cmd}")?;
        let after_position = file.stream_position()?;
        let log_pointer = LogPointer { offset: before_position, len: after_position - before_position };
        if self.kvs.contains_key(&key) {
            self.stale_log_ct += 1;
        }
        self.kvs.insert(key, log_pointer);
        if self.stale_log_ct > 9 {
            self.compact_log()?;
        }
        Ok(())
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        if let Some(log_pointer) = self.kvs.get(&key) {
            let mut file = OpenOptions::new().read(true).open(&self.file_path)?;
            file.seek(SeekFrom::Start(log_pointer.offset))?;
            let mut buffer = vec![0; log_pointer.len as usize];
            file.read_exact(&mut buffer[..])?;
            let deserialized_cmd: LogCmd = serde_json::from_slice(&buffer)?;
            match deserialized_cmd {
                LogCmd::Set { k: _, v } => {
                    return Ok(Some(v));
                },
                LogCmd::Remove { k: _ } => return Ok(None)
            };
        } else {
            Ok(None)
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
            self.stale_log_ct += 1;
            if self.stale_log_ct > 9 {
                self.compact_log()?;
            }
            Ok(())
       } else {
            Err(Error::new(ErrorKind::NotFound , "Key not found"))
       }
   }

    fn compact_log(&mut self) -> Result<()> {
        let temp_log_path = self.file_path.with_extension("json.compact");
        let mut temp_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&temp_log_path)?;
        let mut current_log_file = File::open(&self.file_path)?;
        let mut new_kvs = HashMap::new();
        let mut current_pos = 0;

        for (key, log_pointer) in &self.kvs {
            current_log_file.seek(SeekFrom::Start(log_pointer.offset))?;
            let mut entry_reader = current_log_file.try_clone()?.take(log_pointer.len);
            let len_written = std::io::copy(&mut entry_reader, &mut temp_file)?;

            new_kvs.insert(
                key.clone(),
                LogPointer {
                    offset: current_pos,
                    len: len_written,
                },
            );
            current_pos += len_written;
        }
        std::fs::rename(&temp_log_path, &self.file_path)?;
        self.kvs = new_kvs;
        self.stale_log_ct = 0;
        Ok(())
    }
}