use std::collections::{BTreeSet, HashSet};
use std::{collections::HashMap, sync::Arc};

use crate::tools::date::current_millis;

use super::db_config::RedisConfig;

/*
 * ZsetElement 对象
 *
 * @param value 值
 * @param score 分
 */
#[derive(Debug)]
pub struct ZsetElement {
    value: String,
    score: usize,
}

impl ZsetElement {
    fn new(value: String, score: usize) -> Self {
        ZsetElement { value, score }
    }
}

impl Ord for ZsetElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for ZsetElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for ZsetElement {}

impl PartialEq for ZsetElement {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

pub enum RedisValue {
    StringValue(String),
    ListValue(Vec<String>),
    SetValue(HashSet<String>),
    HashValue(HashMap<String, String>),
    ZsetValue(BTreeSet<ZsetElement>),
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

    pub fn get_value(&self) -> &RedisValue {
        return &self.value;
    }

    pub fn set_expire_at(&mut self, expire_at: i64) {
        self.expire_at = expire_at;
    }
}

pub struct Redis {
    pub databases: Vec<HashMap<String, RedisData>>,
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

        Redis {
            databases,
            redis_config,
        }
    }

    /*
     * 获取数据库列表
     *
     * @return databases
     */
    pub fn get_databases(&self) -> &Vec<HashMap<String, RedisData>> {
        &self.databases
    }

    /*
     * 获取数据库 key 的数量
     *
     * @param db_index 数据库索引
     * @return 数据库大小
     */
    pub fn dbsize(&self, db_index: usize) -> usize {
        if db_index < self.databases.len() {
            self.databases[db_index].len()
        } else {
            panic!("Invalid database index");
        }
    }

    pub fn zadd(
        &mut self,
        db_index: usize,
        key: String,
        value: String,
        score: usize,
    ) -> Result<usize, String> {
        let db = &mut self.databases[db_index];

        let zset = match db.get_mut(&key) {
            Some(redis_data) => match &mut redis_data.value {
                RedisValue::ZsetValue(zset) => zset,
                _ => {
                    return Err(format!(
                        "Key {} exists in the database but is not a sorted set.",
                        key
                    ))
                }
            },
            None => {
                let zset = BTreeSet::new();
                db.insert(key.clone(), RedisData::new(RedisValue::ZsetValue(zset), -1));
                match db.get_mut(&key) {
                    Some(redis_data) => match &mut redis_data.value {
                        RedisValue::ZsetValue(zset) => zset,
                        _ => unreachable!(),
                    },
                    None => unreachable!(),
                }
            }
        };

        zset.insert(ZsetElement::new(value, score));

        Ok(zset.len())
    }

    pub fn zcard(&self, db_index: usize, key: &str) -> Result<usize, String> {
        let db = &self.databases[db_index];

        match db.get(key) {
            Some(redis_data) => match &redis_data.value {
                RedisValue::ZsetValue(zset) => Ok(zset.len()),
                _ => Err(format!(
                    "Key {} exists in the database but is not a sorted set.",
                    key
                )),
            },
            None => Err(format!("Key {} does not exist in the database.", key)),
        }
    }

    pub fn zscore(&self, db_index: usize, key: &str, member: &str) -> Result<Option<usize>, String> {
        let db = &self.databases[db_index];
    
        match db.get(key) {
            Some(redis_data) => match &redis_data.value {
                RedisValue::ZsetValue(zset) => {
                    for zset_element in zset {
                        if zset_element.value == member {
                            return Ok(Some(zset_element.score));
                        }
                    }
                    // If the member is not found in the sorted set
                    Ok(None)
                }
                _ => Err(format!(
                    "Key {} exists in the database but is not a sorted set.",
                    key
                )),
            },
            None => Err(format!("Key {} does not exist in the database.", key)),
        }
    }

    pub fn zcount(&self, db_index: usize, key: &str, min: i64, max: i64) -> Result<usize, String> {
        let db = &self.databases[db_index];

        match db.get(key) {
            Some(redis_data) => match &redis_data.value {
                RedisValue::ZsetValue(zset) => {
                    let count = zset
                        .iter()
                        .filter(|zset_element| {
                            zset_element.score >= min as usize && zset_element.score <= max as usize
                        })
                        .count();
                    Ok(count)
                }
                _ => Err(format!(
                    "Key {} exists in the database but is not a sorted set.",
                    key
                )),
            },
            None => Err(format!("Key {} does not exist in the database.", key)),
        }
    }

    /*
     * 设置值的同时设置过期时间
     *
     * @param db_index DB 索引
     * @param key 数据键
     * @param value 数据值
     * @param ttl 过期时间，单位：毫秒
     */
    pub fn set_with_ttl(&mut self, db_index: usize, key: String, value: String, ttl: i64) {
        if db_index < self.databases.len() {
            let redis_value = RedisData::new(RedisValue::StringValue(value.clone()), ttl);
            self.databases[db_index].insert(key.clone(), redis_value);
        } else {
            panic!("Invalid database index");
        }
    }

