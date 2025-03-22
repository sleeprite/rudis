use anyhow::Error;
use std::process::id;
use std::sync::Arc;

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

use crate::args::Args;
use crate::db::{DbManager, DbMessage};
use crate::frame::Frame;
use crate::command::Command;

pub struct Server {
    args: Arc<Args>,
    db_manager: Arc<DbManager>,
}

impl Server {

    pub fn new(args: Arc<Args>, db_manager: Arc<DbManager>) -> Self {
        Server { args, db_manager }
    }

    pub async fn start(&self) {
        match TcpListener::bind(format!("{}:{}", self.args.bind, self.args.port)).await {
            Ok(listener) => {
                self.server_info();
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

    fn server_info(&self) {
        let pid = id();
        let version = env!("CARGO_PKG_VERSION");
        let pattern = format!(
            r#"
             /\_____/\
            /  o   o  \          Rudis {}
           ( ==  ^  == )
            )         (          Bind: {} PID: {}
           (           )
          ( (  )   (  ) )
         (__(__)___(__)__)
        "#, version, self.args.port, pid);
        println!("{}", pattern);
    }
}

pub struct Handler {
    authenticated: bool,
    db_manager: Arc<DbManager>,
    db_sender: Sender<DbMessage>,
    stream: TcpStream,
    args: Arc<Args>
}

impl Handler {

    pub fn new(db_manager: Arc<DbManager>, stream: TcpStream, args: Arc<Args>) -> Self {
        let args_ref = args.as_ref();
        let authenticated = args_ref.requirepass.is_none();
        let db_manager_ref = db_manager.as_ref();
        let db_sender = db_manager_ref.get_sender(0);
        Handler {
            authenticated,
            db_manager,
            db_sender,
            stream,
            args,
        }
    }

    /**
     * 登录认证 - 方法
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
     * 切换 db_sender 发送器
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
     * 读取 Stream 字节
     * 
     * 通过 loop 和 n < temp_buf.len()，读取完整命令
     * 
     * @param self.stream 客户端
     */
    async fn read_bytes(&mut self) -> Result<Vec<u8>, Error> {
        let mut buffer = Vec::new();
        let mut temp_buf = [0; 1024];
        loop {
            let n = match self.stream.read(&mut temp_buf).await {
                Ok(n) => n,
                Err(e) => {  
                    return Err(Error::msg(format!("Failed to read from stream: {:?}", e)));
                }
            };
            buffer.extend_from_slice(&temp_buf[..n]);
            if n < temp_buf.len() {
                break;
            }
        }
        Ok(buffer)
    }

    /**
     * 写入 Frame 到客户端
     * 
     * 如果写入失败，会记录错误并直接返回，不会抛出错误。
     * 
     * @param frame
     */
    async fn write_frame(&mut self, frame: Frame) {
        let bytes = frame.as_bytes();
        if let Err(e) = self.stream.write_all(&bytes).await {
            eprintln!("Failed to write to socket; err = {:?}", e);
        }
    }

    pub async fn handle(&mut self) {

        loop {
            
            let bytes = match self.read_bytes().await {
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
                    self.write_frame(frame).await;
                    continue;
                }
            };

            match command {
                Command::Auth(_) => {},
                _ => { 
                    if self.args.requirepass.is_some() {
                        if self.authenticated == false {
                            let frame = Frame::Error("NOAUTH Authentication required.".to_string());
                            self.write_frame(frame).await;
                            continue;
                        }
                    } 
                },
            };

            let result = match command {
                Command::Auth(auth) => auth.apply(self),
                Command::Save(save) => save.apply(self.db_manager.clone()).await,
                Command::Bgsave(bgsave) => bgsave.apply(self.db_manager.clone()).await,
                Command::Flushall(flushall) => flushall.apply(self.db_manager.clone()).await,
                Command::Select(select) => select.apply(self),
                Command::Unknown(unknown) => unknown.apply(),
                Command::Ping(ping) => ping.apply(),
                Command::Echo(echo) => echo.apply(),
                _ => {
                    
                    let (sender, receiver) = oneshot::channel();
                    match self.db_sender.send(DbMessage {
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
                    self.write_frame(frame).await;
                }
                Err(e) => {
                    println!("Failed to receive; err = {:?}", e);
                }
            }
        }
    }
}