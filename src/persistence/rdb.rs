use std::fs::File;
use std::io::Seek;
use std::io::Write;
use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::SeekFrom,
    sync::{Arc, Mutex},
};

use indicatif::ProgressBar;
use indicatif::ProgressStyle;

use crate::db::{
    db::{Redis, RedisData, RedisValue},
    db_config::RedisConfig,
};

pub struct RDB {
    pub redis_config: Arc<RedisConfig>,
    pub redis: Arc<Mutex<Redis>>,
    pub rdb_file: Option<std::fs::File>,
}

impl RDB {
    pub fn new(redis_config: Arc<RedisConfig>, redis: Arc<Mutex<Redis>>) -> RDB {
        let mut rdb_file = None;

        if let Some(filename) = &redis_config.dbfilename {
            rdb_file = Some(
                OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(filename)
                    .expect("Failed to open AOF file"),
            );
        }

        RDB {
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
            let databases: &Vec<HashMap<String, RedisData>> = redis_ref.get_databases();
            for (db_index, database) in databases.iter().enumerate() {
                for (key, redis_data) in database.iter() {
                    let expire_at = redis_data.get_expire_at();
                    let protocol_line = match redis_data.get_value() {
                        RedisValue::ListValue(list) => {
                            format!("{}\\r\\n{}\\r\\n{:?}\\r\\nList\\r\\n{}",db_index, key, list, expire_at)
                        }
                        RedisValue::HashValue(hash) => {
                            format!("{}\\r\\n{}\\r\\n{:?}\\r\\nHash\\r\\n{}", db_index, key, hash, expire_at)
                        }
                        RedisValue::ZsetValue(zset) => {
                            format!("{}\\r\\n{}\\r\\n{:?}\\r\\nZset\\r\\n{}", db_index, key, zset, expire_at)
                        }
                        RedisValue::StringValue(value) => {
                            format!("{}\\r\\n{}\\r\\n{}\\r\\nString\\r\\n{}", db_index, key, value, expire_at)
                        }
                        RedisValue::SetValue(set) => {
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
            if let Ok(mut file) = File::open(filename) {
                use std::io::{BufRead, BufReader};
                let line_count = BufReader::new(&file).lines().count() as u64;

                if let Ok(_) = file.seek(SeekFrom::Start(0)) {
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
                                "List" => Some(RedisValue::ListValue(serde_json::from_str(value_str).unwrap())),
                                "Hash" => Some(RedisValue::HashValue(serde_json::from_str(value_str).unwrap())),
                                "Zset" => Some(RedisValue::ZsetValue(serde_json::from_str(value_str).unwrap())),
                                "String" => Some(RedisValue::StringValue(value_str.to_string())),
                                "Set" => Some(RedisValue::SetValue(serde_json::from_str(value_str).unwrap())),
                                _ => None
                            };

                            match value {
                                Some(redis_value) => {
                                    // Handle the RedisValue here
                                    let key = parts[1].to_string();
                                    redis_ref.set(db_index, key, redis_value, expire_at);
                                    
                                },
                                None => {
                                    // Handle the case where value is None
                                    // Example: println!("No RedisValue found.");
                                }
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
