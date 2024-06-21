use std::{collections::HashMap, fs::OpenOptions, io::SeekFrom, sync::{Arc, Mutex}};
use std::io::Write;
use std::io::Seek;

use crate::db::{db::{Redis, RedisData, RedisValue}, db_config::RedisConfig};

pub struct RDB {
    pub redis_config: Arc<RedisConfig>,
    pub redis: Arc<Mutex<Redis>>,
    pub rdb_file: Option<std::fs::File>,
}

impl RDB {
    
    pub fn new(redis_config: Arc<RedisConfig>, redis: Arc<Mutex<Redis>>) -> RDB {
        let mut rdb_file = None;
      
        if let Some(filename ) = &redis_config.dbfilename {
            rdb_file = Some(OpenOptions::new().create(true).write(true).open(filename).expect("Failed to open AOF file"));
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
                            format!("{} {} {:?} List {}",  db_index, key, list, expire_at)
                        },
                        RedisValue::HashValue(hash) => {
                            format!("{} {} {:?} Hash {}",  db_index, key, hash, expire_at)
                        },
                        RedisValue::ZsetValue(zset) => {
                            format!("{} {} {:?} Zset {}",  db_index, key, zset, expire_at)
                        },
                        RedisValue::StringValue(value) => {                            
                            format!("{} {} {:?} String {}",  db_index, key, value, expire_at)
                        },
                        RedisValue::SetValue(set) => {
                            format!("{} {} {:?} Set {}",  db_index, key, set, expire_at)
                        },
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
}