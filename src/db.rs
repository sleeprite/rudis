use std::{collections::HashMap, time::Duration};

use anyhow::Error;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{command::Command, message::Message, structure::Structure};

// Db 仓库
pub struct DbRepository {
    senders: Vec<Sender<Message>>,
}

impl DbRepository {
    
    // 创建 Db 并维护 sender 对象
    pub fn new(size: usize) -> Self {

        // 创建 DB 实例（单线程）
        let mut dbs = Vec::new();
        let mut senders = Vec::new();

        for _ in 0..size {
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

        DbRepository { senders }
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
                Command::Unknown(unknown) => unknown.apply(self),
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
    pub fn remove(&mut self, key: &str) {
        self.expire_records.remove(key);
        self.records.remove(key);
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
}