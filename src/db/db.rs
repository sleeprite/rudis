use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashSet;
use std::io::Seek;
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{SeekFrom, Write},
    sync::Arc,
};

use crate::tools::date::current_millis;

use super::db_config::RedisConfig;

pub enum RedisValue {
    StringValue(String),
    ListValue(Vec<String>),
    SetValue(HashSet<String>),
    HashValue(HashMap<String, String>)
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
     * 获取剩余过期时间（秒）
     *
     * @param db_index 数据库索引
     * @param key 数据键
     */
    pub fn ttl(&self, db_index: usize, key: String) -> i64 {
        if db_index < self.databases.len() {
            if let Some(redis_value) = self.databases[db_index].get(&key) {
                if redis_value.get_expire_at() == -1 {
                    return -1;
                } else {
                    return (redis_value.get_expire_at() - current_millis()) / 1000;
                }
            }
        }

        -2 // Key不存在或无过期时间返回-2
    }

    /*
     * 获取剩余过期时间（毫秒）
     *
     * @param db_index 数据库索引
     * @param key 数据键
     */
    pub fn pttl(&self, db_index: usize, key: String) -> i64 {
        if db_index < self.databases.len() {
            if let Some(redis_value) = self.databases[db_index].get(&key) {
                if redis_value.get_expire_at() == -1 {
                    return -1;
                } else {
                    return redis_value.get_expire_at() - current_millis();
                }
            }
        }

        -2 // Key不存在或无过期时间返回-2
    }

