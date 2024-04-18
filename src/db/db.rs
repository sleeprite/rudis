use indicatif::{ProgressBar, ProgressStyle};
use std::io::Seek;
use std::{ collections::HashMap, fs::{File, OpenOptions}, io::{SeekFrom, Write}, sync::Arc};

use crate::tools::date::current_millis;

use super::db_config::RedisConfig;

pub enum RedisValue {
    StringValue(String),
    StringArrayValue(Vec<String>),
}

pub struct RedisData {
    value: RedisValue,
    expire_at: i64,
}

impl RedisData {
    pub fn new(value: RedisValue, expire_at: i64) -> Self {
        Self { value, expire_at }
    }

    pub fn is_expired(&self) -> bool {
        self.expire_at != -1 && self.expire_at <= current_millis()
    }

    pub fn get_expire_at(&self) -> i64 {
        return self.expire_at;
    }

    pub fn set_expire_at(&mut self, expire_at: i64) {
        self.expire_at = expire_at;
    }
}

pub struct Redis {
    pub databases: Vec<HashMap<String, RedisData>>,
    pub appendfile: Option<std::fs::File>,
    pub redis_config: Arc<RedisConfig>,
}

impl Redis {
    /*
     * Redis 构造函数
     *
     * @param redis_config 配置文件
     */
    pub fn new(redis_config: Arc<RedisConfig>) -> Redis {
        let mut databases = Vec::new();
        for _ in 0..redis_config.databases {
            databases.push(HashMap::new());
        }

        /*
         * 判定 appendonly 是否开启，appendfilename 是否不为 None
         *
         * 成立则创建 File 实例，否则不创建
         */
        let mut appendfile: Option<File> = None;

        if redis_config.appendonly && redis_config.appendfilename.is_some() {
            if let Some(filename) = &redis_config.appendfilename {
                appendfile = Some(
                    OpenOptions::new().create(true).read(true).write(true).append(true).open(filename).expect("Failed to open AOF file"),
                )
            }
        }

        Redis {
            databases,
            appendfile,
            redis_config,
        }
    }

    /*
     * 获取数据库大小
     *
     * @param db_index 数据库索引
     * @return 数据库大小
     */
    pub fn size(&self, db_index: usize) -> usize {
        if db_index < self.databases.len() {
            self.databases[db_index].len()
        } else {
            panic!("Invalid database index");
        }
    }

    /*
     * 设置值
     *
     * @param db_index 数据库索引
     * @param key 数据键
     */
    pub fn set(&mut self, db_index: usize, key: String, value: String, is_aof_recovery: bool) {
        if db_index < self.databases.len() {
            self.databases[db_index].insert(
                key.clone(),
                RedisData::new(RedisValue::StringValue(value.clone()), -1),
            );
            if !is_aof_recovery {
                self.append_aof(&format!("{} SET {} {}", db_index, key, value));
            }
        } else {
            panic!("Invalid database index");
        }
    }

    /*
     * 获取值
     *
     * @param db_index 数据库索引
     * @param key 数据键
     */
    pub fn get(&self, db_index: usize, key: &String) -> Result<Option<&String>, &str> {
        if db_index < self.databases.len() {
            match self.databases[db_index].get(key) {
                Some(redis_value) => match &redis_value.value {
                    RedisValue::StringValue(s) => Ok(Some(s)),
                    _ => Err("ERR Operation against a key holding the wrong kind of value"),
                },
                None => Ok(None),
            }
        } else {
            Err("Invalid database index")
        }
    }

    /*
     * 删除 Key
     *
     * @param db_index 数据库索引
     * @param key 数据键
     * @return 如果删除成功返回 true，如果不存在返回 false
     */
    pub fn del(&mut self, db_index: usize, key: &String, is_aof_recovery: bool) -> bool {
        if let Some(db) = self.databases.get_mut(db_index) {
            if db.remove(key).is_some() {
                if !is_aof_recovery {
                    self.append_aof(&format!("{} DEL {}", db_index, key));
                }
                return true;
            }
        }
        false
    }

    /*
     * 检测 Key 是否存在
     *
     * @param db_index 数据库索引
     * @param key 数据键
     */
    pub fn exists(&self, db_index: usize, key: &String) -> bool {
        if db_index < self.databases.len() {
            self.databases[db_index].contains_key(key)
        } else {
            panic!("Invalid database index");
        }
    }

