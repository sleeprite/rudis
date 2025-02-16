use std::sync::Arc;

use anyhow::Error;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream, sync::{mpsc::Sender, oneshot}};

use crate::{
    config::Config, command::Command, db::{DbManager, DbMessage}, frame::Frame
};

pub struct ServerHandler {
    authenticated: bool,
    db_manager: Arc<DbManager>,
    db_sender: Sender<DbMessage>,
    stream: TcpStream,
    config: Arc<Config>
}

impl ServerHandler {

    pub fn new(db_manager: Arc<DbManager>, stream: TcpStream, config: Arc<Config>) -> Self {
        let db_manager_ref = db_manager.as_ref();
        let db_sender = db_manager_ref.get_sender(0);
        let config_ref = config.as_ref();
        ServerHandler {
            authenticated: config_ref.requirepass.is_none(),
            db_manager,
            db_sender,
            stream,
            config,
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
        if let Some(ref requirepass) = self.config.requirepass {
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
        if self.config.databases - 1 < idx {
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
                    if self.config.requirepass.is_some() {
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
                Command::Flushall(flushall) => flushall.apply(self.db_manager.clone()),
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
