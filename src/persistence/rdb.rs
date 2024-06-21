use std::{fs::OpenOptions, sync::{Arc, Mutex}};
use std::io::Write;

use crate::db::{db::Redis, db_config::RedisConfig};

pub struct RDB {
    pub redis_config: Arc<RedisConfig>,
    pub redis: Arc<Mutex<Redis>>,
    pub rdb_file: Option<std::fs::File>,
}

impl RDB {
    
    pub fn new(redis_config: Arc<RedisConfig>, redis: Arc<Mutex<Redis>>) -> RDB {
        let mut rdb_file = None;
        if let Some(filename ) = &redis_config.dbfilename {
            rdb_file = Some(OpenOptions::new().create(true).read(true).write(true).append(true).open(filename).expect("Failed to open AOF file"));
        }

        RDB {
            redis_config,
            redis,
            rdb_file,
        }
    }

    pub fn save(&mut self) {


        if let Some(file) = self.rdb_file.as_mut() {
        
            // 查询所有键值，并将 dbIndex key value valueType 组合，存储到 dump.rdb 文件
            let mut redis_ref = self.redis.lock().unwrap();
        
            if let Err(err) = writeln!(file, "{}", "") {
                eprintln!("Failed to append to AOF file: {}", err);
            }
        }
    }
}