    /*
     * 设置值的同时，设置过期时间
     *
     * @param db_index 数据库索引
     * @param key 数据键
     * @param value 数据值
     * @param ttl 过期时间，单位：毫秒
     */
    pub fn set_with_ttl(&mut self, db_index: usize, key: String, value: String, ttl: i64, is_aof_recovery: bool) {
        if db_index < self.databases.len() {
            let redis_value = RedisData::new(RedisValue::StringValue(value.clone()), ttl);
            let expire_at = redis_value.get_expire_at();
            self.databases[db_index].insert(key.clone(), redis_value);
            if !is_aof_recovery {
                self.append_aof(&format!("{} SET {} {} {}", db_index, key, value, expire_at));
            }
        } else {
            panic!("Invalid database index");
        }
    }

    /*
     * 检查过期
     *
     * @param db_index 数据库索引
     * @param key 数据键
     */
    pub fn check_ttl(&mut self, db_index: usize, key: &String) {
        if db_index < self.databases.len() {
            match self.databases[db_index].get(key) {
                Some(redis_value) => {
                    if redis_value.is_expired() {
                        self.databases[db_index].remove(key);
                    }
                }
                None => {
                    // Handle the case when redis_value is None
                }
            }
        } else {
            panic!("Invalid database index");
        }
    }

    /**
     * 检查过期【所有键】
     *
     * @param db_index 数据库索引
     */
    pub fn check_all_ttl(&mut self, db_index: usize) {
        if db_index < self.databases.len() {
            let mut expired_keys = Vec::new();

            for (key, value) in &self.databases[db_index] {
                if value.is_expired() {
                    expired_keys.push(key.clone());
                }
            }

            for key in expired_keys {
                self.databases[db_index].remove(&key);
            }
        } else {
            panic!("Invalid database index");
        }
    }

    /*
     * expire 方法用于设置键的过期时间
     *
     * @param db_index 数据库索引
     * @param key 主键
     * @param ttl_millis 过期时间，单位: 毫秒
     */
    pub fn expire(&mut self, db_index: usize, key: String, ttl_millis: i64, is_aof_recovery: bool) -> bool {

        if db_index >= self.databases.len() {
            panic!("Invalid database index");
        }

        if let Some(redis_value) = self.databases[db_index].get_mut(&key) {
            redis_value.set_expire_at(ttl_millis);
            if !is_aof_recovery {
                let expire_at = redis_value.get_expire_at();
                self.append_aof(&format!("{} EXPIRE {} {}", db_index, key, expire_at));
            }
            return true;
        }

        false
    }

    /*
     * 清空数据库
     *
     * @param db_index 数据库索引
     */
    pub fn flush_db(&mut self, db_index: usize, is_aof_recovery: bool) {
        if db_index < self.databases.len() {
            self.databases[db_index].clear();
            if !is_aof_recovery {
                self.append_aof(&format!("{} FLUSHDB", db_index));
            }
        } 
    }

    /*
     * 清空所有数据库
     */
    pub fn flush_all(&mut self, is_aof_recovery: bool) {
        for db in &mut self.databases {
            db.clear();
        }
        if !is_aof_recovery {
            self.append_aof(&format!("-1 FLUSHALL"));
        }
    }

    /*
     * 重命名主键
     *
     * @param old_key 旧主键名称
     * @param new_key 新主键名称
     */
    pub fn rename(&mut self, db_index: usize, old_key: &str, new_key: &str, is_aof_recovery: bool) -> bool {
        let db = self.databases.get_mut(db_index).unwrap();
        // 检查是否存在旧键
        if let Some(value) = db.remove(old_key) {
            // 将值从旧键移到新键
            db.insert(new_key.to_string(), value);
            if !is_aof_recovery {
                self.append_aof(&format!("{} RENAME {} {}", db_index, old_key, new_key));
            }
            return true;
        }
        false
    }

    /*
     * 移动主键
     *
     * @param key 主键
     * @param src_db_index 源数据库
     * @param target_db_index 目标数据库
     */
    pub fn move_key(
        &mut self,
        src_db_index: usize,
        key: &str,
        target_db_index: usize,
    ) -> Result<(), ()> {
        let src_db = match self.databases.get_mut(src_db_index) {
            Some(db) => db,
            None => return Err(()), // 如果源数据库不存在，则返回错误
        };

        if let Some(value) = src_db.remove(key) {
            // 如果源数据库中存在键，则将其移动到目标数据库中
            let dest_db = self.databases.get_mut(target_db_index).unwrap();
            dest_db.insert(key.to_string(), value);
            Ok(())
        } else {
            Err(()) // 如果源数据库中不存在键，则返回错误
        }
    }

