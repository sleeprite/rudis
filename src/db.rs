use std::{
    collections::{BTreeMap, HashMap, HashSet},
    sync::Arc,
    time::Duration,
};

use anyhow::Error;
use regex::Regex;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::{
    mpsc::{channel, Receiver, Sender},
    oneshot,
};

use crate::{config::Config, command::Command, frame::Frame};

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
     * @param config 参数
     */
    pub fn new(config: Arc<Config>) -> Self {
        let mut dbs = Vec::new();
        let mut senders = Vec::new();

        for _ in 0..config.databases {
            let db = Db::new();
            senders.push(db.sender.clone());
            dbs.push(db);
        }

        for mut db in dbs {
            tokio::spawn(async move {
                db.run().await; // 运行项目 - 独立线程
            });
        }

        DbManager { senders }
    }

    /**
     * 获取发送者
     *
     * @param idx 数据库索引
     */
    pub fn get_sender(&self, idx: usize) -> Sender<DbMessage> {
        if let Some(sender) = self.senders.get(idx) {
            sender.clone()
        } else {
            panic!("Index out of bounds");
        }
    }

    /**
     * 获取发送者【所有】
     */
    pub fn get_senders(&self) -> Vec<Sender<DbMessage>> {
        self.senders.clone()
    }
}

pub enum Structure {
    String(String),
    Hash(HashMap<String, String>),
    SortedSet(BTreeMap<String, f64>),
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
                Command::Flushdb(flushdb) => flushdb.apply(self),
                Command::RandomKey(randomkey) => randomkey.apply(self),
                Command::Renamenx(renamenx) => renamenx.apply(self),
                Command::Rename(rename) => rename.apply(self),
                Command::Exists(exists) => exists.apply(self),
                Command::Expire(expire) => expire.apply(self),
                Command::Ttl(ttl) => ttl.apply(self),
                Command::Type(r#type) => r#type.apply(self),
                Command::Pttl(pttl) => pttl.apply(self),
                Command::Mset(mset) => mset.apply(self),
                Command::Mget(mget) => mget.apply(self),
                Command::Strlen(strlen) => strlen.apply(self),
                Command::Append(append) => append.apply(self),
                Command::Dbsize(dbsize) => dbsize.apply(self),
                Command::Persist(persist) => persist.apply(self),
                Command::Hexists(hexists) => hexists.apply(self),
                Command::Hstrlen(hstrlen) => hstrlen.apply(self),
                Command::Hgetall(hgetall) => hgetall.apply(self),
                Command::Hsetnx(hsetnx) => hsetnx.apply(self),
                Command::Hmget(hmget) => hmget.apply(self),
                Command::Hmset(hmset) => hmset.apply(self),
                Command::Hset(hset) => hset.apply(self),
                Command::Hget(hget) => hget.apply(self),
                Command::Hdel(hdel) => hdel.apply(self),
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
                Command::Scard(scard) => scard.apply(self),
                Command::Spop(spop) => spop.apply(self),
                Command::Srem(srem) => srem.apply(self),
                Command::Sinter(sinter) => sinter.apply(self),
                Command::Sunionstore(sunionstore) => sunionstore.apply(self),
                Command::Sismember(sismember) => sismember.apply(self),
                Command::Smembers(smembers) => smembers.apply(self),
                Command::Sunion(sunion) => sunion.apply(self),
                Command::Rpushx(rpushx) => rpushx.apply(self),
                Command::Lpushx(lpushx) => lpushx.apply(self),
                Command::Incr(incr) => incr.apply(self),
                Command::Decr(decr) => decr.apply(self),
                Command::Lset(lset) => lset.apply(self),
                Command::Zadd(zadd) => zadd.apply(self),
                Command::Zcount(zcount) => zcount.apply(self),
                Command::Zscore(zscore) => zscore.apply(self),
                Command::Zcard(zcard) => zcard.apply(self),
                Command::Zrank(zrank) => zrank.apply(self),
                Command::Zrem(zrem) => zrem.apply(self),
                Command::ExpireAt(expireat) => expireat.apply(self),
                Command::Incrby(incrby) => incrby.apply(self),
                Command::Decrby(decrby) => decrby.apply(self),
                Command::PexpireAt(pexpireat) => pexpireat.apply(self),
                Command::Pexpire(pexpire) => pexpire.apply(self),
                Command::Lrange(lrange) => lrange.apply(self),
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
     * @param ttl 距离现在多少【毫秒】后过期
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
     * 随机返回一个键
     */
    pub fn random_key(&self) -> Option<String> {
        let keys: Vec<String> = self.records.keys().cloned().collect();
        if keys.is_empty() {
            return None;
        }
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let random_index = (now as usize) % keys.len();
        Some(keys[random_index].clone())
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
     * 匹配 key 的逻辑
     * 
     * @param key 键名称
     * @param pattern 表达式
     */
    fn match_pattern(&self, key: &str, pattern: &str) -> bool {
        fn convert_pattern(pattern: &str) -> String {
            let mut regex_pattern = String::new();
            let mut chars = pattern.chars().peekable();
            while let Some(p) = chars.next() {
                match p {
                    '*' => regex_pattern.push_str(".*"), // 匹配任意字符任意次
                    '?' => regex_pattern.push('.'),     // 匹配任意单个字符
                    '[' => {
                        regex_pattern.push('[');
                        if let Some(next) = chars.peek() {
                            if *next == '^' {
                                regex_pattern.push('^');
                                chars.next(); // 跳过 '^'
                            }
                        }
                        while let Some(ch) = chars.next() {
                            if ch == ']' {
                                break;
                            }
                            regex_pattern.push(ch);
                        }
                        regex_pattern.push(']');
                    }
                    _ => regex_pattern.push(p) // 其他字符直接添加
                }
            }
            regex_pattern
        }
        let regex_pattern = convert_pattern(pattern);
        let regex = Regex::new(&regex_pattern).unwrap();
        regex.is_match(key)
    }
}