    pub fn key_type(&self, db_index: usize, key: String) -> String {
        if db_index < self.databases.len() {
            match self.databases[db_index].get(&key) {
                Some(redis_value) => match &redis_value.value {
                    RedisValue::ListValue(_) => "list".to_string(),
                    RedisValue::StringValue(_) => "string".to_string(),
                    RedisValue::SetValue(_) => "set".to_string(),
                    RedisValue::HashValue(_) => "hash".to_string(),
                },
                None => "none".to_string(),
            }
        } else {
            "Invalid database index".to_string()
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
    pub fn set_with_ttl(
        &mut self,
        db_index: usize,
        key: String,
        value: String,
        ttl: i64,
        is_aof_recovery: bool,
    ) {
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
    pub fn expire(
        &mut self,
        db_index: usize,
        key: String,
        ttl_millis: i64,
        is_aof_recovery: bool,
    ) -> bool {
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
    pub fn rename(
        &mut self,
        db_index: usize,
        old_key: &str,
        new_key: &str,
        is_aof_recovery: bool,
    ) -> Result<bool, &str> {
        let db = match self.databases.get_mut(db_index) {
            Some(db) => db,
            None => return Err("Database index out of bounds"),
        };

        if let Some(value) = db.remove(old_key) {
            db.insert(new_key.to_string(), value);
            if !is_aof_recovery {
                self.append_aof(&format!("{} RENAME {} {}", db_index, old_key, new_key));
            }
            return Ok(true);
        }

        Err("ERR no such key")
    }

    /*
     * 移动主键
     *
     * @param key 主键
     * @param src_db_index 源数据库
     * @param target_db_index 目标数据库
     * @param is_aof_recovery 是否记录 aof 日志
     */
    pub fn move_key(
        &mut self,
        src_db_index: usize,
        key: &str,
        target_db_index: usize,
        is_aof_recovery: bool,
    ) -> bool {
        if let Some(src_db) = self.databases.get_mut(src_db_index) {
            if let Some(value) = src_db.remove(key) {
                if let Some(dest_db) = self.databases.get_mut(target_db_index) {
                    if !dest_db.contains_key(key) {
                        dest_db.insert(key.to_string(), value);
                        if !is_aof_recovery {
                            self.append_aof(&format!(
                                "{} MOVE {} {}",
                                src_db_index, key, target_db_index
                            ));
                        }
                        return true;
                    }
                }
            }
        }
        false
    }

    /*
     * 将一个或多个值插入到列表的头部
     *
     * @param db_index  DB 索引
     * @param key 列表键
     * @param values 要插入的值
     * @param is_aof_recovery 是否记录 aof 日志
     */
    pub fn lpush(
        &mut self,
        db_index: usize,
        key: String,
        values: Vec<String>,
        is_aof_recovery: bool,
    ) {
        if db_index < self.databases.len() {
            let list = self.databases[db_index]
                .entry(key.clone())
                .or_insert(RedisData::new(RedisValue::ListValue(vec![]), -1));
            if let RedisValue::ListValue(ref mut current_values) = list.value {
                current_values.splice(0..0, values.clone());
                if !is_aof_recovery {
                    let values_str = values.join(" ");
                    self.append_aof(&format!("{} LPUSH {} {}", db_index, key, values_str));
                }
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
     * @param is_aof_recovery 是否记录 aof 日志
     */
    pub fn rpush(
        &mut self,
        db_index: usize,
        key: String,
        values: Vec<String>,
        is_aof_recovery: bool,
    ) {
        if db_index < self.databases.len() {
            let list = self.databases[db_index]
                .entry(key.clone())
                .or_insert(RedisData::new(RedisValue::ListValue(vec![]), -1));
            if let RedisValue::ListValue(ref mut current_values) = list.value {
                current_values.extend(values.clone());
                if !is_aof_recovery {
                    let values_str = values.join(" ");
                    self.append_aof(&format!("{} RPUSH {} {}", db_index, key, values_str));
                }
            }
        } else {
            panic!("Invalid database index");
        }
    }

    /*
     * 移除并返回列表的第一个元素
     * 
     * @param db_index 数据库索引
     * @param key 列表键
     * @param is_aof_recovery 是否为数据恢复
     */
    pub fn lpop(&mut self, db_index: usize, key: String, is_aof_recovery: bool) -> Option<String> {
        if db_index < self.databases.len() {
            match self.databases[db_index].get_mut(&key) {
                Some(list) => {
                    if let RedisValue::ListValue(ref mut current_values) = list.value {
                        if !current_values.is_empty() {
                            let popped_value = current_values.remove(0);

                            // Check if the list is empty after removal
                            if current_values.is_empty() {
                                self.databases[db_index].remove(&key);
                            }

                            if !is_aof_recovery {
                                self.append_aof(&format!("{} LPOP {}", db_index, key));
                            }

                            return Some(popped_value);
                        }
                    }
                }
                None => return None, // Key does not exist
            }
        } else {
            panic!("Invalid database index");
        }

        None
    }

    /*
     * 移除并返回列表的最后一个元素。
     * 
     * @param db_index 数据库索引
     * @param key 列表键
     * @param is_aof_recovery 是否为数据恢复
     */
    pub fn rpop(&mut self, db_index: usize, key: String, is_aof_recovery: bool) -> Option<String> {
        if db_index < self.databases.len() {
            match self.databases[db_index].get_mut(&key) {
                Some(list) => {
                    if let RedisValue::ListValue(ref mut current_values) = list.value {
                        if !current_values.is_empty() {
                            let popped_value = current_values.pop();

                            // Check if the list is empty after removal
                            if current_values.is_empty() {
                                self.databases[db_index].remove(&key);
                            }

                            if !is_aof_recovery {
                                self.append_aof(&format!("{} RPOP {}", db_index, key));
                            }

                            return popped_value;
                        }
                    }
                }
                None => return None, // Key does not exist
            }
        } else {
            panic!("Invalid database index");
        }

        None
    }

    /*
     * 返回列表中指定区间内的元素
     *
     * @param db_index DB 索引
     * @param key 列表键
     * @param start 开始索引
     * @param end 结束索引
     */
    pub fn lrange(&mut self, db_index: usize, key: String, start: i64, end: i64) -> Vec<String> {
        if db_index < self.databases.len() {
            match self.databases[db_index].get(&key) {
                Some(list) => {
                    if let RedisValue::ListValue(ref current_values) = list.value {
                        let list_length = current_values.len() as i64;
                        let mut adjusted_start = if start < 0 {
                            list_length + start
                        } else {
                            start
                        };
                        let mut adjusted_end = if end < 0 { list_length + end } else { end };

                        // Adjust start/end to be within list bounds
                        adjusted_start = adjusted_start.max(0).min(list_length - 1);
                        adjusted_end = adjusted_end.max(0).min(list_length - 1);

                        if adjusted_start > adjusted_end {
                            return Vec::new(); // Empty range
                        }

                        return current_values[adjusted_start as usize..=adjusted_end as usize]
                            .to_vec();
                    }
                }
                None => return Vec::new(), // Key does not exist
            }
        } else {
            panic!("Invalid database index");
        }

        Vec::new()
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
                if let RedisValue::ListValue(ref array) = redis_value.value {
                    return array.len();
                }
            }
        } else {
            panic!("Invalid database index");
        }
        0 // Return 0 if the key doesn't exist or is not a list
    }

    /*
     * 通过索引获取列表中的元素
     *
     * @param db_index DB 索引
     * @param key 列表键
     * @param index 值索引
     */
    pub fn lindex(&self, db_index: usize, key: &String, index: i64) -> Option<String> {
        if db_index < self.databases.len() {
            if let Some(redis_value) = self.databases[db_index].get(key) {
                if let RedisValue::ListValue(ref array) = redis_value.value {
                    let index = if index < 0 {
                        (array.len() as i64 + index) as usize
                    } else {
                        index as usize
                    };

                    if index < array.len() {
                        return Some(array[index].clone());
                    }
                }
            }
        } else {
            panic!("Invalid database index");
        }
        None // Return None if the key doesn't exist, is not a list, or the index is out of bounds
    }

    /*
     * 将一个或多个成员添加到集合中
     *
     * @param db_index  DB 索引
     * @param key 集合键
     * @param members 要添加的成员
     */
    pub fn sadd(
        &mut self,
        db_index: usize,
        key: String,
        members: Vec<String>,
        is_aof_recovery: bool,
    ) -> Result<i64, String> {
        if db_index < self.databases.len() {
            let set = self.databases[db_index]
                .entry(key.clone())
                .or_insert(RedisData::new(RedisValue::SetValue(HashSet::new()), -1));
            if let RedisValue::SetValue(ref mut current_members) = set.value {
                let mut count = 0;
                for member in &members {
                    if current_members.insert(member.clone()) {
                        count += 1;
                    }
                }
                if !is_aof_recovery {
                    self.append_aof(&format!("{} SADD {} {}", db_index, key, members.join(" ")));
                }
                Ok(count)
            } else {
                Err("Key exists and is not a set".to_string())
            }
        } else {
            Err("Invalid database index".to_string())
        }
    }

    /*
     * 返回集合中的所有的成员。 不存在的集合 key 被视为空集合。
     * 
     * @param db_index 数据库索引
     * @param key 列表键
     */
    pub fn smembers(&self, db_index: usize, key: &String) -> Option<&HashSet<String>> {
        if let Some(set) = self.databases.get(db_index)?.get(key) {
            if let RedisValue::SetValue(members) = &set.value {
                return Some(members);
            }
        }
        None
    }
    
    /*
     * 返回集合中元素的数量。
     * 
     * @param db_index 数据库索引
     * @param key 列表键
     */
    pub fn scard(&self, db_index: usize, key: &String) -> Option<usize> {
        if let Some(set) = self.databases.get(db_index)?.get(key) {
            if let RedisValue::SetValue(members) = &set.value {
                return Some(members.len());
            }
        }
        None
    }

    /*
     * 获取列表长度
     *
     * @param db_index DB 索引
     * @param key 列表键
     * @return 列表长度，如果键不存在或者不是列表则返回 0
     */
    pub fn append(
        &mut self,
        db_index: usize,
        key: String,
        value: String,
        is_aof_recovery: bool,
    ) -> Result<usize, String> {
        let db = match self.databases.get_mut(db_index) {
            Some(db) => db,
            None => return Err("ERR invalid database index".to_string()),
        };

        let len = match db.get_mut(&key) {
            Some(redis_data) => {
                if let RedisValue::StringValue(s) = &mut redis_data.value {
                    s.push_str(&value);
                    s.len()
                } else {
                    return Err(
                        "ERR Operation against a key holding the wrong kind of value".to_string(),
                    );
                }
            }
            None => {
                let redis_data = RedisData::new(RedisValue::StringValue(value.clone()), -1);
                db.insert(key.clone(), redis_data);

                value.len()
            }
        };

        if !is_aof_recovery {
            self.append_aof(&format!("{} APPEND {} {}", db_index, key, value));
        }

        Ok(len)
    }

    /*
     * 将 key 中储存的数字值增一。
     * 
     * @param db_index 数据库索引
     * @param key 列表键
     * @param increment 步长
     */
    pub fn incr(
        &mut self,
        db_index: usize,
        key: String,
        increment: i64,
        is_aof_recovery: bool,
    ) -> Result<i64, String> {
        let database = match self.databases.get_mut(db_index) {
            Some(db) => db,
            None => return Err("ERR invalid DB index".to_string()),
        };

        let redis_data = database
            .entry(key.clone())
            .or_insert_with(|| RedisData::new(RedisValue::StringValue("0".to_string()), -1));

        let result = match &mut redis_data.value {
            RedisValue::StringValue(val) => match val.parse::<i64>() {
                Ok(current_val) => current_val + increment,
                Err(_) => return Err("ERR value is not an integer".to_string()),
            },
            _ => {
                return Err(
                    "ERR Operation against a key holding the wrong kind of value".to_string(),
                )
            }
        };

        if let RedisValue::StringValue(val) = &mut redis_data.value {
            *val = result.to_string();
        }

        if !is_aof_recovery {
            self.append_aof(&format!("{} INCR {} {}", db_index, key, increment));
        }

        Ok(result)
    }

    /*
     * 将 key 中储存的数字值减一。
     * 
     * @param db_index 数据库索引
     * @param key 列表键
     * @param increment 步长
     */
    pub fn decr(
        &mut self,
        db_index: usize,
        key: String,
        increment: i64,
        is_aof_recovery: bool,
    ) -> Result<i64, String> {
        let database = match self.databases.get_mut(db_index) {
            Some(db) => db,
            None => return Err("ERR invalid DB index".to_string()),
        };

        let redis_data = database
            .entry(key.clone())
            .or_insert_with(|| RedisData::new(RedisValue::StringValue("0".to_string()), -1));

        let result = match &mut redis_data.value {
            RedisValue::StringValue(val) => match val.parse::<i64>() {
                Ok(current_val) => current_val - increment,
                Err(_) => return Err("ERR value is not an integer".to_string()),
            },
            _ => {
                return Err(
                    "ERR Operation against a key holding the wrong kind of value".to_string(),
                )
            }
        };

        if let RedisValue::StringValue(val) = &mut redis_data.value {
            *val = result.to_string();
        }

        if !is_aof_recovery {
            self.append_aof(&format!("{} DECR {} {}", db_index, key, increment));
        }

        Ok(result)
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
                        pb.set_style(ProgressStyle::default_bar().template("[{bar:41.green/cyan}] percent: {percent}% lines: {pos}/{len} time: {elapsed_precise}").progress_chars("=>-"));

                        let reader = BufReader::new(&mut file);
                        for line in reader.lines() {
                            if let Ok(operation) = line {
                                let parts: Vec<&str> =
                                    operation.trim().split_whitespace().collect();
                                let db_index = parts[0].to_string();
                                let db_index_usize = db_index.parse::<usize>().unwrap();

                                if !parts.is_empty() {
                                    match parts[1] {
                                        "SET" => {
                                            let key = parts[2].to_string();
                                            let val = parts[3].to_string();
                                            let expire_at = parts
                                                .get(4)
                                                .and_then(|v| v.parse().ok())
                                                .unwrap_or(-1);
                                            if expire_at == -1 {
                                                self.set(db_index_usize, key, val, true);
                                            } else {
                                                if expire_at > current_millis() {
                                                    self.set_with_ttl(
                                                        db_index_usize,
                                                        key,
                                                        val,
                                                        expire_at,
                                                        true,
                                                    );
                                                }
                                            }
                                        }
                                        "DEL" => {
                                            let key = parts[2].to_string();
                                            self.del(db_index_usize, &key, true);
                                        }
                                        "EXPIRE" => {
                                            let key = parts[2].to_string();
                                            let expire_at = parts
                                                .get(3)
                                                .and_then(|v| v.parse().ok())
                                                .unwrap_or(-1);
                                            self.expire(db_index_usize, key, expire_at, true);
                                        }
                                        "RENAME" => {
                                            let old_key = parts[2].to_string();
                                            let new_key = parts[3].to_string();
                                            match self.rename(
                                                db_index_usize,
                                                &old_key,
                                                &new_key,
                                                true,
                                            ) {
                                                Ok(_) => {}
                                                Err(_) => {}
                                            }
                                        }
                                        "MOVE" => {
                                            let key = parts[2].to_string();
                                            let target_db_index = parts[3].to_string();
                                            let target_db_index_usize =
                                                target_db_index.parse::<usize>().unwrap();
                                            self.move_key(
                                                db_index_usize,
                                                &key,
                                                target_db_index_usize,
                                                true,
                                            );
                                        }
                                        "LPUSH" => {
                                            let key = parts[2].to_string();
                                            let values: Vec<String> = parts[3..]
                                                .iter()
                                                .enumerate()
                                                .map(|(_, &x)| x.to_string())
                                                .collect();
                                            self.lpush(db_index_usize, key, values, true);
                                        }
                                        "SADD" => {
                                            let key = parts[2].to_string();
                                            let values: Vec<String> = parts[3..]
                                                .iter()
                                                .enumerate()
                                                .map(|(_, &x)| x.to_string())
                                                .collect();
                                            match self.sadd(db_index_usize, key, values, true) {
                                                Ok(_) => {}
                                                Err(_) => {}
                                            };
                                        }
                                        "RPUSH" => {
                                            let key = parts[2].to_string();
                                            let values: Vec<String> = parts[3..]
                                                .iter()
                                                .enumerate()
                                                .map(|(_, &x)| x.to_string())
                                                .collect();
                                            self.rpush(db_index_usize, key, values, true);
                                        }
                                        "APPEND" => {
                                            let key = parts[2].to_string();
                                            let value = parts[3].to_string();
                                            match self.append(db_index_usize, key, value, true) {
                                                Ok(_) => {}
                                                Err(_) => {}
                                            };
                                        }
                                        "INCR" => {
                                            let key = parts[2].to_string();
                                            let increment_str = parts[3].to_string();
                                            let increment = increment_str.parse::<i64>().unwrap();
                                            match self.incr(db_index_usize, key, increment, true) {
                                                Ok(_) => {}
                                                Err(_) => {}
                                            };
                                        }
                                        "DECR" => {
                                            let key = parts[2].to_string();
                                            let increment_str = parts[3].to_string();
                                            let increment = increment_str.parse::<i64>().unwrap();
                                            match self.decr(db_index_usize, key, increment, true) {
                                                Ok(_) => {}
                                                Err(_) => {}
                                            };
                                        }
                                        "LPOP" => {
                                            let key = parts[2].to_string();
                                            self.lpop(db_index_usize, key, true);
                                        }
                                        "RPOP" => {
                                            let key = parts[2].to_string();
                                            self.rpop(db_index_usize, key, true);
                                        }
                                        "FLUSHALL" => self.flush_all(true),
                                        "FLUSHDB" => self.flush_db(db_index_usize, true),
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
