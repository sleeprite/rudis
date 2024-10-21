use std::collections::HashMap;

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
    pub record: HashMap<String, Structure>,
    pub expire_record: HashMap<String, SystemTime>,
}

impl Db {

    pub fn new() -> Self {
        let (sender, receiver) = channel(1024);

        Db {
            record: HashMap::new(),
            expire_record: HashMap::new(),
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
        self.record.insert(key, value);
    }

    /**
     * 获取键值
     */
    pub fn get(&mut self, key: &str) -> Option<&Structure> {
        self.expire_if_needed(key); 
        self.record.get(key)
    }

    /**
     * 设置过期
     */
    pub fn expire(&mut self, key: String, ttl: u64) {
        let expire_time = SystemTime::now() + std::time::Duration::from_secs(ttl);
        self.expire_record.insert(key, expire_time);
    }

    /**
     * 删除键值
     */
    pub fn remove(&mut self, key: &str) {
        self.expire_record.remove(key);
        self.record.remove(key);
    }

    /**
     * 懒加载
     */
    pub fn expire_if_needed(&mut self, key: &str) {
        if let Some(expire_time) = self.expire_record.get(key) {
            let now = SystemTime::now();
            if now.duration_since(UNIX_EPOCH).unwrap().as_secs() > expire_time.duration_since(UNIX_EPOCH).unwrap().as_secs() {
                self.expire_record.remove(key);
                self.record.remove(key);
            }
        }
    }
}
