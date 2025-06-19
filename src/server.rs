use anyhow::Error;

use tokio::net::TcpStream;

use std::sync::{Arc};

use tokio::net::TcpListener;
use tokio::sync::mpsc::Sender;
use tokio::sync::{oneshot, Mutex};

use crate::args::Args;
use crate::db::{DatabaseManager, DatabaseMessage};
use crate::frame::Frame;
use crate::command::Command;
use crate::network::connection::Connection;
use crate::replication::ReplicationManager;

pub struct Server {
    args: Arc<Args>,
    db_manager: Arc<DatabaseManager>,
    replication_manager: Arc<Mutex<ReplicationManager>>,
}

impl Server {

    pub fn new(args: Arc<Args>) -> Self {
        let db_manager = Arc::new(DatabaseManager::new(args.clone()));
        let replication_manager = Arc::new(Mutex::new(ReplicationManager::new(args.clone(), db_manager.clone())));
        Server { args, db_manager, replication_manager }
    }

    pub async fn start(&self) {

        if self.args.is_slave() { // 如果是 Slave 节点
            let rm = self.replication_manager.clone();
            tokio::spawn(async move {
                let mut rm = rm.lock().await;
                if let Err(e) = rm.connect().await {
                    log::error!("Failed to connect to master: {}", e);
                }
            });
        }

        match TcpListener::bind(format!("{}:{}", self.args.bind, self.args.port)).await {
            Ok(listener) => {
                log::info!("Server initialized");
                log::info!("Ready to accept connections");
                loop {
                    match listener.accept().await {
                        Ok((stream, _address)) => {
                            let mut handler = Handler::new(self.db_manager.clone(), stream, self.args.clone());
                            tokio::spawn(async move {
                                handler.handle().await;
                            });
                        }
                        Err(e) => {
                            log::error!("Failed to accept connection: {}", e);
                        }
                    }
                }
            }
            Err(_e) => {
                log::error!("Failed to bind to address {}:{}", self.args.bind, self.args.port);
                std::process::exit(1);
            }
        }
    }
}

pub struct Handler {
    authenticated: bool,
    connection: Connection,
    db_manager: Arc<DatabaseManager>,
    db_sender: Sender<DatabaseMessage>,
    args: Arc<Args>
}

impl Handler {

    pub fn new(db_manager: Arc<DatabaseManager>, stream: TcpStream, args: Arc<Args>) -> Self {
        let args_ref = args.as_ref();
        let authenticated = args_ref.requirepass.is_none();
        let db_manager_ref = db_manager.as_ref();
        let db_sender = db_manager_ref.get_sender(0);
        let connection = Connection::new(stream);
        Handler {
            authenticated,
            connection,
            db_manager,
            db_sender,
            args,
        }
    }

    /**
     * 登录认证
     * 
     * 如果 "密码" 不匹配，响应 ERR invalid password 错误
     * 
     * @param input_requirepass 输入密码【只读】
     */
    pub fn login(&mut self, input_requirepass: &String) -> Result<(), Error> {
        if let Some(ref requirepass) = self.args.requirepass {
            if requirepass == input_requirepass {
                self.authenticated = true;
                return Ok(())
            } 
            return Err(Error::msg("ERR invalid password"));
        } else {
            Ok(())
        }
    }

    /**
     * 切换索引
     * 
     * 如果索引超出，响应 ERR DB index is out of range 错误
     * 
     * @param idx 索引
     */
    pub fn change_sender(&mut self, idx: usize) -> Result<(), Error> {
        if self.args.databases - 1 < idx {
            return Err(Error::msg("ERR DB index is out of range"));
        }
        self.db_sender = self.db_manager.get_sender(idx);
        Ok(())
    }

    /**
     * 处理请求
     */
    pub async fn handle(&mut self) {

        loop {

            let bytes = match self.connection.read_bytes().await {
                Ok(bytes) => bytes,
                Err(e) => {
                    eprintln!("Failed to read from stream; err = {:?}", e);
                    return;
                }
            };

            let frame = Frame::parse_from_bytes(bytes.as_slice()).unwrap();
            let command = match Command::parse_from_frame(frame) {
                Ok(cmd) => cmd,
                Err(e) => {
                    let frame = Frame::Error(e.to_string());
                    self.connection.write_bytes(frame.as_bytes()).await;
                    continue;
                }
            };

            match command {
                Command::Auth(_) => {},
                _ => { 
                    if self.args.requirepass.is_some() {
                        if self.authenticated == false {
                            let frame = Frame::Error("NOAUTH Authentication required.".to_string());
                            self.connection.write_bytes(frame.as_bytes()).await;
                            continue;
                        }
                    } 
                },
            };

            let result = match command {
                Command::Auth(auth) => auth.apply(self),
                Command::Replconf(replconf) => replconf.apply(),
                Command::Save(save) => save.apply(self.db_manager.clone(), self.args.clone()).await,
                Command::Bgsave(bgsave) => bgsave.apply(self.db_manager.clone(), self.args.clone()).await,
                Command::Psync(psync) => psync.apply(self.db_manager.clone(), self.args.clone()).await,
                Command::Flushall(flushall) => flushall.apply(self.db_manager.clone()).await,
                Command::Select(select) => select.apply(self),
                Command::Unknown(unknown) => unknown.apply(),
                Command::Ping(ping) => ping.apply(),
                Command::Echo(echo) => echo.apply(),
                _ => {
                    
                    let (sender, receiver) = oneshot::channel();
                    match self.db_sender.send(DatabaseMessage::Command {
                            sender: sender,
                            command,
                    }).await { 
                        Ok(()) => {}
                        Err(e) => {
                            eprintln!("Failed to write to socket; err = {:?}", e);
                        }
                    };

                    let result = match receiver.await {
                        Ok(f) => f,
                        Err(e) => Frame::Error(format!("{:?}", e)),
                    };
                    Ok(result)
                }
            };

            match result {
                Ok(frame) => {
                    self.connection.write_bytes(frame.as_bytes()).await;
                }
                Err(e) => {
                    println!("Failed to receive; err = {:?}", e);
                }
            }
        }
    }
}