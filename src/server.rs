use anyhow::Error;

use tokio::net::TcpStream;

use std::path::PathBuf;
use std::sync::{Arc};

use tokio::net::TcpListener;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

use crate::args::Args;
use crate::persistence::aof_file::AofFile;
use crate::store::db::{DatabaseMessage};
use crate::store::db_manager::DatabaseManager;
use crate::network::connection::Connection;
use crate::replication::ReplicationManager;
use crate::command::Command;
use crate::frame::Frame;

pub struct Server {
    args: Arc<Args>,
    aof_file: Option<AofFile>,
    aof_sender: Option<Sender<Frame>>,
    db_manager: Arc<DatabaseManager>
}

impl Server {

    pub fn new(args: Arc<Args>) -> Self {
        let db_manager = Arc::new(DatabaseManager::new(args.clone()));
        let (aof_file, aof_sender) = if args.appendonly == "yes" {
            let file_path = PathBuf::from(&args.dir).join(&args.appendfilename);
            let file = AofFile::new(file_path);
            let sender = file.get_sender();
            (Some(file), Some(sender))
        } else {
            (None, None)
        };

        Server { 
            args, 
            aof_file, 
            aof_sender,
            db_manager
        }
    }

    pub async fn start(&mut self) {

        // 如果 aof_file.is_some 执行加载的逻辑
        if let Some(af) = &mut self.aof_file {
            match Self::replay_aof_file(af, self.db_manager.clone()).await {
                Ok(_) => log::info!("Successfully loaded AOF file"),
                Err(_) => log::info!("Failed to load AOF file")
            };
        }

        if self.args.is_slave() {
            let args = self.args.clone();
            let db_manager = self.db_manager.clone();
            tokio::spawn(async move {
                let mut rm = ReplicationManager::new(args,  db_manager);
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
                            let aof_sender = self.aof_sender.clone(); 
                            let mut handler = Handler::new(self.db_manager.clone(), stream, self.args.clone(), aof_sender);
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

    async fn replay_aof_file(aof_file: &mut AofFile, _db_manager: Arc<DatabaseManager>) -> Result<(), Error>  {
        let frames = aof_file.read_all_frames().await;
        for _frame in frames.unwrap() {
            println!("load command")
        }
        Ok(())
    }
}

pub struct Handler {
    certification: bool,
    connection: Connection,
    aof_sender: Option<Sender<Frame>>,
    db_manager: Arc<DatabaseManager>,
    sender: Sender<DatabaseMessage>,
    args: Arc<Args>
}

impl Handler {

    pub fn new(db_manager: Arc<DatabaseManager>, stream: TcpStream, args: Arc<Args>, aof_sender: Option<Sender<Frame>>) -> Self {
        let args_ref = args.as_ref();
        let certification = args_ref.requirepass.is_none();
        let sender = db_manager.as_ref().get_sender(0);
        let connection = Connection::new(stream);
        Handler {
            aof_sender,
            certification,
            connection,
            db_manager,
            sender,
            args,
        }
    }

    /**
     * 客户端登录认证
     * 
     * 如果 "密码" 不匹配，响应 ERR invalid password 错误
     * 
     * @param input_requirepass 输入密码【只读】
     */
    pub fn login(&mut self, input_requirepass: &String) -> Result<(), Error> {
        if let Some(ref requirepass) = self.args.requirepass {
            if requirepass == input_requirepass {
                self.certification = true;
                return Ok(())
            } 
            return Err(Error::msg("ERR invalid password"));
        } else {
            Ok(())
        }
    }

    /**
     * 切换当前数据库索引
     * 
     * 如果索引超出，响应 ERR DB index is out of range 错误
     * 
     * @param idx 目标数据库索引
     */
    pub fn change_sender(&mut self, idx: usize) -> Result<(), Error> {
        if self.args.databases - 1 < idx {
            return Err(Error::msg("ERR DB index is out of range"));
        }
        self.sender = self.db_manager.get_sender(idx);
        Ok(())
    }

    /**
     * 处理客户端连接的主循环
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
            let frame_copy = frame.clone(); // 保留原始帧
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
                        if self.certification == false {
                            let frame = Frame::Error("NOAUTH Authentication required.".to_string());
                            self.connection.write_bytes(frame.as_bytes()).await;
                            continue;
                        }
                    } 
                },
            };

            let should_keep_aof_log = command.is_write();
            let result = self.apply_command(command).await;

            match result {
                Ok(frame) => {
                    if should_keep_aof_log {
                        if let Some(ref aof_sender) = self.aof_sender {
                            let _ = aof_sender.send(frame_copy).await;
                        }
                    }
                    self.connection.write_bytes(frame.as_bytes()).await;
                }
                Err(e) => {
                    println!("Failed to receive; err = {:?}", e);
                }
            }
        }
    }

    async fn apply_command(&mut self, command: Command) -> Result<Frame, Error> {
        match command {
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
            _ => self.apply_db_command(command).await,
        }
    }

    async fn apply_db_command(&self, command: Command) -> Result<Frame, Error> {
        
        let (sender, receiver) = oneshot::channel();
        let message = DatabaseMessage::Command { sender, command };
        if let Err(e) = self.sender.send(message).await {
            return Ok(Frame::Error(format!("Channel closed: {:?}", e)));
        }
        
        let result = match receiver.await {
            Ok(f) => f,
            Err(e) => Frame::Error(format!("{:?}", e))
        };

        Ok(result)
    }
}
