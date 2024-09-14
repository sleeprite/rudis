use anyhow::Error;
use rudis_server::frame::Frame;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::oneshot;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    let repository = Arc::new(DbRepository::new(16));

    loop {

        let (mut socket, _) = listener.accept().await?;
        let rep_clone: Arc<DbRepository> = repository.clone();

        // 创建会话
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            loop {

                // 读取 WS 消息
                let n = match socket.read(&mut buf).await {
                    Ok(n) => {
                        if n == 0 {
                            return;
                        }
                        n
                    }
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // 解析 WS 消息
                let bytes = &buf[0..n];
                let frame = Frame::parse_from_bytes(bytes).unwrap();
                
                // 转化 DB 命令
                let result_command = Command::parse_from_frame(frame);
                let command = match result_command {
                    Ok(cmd) => cmd,
                    Err(e) => {
                        let frame = Frame::Error(e.to_string());
                        if let Err(e) = socket.write_all(&frame.as_bytes()).await {
                            eprintln!("failed to write to socket; err = {:?}", e);
                            return;
                        }
                        continue; 
                    }
                };

                // 创建 OC 通道
                let (sender, receiver) = oneshot::channel();
                let target_sender = rep_clone.get(0);

                // 发送 DB 命令
                match target_sender.send(Message {
                    sender: sender,
                    command,
                }).await {
                    Err(e) => {
                        eprintln!("Failed to connect to the database: {:?}", e)
                    },
                    Ok(()) => {}
                };

                // 接收 DB 响应
                match receiver.await {
                    Ok(f) => {
                        if let Err(e) = socket.write_all(&f.as_bytes()).await {
                            eprintln!("failed to write to socket; err = {:?}", e);
                            return;
                        }
                    }
                    Err(e) => {
                        println!("Failed to receive DB response.");
                    }
                }
            }
        });
    }
}

// 命令
enum Command {
    Set(Set),
    Get(Get),
    Del(Del),
    Unknown(Unknown),
}

impl Command {

    // 根据 frame 获取 command
    pub fn parse_from_frame(frame: Frame)  -> Result<Self, Error>  {
        let command_name = frame.get(0).unwrap();
        let command = match command_name.to_uppercase().as_str() {
            "SET" => Command::Set(Set::parse_from_frame(frame)?),
            "GET" => Command::Get(Get::parse_from_frame(frame)?),
            "DEL" => Command::Del(Del::parse_from_frame(frame)?),
            _ => Command::Unknown(Unknown::parse_from_frame(frame)?),
        };

        Ok(command)
    }
}

pub struct Unknown {}

impl Unknown {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        Ok(Unknown {})
    }

    pub fn apply(self, db: &Db) -> Result<Frame, Error> {
        Ok(Frame::Ok)
    }
}

pub struct Set {
    key: String,
    val: String,
}

impl Set {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error>{

        let key = frame.get(1);
        let val = frame.get(2);

        if key.is_none() {
            return Err(Error::msg("Key is missing"));
        }

        if val.is_none() {
            return Err(Error::msg("Val is missing"));
        }

        let fianl_key = key.unwrap().to_string();
        let final_val = val.unwrap().to_string();

        Ok(Set { 
            key: fianl_key, 
            val: final_val 
        })
    }

    pub fn apply(self,db: &mut Db) -> Result<Frame, Error> {

        db.record.insert(self.key, Structure::String(self.val));

        Ok(Frame::Ok)
    }
}

pub struct Get {
    key: String,
}

impl Get {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let key = "username".to_string();
        Ok(Get { key })
    }

    pub fn apply(self, db: &Db) -> Result<Frame, Error> {
        Ok(Frame::Ok)
    }
}

pub struct Del {
    key: String,
}

impl Del {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let key = "username".to_string();
        Ok(Del { key }) 
    }

    pub fn apply(self, db: &Db) -> Result<Frame, Error> {
        Ok(Frame::Ok)
    }
}

pub enum Structure {
    String(String),
}

struct Message {
    sender: oneshot::Sender<Frame>,
    command: Command,
}

// Db 仓库
struct DbRepository {
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

struct Db {
    record: HashMap<String, Structure>,
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
            let result: Result<Frame, Error> = match command {
                Command::Set(set) => set.apply(self),
                Command::Get(get) => get.apply(self),
                Command::Del(del) => del.apply(self),
                Command::Unknown(unknown) => unknown.apply(self),
            };

            match result {
                Ok(f) => {
                    if let Err(_) = sender.send(f) {
                        // TODO 处理异常
                    }
                }
                Err(e) => {
                    eprintln!("Error applying command: {:?}", e);
                }
            }
        }
    }

    // TODO 常用方法
}