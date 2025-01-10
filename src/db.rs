use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
    time::Duration,
};

use anyhow::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::{
    mpsc::{channel, Receiver, Sender},
    oneshot,
};

use crate::{args::Args,  command::Command, frame::Frame};

/**
 * 消息
 *
 * @param sender 发送者
 * @param command 命令
 */
pub struct DbMessage {
    pub sender: oneshot::Sender<Frame>,
    pub command: Command,
}

// Db 仓库
pub struct DbManager {
    senders: Vec<Sender<DbMessage>>,
}

impl DbManager {
    /**
     * 创建 DB 管理器
     *
     * @param args 参数
     */
    pub fn new(args: Arc<Args>) -> Self {
        let mut dbs = Vec::new();
        let mut senders = Vec::new();

        for _ in 0..args.databases {
            let db = Db::new();
            senders.push(db.sender.clone());
            dbs.push(db);
        }

        for mut db in dbs {
            tokio::spawn(async move {
                db.run().await;
            });
        }

        DbManager { senders }
    }

    /**
     * 获取发送者
     *
     * @param idx 数据库索引
     */
    pub fn get(&self, idx: usize) -> Sender<DbMessage> {
        if let Some(sender) = self.senders.get(idx) {
            sender.clone()
        } else {
            panic!("Index out of bounds");
        }
    }
}

pub enum Structure {
    String(String),
    Hash(HashMap<String, String>),
    Set(HashSet<String>),
    List(Vec<String>),
}

/**
 * 数据库结构
 * 
 * @param receiver
 * @param sender
 */
pub struct Db {
    receiver: Receiver<DbMessage>,
    sender: Sender<DbMessage>,
    pub expire_records: HashMap<String, SystemTime>,
    pub records: HashMap<String, Structure>,
}

impl Db {

    /**
     * 创建数据库
     */
    pub fn new() -> Self {
        let (sender, receiver) = channel(1024);

        Db {
            records: HashMap::new(),
            expire_records: HashMap::new(),
            receiver,
            sender,
        }
    }