    pub fn mset(&mut self, db_index: usize, data: Vec<(String, String)>) {
        if db_index < self.databases.len() {
            for (key, value) in data {
                self.databases[db_index].insert(
                    key.clone(),
                    RedisData::new(RedisValue::StringValue(value.clone()), -1),
                );
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
                    RedisValue::ZsetValue(_) => "zset".to_string(),
                    RedisValue::HashValue(_) => "hash".to_string(),
                },
                None => "none".to_string(),
            }
        } else {
            "Invalid database index".to_string()
        }
    }

    /*
     * 删除数据
     *
     * @param db_index DB 索引
     * @param key 数据主键
     * @return 如果删除成功返回 true，如果不存在返回 false
     */
    pub fn del(&mut self, db_index: usize, key: &String) -> bool {
        if let Some(db) = self.databases.get_mut(db_index) {
            if db.remove(key).is_some() {
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

    /**
     * 检查过期【所有库】
     */
    pub fn check_all_database_ttl(&mut self) {
        for (_db_index, database) in self.databases.iter_mut().enumerate() {
            let mut expired_keys = HashSet::new();
            for (key, value) in database.iter() {
                if value.is_expired() {
                    expired_keys.insert(key.clone());
                }
            }
            for key in expired_keys {
                database.remove(&key);
            }
        }
    }

    /*
     * expire 方法用于设置键的过期时间
     *
     * @param db_index DB 索引
     * @param key 主键
     * @param ttl_millis 过期时间，单位: 毫秒
     */
    pub fn expire(&mut self, db_index: usize, key: String, ttl_millis: i64) -> bool {
        if db_index >= self.databases.len() {
            panic!("Invalid database index");
        }

        if let Some(redis_value) = self.databases[db_index].get_mut(&key) {
            redis_value.set_expire_at(ttl_millis);
            return true;
        }

        false
    }

    /*
     * 清空数据
     *
     * @param db_index DB 索引
     */
    pub fn flush_db(&mut self, db_index: usize) {
        if db_index < self.databases.len() {
            self.databases[db_index].clear();
        }
    }

    /*
     * 清空数据【所有】
     */
    pub fn flush_all(&mut self) {
        for db in &mut self.databases {
            db.clear();
        }
    }

    /*
     * 数据主键重命名
     *
     * @param old_key 旧主键名称
     * @param new_key 新主键名称
     */
    pub fn rename(&mut self, db_index: usize, old_key: &str, new_key: &str) -> Result<bool, &str> {
        let db = match self.databases.get_mut(db_index) {
            Some(db) => db,
            None => return Err("Database index out of bounds"),
        };

        if let Some(value) = db.remove(old_key) {
            db.insert(new_key.to_string(), value);
            return Ok(true);
        }

        Err("ERR no such key")
    }

    /*
     * 移动数据【n 库 -> m 库】
     *
     * @param key 数据主键
     * @param src_db_index 来源 DB 索引
     * @param target_db_index 目标 DB 索引
     */
    pub fn move_key(&mut self, src_db_index: usize, key: &str, target_db_index: usize) -> bool {
        if let Some(src_db) = self.databases.get_mut(src_db_index) {
            if let Some(value) = src_db.remove(key) {
                if let Some(dest_db) = self.databases.get_mut(target_db_index) {
                    if !dest_db.contains_key(key) {
                        dest_db.insert(key.to_string(), value);
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
     * @param db_index DB 索引
     * @param key 列表键
     * @param values 要插入的值
     */
    pub fn lpush(&mut self, db_index: usize, key: String, values: Vec<String>) {
        if db_index < self.databases.len() {
            let list = self.databases[db_index]
                .entry(key.clone())
                .or_insert(RedisData::new(RedisValue::ListValue(vec![]), -1));
            if let RedisValue::ListValue(ref mut current_values) = list.value {
                current_values.splice(0..0, values.clone());
            }
        } else {
            panic!("Invalid database index");
        }
    }

    pub fn hmset(
        &mut self,
        db_index: usize,
        key: String,
        values: HashMap<String, String>,
    ) -> Result<(), &'static str> {
        if let Some(db) = self.databases.get_mut(db_index) {
            if let Some(redis_data) = db.get_mut(&key) {
                if let RedisValue::HashValue(hash_map) = &mut redis_data.value {
                    *hash_map = values;
                    return Ok(());
                } else {
                    return Err("Cannot use hashmap to overwrite values of non HashValue types");
                }
            } else {
                db.insert(key, RedisData::new(RedisValue::HashValue(values), -1));
                return Ok(());
            }
        }
        Err("Invalid database index")
    }

    pub fn hset(
        &mut self,
        db_index: usize,
        key: String,
        field: String,
        value: String,
    ) -> Result<i32, &'static str> {
        if let Some(db) = self.databases.get_mut(db_index) {
            if let Some(redis_data) = db.get_mut(&key) {
                if let RedisValue::HashValue(hash_map) = &mut redis_data.value {
                    hash_map.insert(field.clone(), value.clone());
                    return Ok(1);
                } else {
                    return Err(
                        "Cannot overwrite non HashValue type values with a single field value",
                    );
                }
            } else {
                let mut values = HashMap::new();
                values.insert(field.clone(), value.clone());
                db.insert(
                    key.clone(),
                    RedisData::new(RedisValue::HashValue(values), -1),
                );
                return Ok(1);
            }
        }
        Err("Invalid database index")
    }

    pub fn hget(
        &self,
        db_index: usize,
        key: &str,
        field: &str,
    ) -> Result<Option<String>, &'static str> {
        // 获取数据库索引对应的数据库
        if let Some(db) = self.databases.get(db_index) {
            // 从数据库中获取指定键
            if let Some(redis_data) = db.get(key) {
                // 判断 Redis 数据类型是否为 HashValue
                if let RedisValue::HashValue(hash_map) = &redis_data.value {
                    // 从哈希映射中获取指定字段的值
                    if let Some(value) = hash_map.get(field) {
                        // 返回值
                        return Ok(Some(value.clone()));
                    } else {
                        // 字段不存在
                        return Ok(None);
                    }
                } else {
                    // 键存在，但不是 HashValue 类型
                    return Err(
                        "WRONGTYPE Operation against a key holding the wrong kind of value",
                    );
                }
            } else {
                // 键不存在
                return Ok(None);
            }
        } else {
            // 数据库索引不存在
            return Err("数据库索引不存在");
        }
    }

    pub fn hexists(&self, db_index: usize, key: &str, field: &str) -> Result<bool, &'static str> {
        // 获取数据库索引对应的数据库
        if let Some(db) = self.databases.get(db_index) {
            // 从数据库中获取指定键
            if let Some(redis_data) = db.get(key) {
                // 判断 Redis 数据类型是否为 HashValue
                if let RedisValue::HashValue(hash_map) = &redis_data.value {
                    // 检查哈希映射中是否存在指定字段
                    if hash_map.contains_key(field) {
                        // 字段存在
                        return Ok(true);
                    } else {
                        // 字段不存在
                        return Ok(false);
                    }
                } else {
                    // 键存在，但不是 HashValue 类型
                    return Err(
                        "WRONGTYPE Operation against a key holding the wrong kind of value",
                    );
                }
            } else {
                // 键不存在
                return Ok(false);
            }
        } else {
            // 数据库索引不存在
            return Err("数据库索引不存在");
        }
    }

    pub fn hdel(
        &mut self,
        db_index: usize,
        key: &str,
        fields: &[&str],
    ) -> Result<usize, &'static str> {
        // 获取数据库索引对应的数据库
        if let Some(db) = self.databases.get_mut(db_index) {
            // 从数据库中获取指定键
            if let Some(redis_data) = db.get_mut(key) {
                // 判断 Redis 数据类型是否为 HashValue
                if let RedisValue::HashValue(hash_map) = &mut redis_data.value {
                    let mut deleted_count = 0;
                    for field in fields {
                        // 从哈希映射中删除指定字段
                        if hash_map.remove((*field).to_string().as_str()).is_some() {
                            deleted_count += 1;
                        }
                    }
                    return Ok(deleted_count);
                } else {
                    // 键存在，但不是 HashValue 类型
                    return Err(
                        "WRONGTYPE Operation against a key holding the wrong kind of value",
                    );
                }
            } else {
                // 键不存在
                return Ok(0);
            }
        } else {
            // 数据库索引不存在
            return Err("数据库索引不存在");
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
                .entry(key.clone())
                .or_insert(RedisData::new(RedisValue::ListValue(vec![]), -1));
            if let RedisValue::ListValue(ref mut current_values) = list.value {
                current_values.extend(values.clone());
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
     */
    pub fn lpop(&mut self, db_index: usize, key: String) -> Option<String> {
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
     */
    pub fn rpop(&mut self, db_index: usize, key: String) -> Option<String> {
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
    pub fn append(&mut self, db_index: usize, key: String, value: String) -> Result<usize, String> {
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

        Ok(len)
    }

    /*
     * 将 key 中储存的数字值增一。
     *
     * @param db_index 数据库索引
     * @param key 列表键
     * @param increment 步长
     */
    pub fn incr(&mut self, db_index: usize, key: String, increment: i64) -> Result<i64, String> {
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

        Ok(result)
    }

    /*
     * 将 key 中储存的数字值减一。
     *
     * @param db_index 数据库索引
     * @param key 列表键
     * @param increment 步长
     */
    pub fn decr(&mut self, db_index: usize, key: String, increment: i64) -> Result<i64, String> {
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

        Ok(result)
    }
}
