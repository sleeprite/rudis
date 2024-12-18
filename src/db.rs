use std::{collections::HashMap, sync::Arc, time::Duration};

use anyhow::Error;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{args::Args, command::Command, message::Message, structure::Structure};

// Db 仓库
pub struct DbManager {
    senders: Vec<Sender<Message>>,
}

impl DbManager {
    
    // 创建 Db 并维护 sender 对象
    pub fn new(args: Arc<Args>) -> Self {

        // 创建 DB 实例（单线程）
        let mut dbs = Vec::new();
        let mut senders = Vec::new();

        for _ in 0..args.databases {
            let db = Db::new();
            senders.push(db.sender.clone());
            dbs.push(db);
        }

        // 启动 DB 实例（多线程）
        for mut db in dbs {
            tokio::spawn(async move {
                db.run().await;
            });
        }

        DbManager { senders }
    }

    pub fn get(&self, idx: usize) -> Sender<Message> {
        if let Some(sender) = self.senders.get(idx) {
            sender.clone()
        } else {
            panic!("Index out of bounds");
        }
    }
}

/**
 * @param receiver
 * @param sender
 */
pub struct Db {
    receiver: Receiver<Message>,
    sender: Sender<Message>,
    pub records: HashMap<String, Structure>,
    pub expire_records: HashMap<String, SystemTime>,
}

impl Db {

    pub fn new() -> Self {
        let (sender, receiver) = channel(1024);

        Db {
            records: HashMap::new(),
            expire_records: HashMap::new(),
            receiver,
            sender,
        }
    }

    async fn run(&mut self) {
        
        while let Some(Message { sender, command }) = self.receiver.recv().await {
        
            let result: Result<crate::frame::Frame, Error> = match command {
                Command::Set(set) => set.apply(self),
                Command::Get(get) => get.apply(self),
                Command::Del(del) => del.apply(self),
                Command::Expire(expire) => expire.apply(self),
                Command::Unknown(unknown) => unknown.apply(self),
                Command::Pttl(pttl) => pttl.apply(self),
                Command::Ttl(ttl) => ttl.apply(self),
                _ => Err(Error::msg("program exception")),
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
     */
    pub fn remove(&mut self, key: &str) -> bool {
        if self.records.contains_key(key) {
            self.expire_records.remove(key);
            self.records.remove(key);
            return true
        }
        false
    }

    /**
     * 过期检测
     * 
     * @param key 键名
     */
    pub fn expire_if_needed(&mut self, key: &str) {
        if let Some(expire_time) = self.expire_records.get(key) {
            let now = SystemTime::now();
            if now.duration_since(UNIX_EPOCH).unwrap().as_secs() > expire_time.duration_since(UNIX_EPOCH).unwrap().as_secs() {
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
                // 键已过期，应该从数据库中移除
                self.remove(key);
                -1
            } else {
                // 计算剩余时间
                let duration = expire_time.duration_since(now).unwrap_or(Duration::new(0, 0));
                duration.as_secs() as i64 * 1000 + duration.subsec_millis() as i64
            }
        } else if self.records.contains_key(key) {
            // 键存在但没有设置过期时间
            -2
        } else {
            // 键不存在
            -2
        }
    }
}