    /*
     * 将一个或多个值插入到列表的头部
     *
     * @param db_index  DB 索引
     * @param key 列表键
     * @param values 要插入的值
     */
    pub fn lpush(&mut self, db_index: usize, key: String, values: Vec<String>) {
        if db_index < self.databases.len() {
            let list = self.databases[db_index]
                .entry(key)
                .or_insert(RedisData::new(RedisValue::StringArrayValue(vec![]), -1));

            if let RedisValue::StringArrayValue(ref mut current_values) = list.value {
                current_values.splice(0..0, values);
            }
        } else {
            panic!("Invalid database index");
        }
    }

    /*
     * 将一个或多个值插入到列表的尾部
     *
     * @param db_index  DB 索引
     * @param key 列表键
     * @param values 要插入的值
     */
    pub fn rpush(&mut self, db_index: usize, key: String, values: Vec<String>) {
        if db_index < self.databases.len() {
            let list = self.databases[db_index]
                .entry(key)
                .or_insert(RedisData::new(RedisValue::StringArrayValue(vec![]), -1));

            if let RedisValue::StringArrayValue(ref mut current_values) = list.value {
                current_values.extend(values);
            }
        } else {
            panic!("Invalid database index");
        }
    }

    /*
     * 获取列表长度
     *
     * @param db_index DB 索引
     * @param key 列表键
     * @return 列表长度，如果键不存在或者不是列表则返回 0
     */
    pub fn llen(&self, db_index: usize, key: &String) -> usize {
        if db_index < self.databases.len() {
            if let Some(redis_value) = self.databases[db_index].get(key) {
                if let RedisValue::StringArrayValue(ref array) = redis_value.value {
                    return array.len();
                }
            }
        } else {
            panic!("Invalid database index");
        }
        0 // Return 0 if the key doesn't exist or is not a list
    }

    /*
     * 将 appendfile 文件 load 内容到数据库
     *
     * 调用时机：项目启动
     */
    pub fn load_aof(&mut self) {
        if self.redis_config.appendonly {
            if let Some(appendfilename) = &self.redis_config.appendfilename {
                if let Ok(mut file) = File::open(appendfilename) {

                    use std::io::{BufRead, BufReader};
                    let line_count = BufReader::new(&file).lines().count() as u64;
                    if let Ok(_) = file.seek(SeekFrom::Start(0)) {

                        // 创建 ProgressBar 进度条 {pos} {len} {percent}
                        let pb = ProgressBar::new(line_count);
                        pb.set_style(ProgressStyle::default_bar().template("[{bar:41}] percent: {percent}% lines: {pos}/{len}").progress_chars("=>-"));

                        let reader = BufReader::new(&mut file);
                        for line in reader.lines() {
                            if let Ok(operation) = line {
                                let parts: Vec<&str> = operation.trim().split_whitespace().collect();
                                let db_index = parts[0].to_string();
                                let db_index_usize = db_index.parse::<usize>().unwrap();

                                if !parts.is_empty() {
                                    match parts[1] {
                                        "SET" => {
                                            let key = parts[2].to_string();
                                            let val = parts[3].to_string();
                                            let expire_at = parts.get(4).and_then(|v| v.parse().ok()).unwrap_or(-1);
                                            if expire_at == -1 { 
                                                self.set(db_index_usize, key, val, true);
                                            } else {
                                                if expire_at > current_millis() {
                                                    self.set_with_ttl(db_index_usize, key, val, expire_at, true);
                                                }
                                            }
                                        }
                                        "DEL" => {
                                            let key = parts[2].to_string();
                                            self.del(db_index_usize, &key, true);
                                        }
                                        "FLUSHALL" => {
                                            self.flush_all(true)
                                        }
                                        "FLUSHDB" => {
                                            self.flush_db(db_index_usize, true)
                                        }
                                        "EXPIRE" => {
                                            let key = parts[2].to_string();
                                            let expire_at = parts.get(3).and_then(|v| v.parse().ok()).unwrap_or(-1);
                                            self.expire(db_index_usize, key, expire_at, true);
                                        }
                                        "RENAME" => {
                                            let old_key = parts[2].to_string();
                                            let new_key = parts[3].to_string();
                                            self.rename(db_index_usize, &old_key, &new_key, true);
                                        }
                                        _ => {
                                            // Handle other operations if needed
                                        }
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

    /*
     * 将需要持久化的命令，存储到 aof 文件
     *
     * @param command 操作字符串
     */
    fn append_aof(&mut self, command: &str) {
        if let Some(file) = self.appendfile.as_mut() {
            if let Err(err) = writeln!(file, "{}", command) {
                eprintln!("Failed to append to AOF file: {}", err);
            }
        }
    }
}
