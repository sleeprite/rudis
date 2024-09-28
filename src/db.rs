use std::collections::HashMap;

use anyhow::Error;
use tokio::sync::mpsc::{channel, Receiver, Sender};

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

pub struct Db {
    pub record: HashMap<String, Structure>,
    sender: Sender<Message>,
    receiver: Receiver<Message>,
}

impl Db {

    pub fn new() -> Self {

        let (sender, receiver) = channel(1024);

        Db {
            record: HashMap::new(),
            sender,
            receiver,
        }
    }

    async fn run(&mut self) {

        while let Some(Message { sender, command }) = self.receiver.recv().await { 
            
            let result: Result<crate::frame::Frame, Error> = match command {
                Command::Set(set) => set.apply(self),
                Command::Get(get) => get.apply(self),
                Command::Del(del) => del.apply(self),
                Command::Unknown(unknown) => unknown.apply(self),
                _ => Err(Error::msg("program exception"))
            };

            match result {
                Ok(f) => {
                    if let Err(_) = sender.send(f) {}
                }
                Err(e) => {
                    eprintln!("Error applying command: {:?}", e);
                }
            }
        }
    }
}