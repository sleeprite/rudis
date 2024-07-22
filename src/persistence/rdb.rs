use std::fs::File;
use std::io::Seek;
use std::io::Write;
use std::{
    fs::OpenOptions,
    io::SeekFrom,
    sync::{Arc, Mutex},
};

use ahash::AHashMap;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;

use crate::db::{
    db::{Redis, RedisData, RedisValue},
    db_config::RedisConfig,
};

pub struct Rdb {
    pub redis_config: Arc<RedisConfig>,
    pub redis: Arc<Mutex<Redis>>,
    pub rdb_file: Option<std::fs::File>,
}

impl Rdb {
    
    pub fn new(redis_config: Arc<RedisConfig>, redis: Arc<Mutex<Redis>>) -> Rdb {
        let mut rdb_file = None;
        if let Some(filename) = &redis_config.dbfilename {
            let base_path = &redis_config.dir;
            let file_path = format!("{}{}", base_path, filename);
            rdb_file = Some(
                OpenOptions::new().create(true).truncate(true).write(true).open(file_path).expect("Failed to open AOF file"),
            );
        }

        Rdb {
            redis_config,
            redis,
            rdb_file,
        }
    }

    pub fn save(&mut self) {
        if let Some(file) = self.rdb_file.as_mut() {
            if let Err(err) = file.set_len(0) {
                eprintln!("Failed to truncate RDB file: {}", err);
                return;
            }
            if let Err(err) = file.seek(SeekFrom::Start(0)) {
                eprintln!("Failed to seek to start of RDB file: {}", err);
                return;
            }
            let redis_ref = self.redis.lock().unwrap();
            let databases: &Vec<AHashMap<String, RedisData>> = redis_ref.get_databases();
            for (db_index, database) in databases.iter().enumerate() {
                for (key, redis_data) in database.iter() {
                    let expire_at = redis_data.get_expire_at();
                    let protocol_line = match redis_data.get_value() {
                        RedisValue::List(list) => {
                            format!("{}\\r\\n{}\\r\\n{:?}\\r\\nList\\r\\n{}",db_index, key, list, expire_at)
                        }
                        RedisValue::Hash(hash) => {
                            format!("{}\\r\\n{}\\r\\n{:?}\\r\\nHash\\r\\n{}", db_index, key, hash, expire_at)
                        }
                        RedisValue::Zset(zset) => {
                            format!("{}\\r\\n{}\\r\\n{:?}\\r\\nZset\\r\\n{}", db_index, key, zset, expire_at)
                        }
                        RedisValue::String(value) => {
                            format!("{}\\r\\n{}\\r\\n{}\\r\\nString\\r\\n{}", db_index, key, value, expire_at)
                        }
                        RedisValue::Set(set) => {
                            format!("{}\\r\\n{}\\r\\n{:?}\\r\\nSet\\r\\n{}", db_index, key, set, expire_at)
                        }
                    };
                    if let Err(err) = writeln!(file, "{}", protocol_line) {
                        eprintln!("Failed to append to RDB file: {}", err);
                    }
                }
            }
        } else {
            eprintln!("RDB file is not available.");
        }
    }

    pub fn load(&mut self) {
        let mut redis_ref = self.redis.lock().unwrap();
        if let Some(filename) = &self.redis_config.dbfilename {
            let base_path = &self.redis_config.dir;
            let file_path = format!("{}{}", base_path, filename);
            if let Ok(mut file) = File::open(file_path) {
                use std::io::{BufRead, BufReader};
                let line_count = BufReader::new(&file).lines().count() as u64;
                if file.seek(SeekFrom::Start(0)).is_ok() {
                    let pb = ProgressBar::new(line_count);
                    pb.set_style(
                        ProgressStyle::default_bar()
                            .template("[{bar:39.green/cyan}] percent: {percent}% lines: {pos}/{len}")
                            .progress_chars("=>-"),
                    );
                    let reader = BufReader::new(&mut file);
                    for line in reader.lines() {
                        if let Ok(operation) = line {
                            
                            let parts: Vec<&str> = operation.split("\\r\\n").collect();
                            let data_type = parts[3].to_string();
                            let db_index = parts[0].parse::<usize>().unwrap();
                            let expire_at = parts[4].parse().unwrap();
                            let value_str = parts[2];
                            let value: Option<RedisValue> = match data_type.as_str() {
                                "List" => Some(RedisValue::List(serde_json::from_str(value_str).unwrap())),
                                "Hash" => Some(RedisValue::Hash(serde_json::from_str(value_str).unwrap())),
                                "Zset" => Some(RedisValue::Zset(serde_json::from_str(value_str).unwrap())),
                                "String" => Some(RedisValue::String(value_str.to_string())),
                                "Set" => Some(RedisValue::Set(serde_json::from_str(value_str).unwrap())),
                                _ => None
                            };

                            if let Some(redis_value) = value {
                                // Handle the RedisValue here
                                let key = parts[1].to_string();
                                redis_ref.set(db_index, key, redis_value, expire_at);
                                    
                            }
                        }
                        pb.inc(1);
                    }
                    pb.finish();
                }
            }
        }
    }
}