    /**
     * 运行数据库
     */
    async fn run(&mut self) {
        while let Some(DbMessage { sender, command }) = self.receiver.recv().await {
            let result: Result<crate::frame::Frame, Error> = match command {
                Command::Set(set) => set.apply(self),
                Command::Get(get) => get.apply(self),
                Command::Del(del) => del.apply(self),
                Command::Rename(rename) => rename.apply(self),
                Command::Exists(exists) => exists.apply(self),
                Command::Expire(expire) => expire.apply(self),
                Command::Flushdb(flushdb) => flushdb.apply(self),
                Command::Pttl(pttl) => pttl.apply(self),
                Command::Ttl(ttl) => ttl.apply(self),
                Command::Type(r#type) => r#type.apply(self),
                Command::Mset(mset) => mset.apply(self),
                Command::Mget(mget) => mget.apply(self),
                Command::Strlen(strlen) => strlen.apply(self),
                Command::Append(append) => append.apply(self),
                Command::Dbsize(dbsize) => dbsize.apply(self),
                Command::Hset(hset) => hset.apply(self),
                Command::Hget(hget) => hget.apply(self),
                Command::Hmset(hmset) => hmset.apply(self),
                Command::Hdel(hdel) => hdel.apply(self),
                Command::Persist(persist) => persist.apply(self),
                Command::Hexists(hexists) => hexists.apply(self),
                Command::Hstrlen(hstrlen) => hstrlen.apply(self),
                Command::Hgetall(hgetall) => hgetall.apply(self),
                Command::Hsetnx(hsetnx) => hsetnx.apply(self),
                Command::Hmget(hmget) => hmget.apply(self),
                Command::Keys(keys) => keys.apply(self),
                Command::Hlen(hlen) => hlen.apply(self),
                Command::Hkeys(hkeys) => hkeys.apply(self),
                Command::Hvals(hvals) => hvals.apply(self),
                Command::Lpush(lpush) => lpush.apply(self),
                Command::Rpush(rpush) => rpush.apply(self),
                Command::Lindex(lindex) => lindex.apply(self),
                Command::Lpop(lpop) => lpop.apply(self),
                Command::Rpop(rpop) => rpop.apply(self),
                Command::Llen(llen) => llen.apply(self),
                Command::Sadd(sadd) => sadd.apply(self),
                Command::Smembers(smembers) => smembers.apply(self),
                Command::Scard(scard) => scard.apply(self),
                _ => Err(Error::msg("Unknown command")),
            };

            match result {
                Ok(f) => if let Err(_) = sender.send(f) {},
                Err(e) => {
                    eprintln!("Error applying command: {:?}", e);
                }
            }
        }
    }

    /**
     * 保存键值
     *
     * @param key 键名
     * @param value 值
     */
    pub fn insert(&mut self, key: String, value: Structure) {
        self.records.insert(key, value);
    }

    /**
     * 获取键值
     *
     * @param key 键名
     */
    pub fn get(&mut self, key: &str) -> Option<&Structure> {
        self.expire_if_needed(key);
        self.records.get(key)
    }

    /**
     * 获取键值【引用】
     *
     * @param key 键名
     */
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Structure> {
        self.expire_if_needed(key);
        self.records.get_mut(key)
    }

    /**
     * 设置过期
     *
     * @param key 键名
     * @param ttl 过期时间（毫秒）
     */
    pub fn expire(&mut self, key: String, ttl: u64) {
        let expire_time = SystemTime::now() + std::time::Duration::from_millis(ttl);
        self.expire_records.insert(key, expire_time);
    }

    /**
     * 删除键值
     *
     * @param key 键名
     * @return 如果删除成功，返回被删除的值；如果删除失败，返回 None
     */
    pub fn remove(&mut self, key: &str) -> Option<Structure> {
        if self.records.contains_key(key) {
            self.expire_records.remove(key);
            self.records.remove(key)
        } else {
            None
        }
    }

    /**
     * 过期检测
     *
     * @param key 键名
     */
    pub fn expire_if_needed(&mut self, key: &str) {
        if let Some(expire_time) = self.expire_records.get(key) {
            let now = SystemTime::now();
            if now.duration_since(UNIX_EPOCH).unwrap().as_secs()
                > expire_time.duration_since(UNIX_EPOCH).unwrap().as_secs()
            {
                self.expire_records.remove(key);
                self.records.remove(key);
            }
        }
    }

    /**
     * 获取过期毫秒数
     *
     * @param key 键名
     * @return 过期毫秒数，如果键不存在则返回 -2，如果键已过期则返回 -1
     */
    pub fn ttl_millis(&mut self, key: &str) -> i64 {
        if let Some(expire_time) = self.expire_records.get(key) {
            let now = SystemTime::now();
            if now >= *expire_time {
                self.remove(key); // 键已过期，应该从数据库中移除
                -1
            } else {
                let duration = expire_time.duration_since(now).unwrap_or(Duration::new(0, 0));
                duration.as_secs() as i64 * 1000 + duration.subsec_millis() as i64
                // 计算剩余时间
            }
        } else if self.records.contains_key(key) {
            -2 // 键存在但没有设置过期时间
        } else {
            -2 // 键不存在
        }
    }

    /**
     * 检查键是否存在
     *
     * @param key 键名
     * @return 如果键存在返回 true，否则返回 false
     */
    pub fn exists(&self, key: &str) -> bool {
        self.records.contains_key(key)
    }

    /**
     * 获取符合给定模式的所有键
     *
     * @param pattern 模式
     * @return 符合模式的所有键的列表
     */
    pub fn keys(&self, pattern: &str) -> Vec<String> {
        self.records.keys().filter(|key| self.match_pattern(key, pattern)).cloned().collect()
    }


     /**
     * 简单的模式匹配函数
     *
     * @param key 键
     * @param pattern 模式
     * @return 如果键符合模式返回 true，否则返回 false
     */
    fn match_pattern(&self, key: &str, pattern: &str) -> bool {
        fn is_match(key: &str, pattern: &str) -> bool {
            let mut key_chars = key.chars().peekable();
            let mut pattern_chars = pattern.chars().peekable();
    
            while let Some(p) = pattern_chars.next() {
                match p {
                    '*' => {
                        if pattern_chars.peek().is_none() {
                            // 如果模式中 '*' 是最后一个字符，那么剩下的 key 都是匹配的
                            return true;
                        }
                        // 尝试匹配 '*' 后面的模式
                        let mut next_pattern = String::new();
                        while let Some(ch) = pattern_chars.next() {
                            if ch == '*' {
                                break;
                            }
                            next_pattern.push(ch);
                        }
                        if next_pattern.is_empty() {
                            return true;
                        }
                        // 尝试在 key 的剩余部分中找到匹配
                        let mut key_start = 0;
                        while let Some(_k) = key_chars.next() {
                            key_start += 1;
                            if is_match(&key[key_start - 1..], &next_pattern) {
                                return true;
                            }
                        }
                        return false;
                    }
                    '?' => {
                        if key_chars.next().is_none() {
                            return false;
                        }
                    }
                    '[' => {
                        let mut set = String::new();
                        let mut negate = false;
                        if let Some(ch) = pattern_chars.next() {
                            if ch == '^' {
                                negate = true;
                            } else {
                                set.push(ch);
                            }
                        }
                        while let Some(ch) = pattern_chars.next() {
                            if ch == ']' {
                                break;
                            }
                            set.push(ch);
                        }
                        if let Some(k) = key_chars.next() {
                            if (negate && set.contains(k)) || (!negate && !set.contains(k)) {
                                return false;
                            }
                        } else {
                            return false;
                        }
                    }
                    _ => {
                        if key_chars.next() != Some(p) {
                            return false;
                        }
                    }
                }
            }
    
            key_chars.next().is_none()
        }
    
        is_match(key, pattern)
    }
}
