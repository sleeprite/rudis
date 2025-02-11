use std::sync::Arc;

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream, sync::{mpsc::Sender, oneshot}};

use crate::{
    args::Args, command::Command, db::{DbManager, DbMessage}, frame::Frame
};

pub struct Handler {
    authenticated: bool,
    db_sender: Sender<DbMessage>,
    db_manager: Arc<DbManager>,
    stream: TcpStream,
    args: Arc<Args>
}

impl Handler {

    /**
     * 创建会话处理器
     */
    pub fn new(db_manager: Arc<DbManager>, stream: TcpStream, args: Arc<Args>) -> Self {
        let args_ref = args.as_ref();
        let authenticated = args_ref.requirepass.is_none();
        let db_manager_ref = db_manager.as_ref();
        Handler {
            authenticated,
            db_sender: db_manager_ref.get_sender(0),
            db_manager,
            stream,
            args,
        }
    }

    /**
     * 登录认证 - 方法
     * 
     * @param input_requirepass 输入密码【只读】
     */
    pub fn login(&mut self, input_requirepass: &String) -> bool {
        if let Some(ref requirepass) = self.args.requirepass {
            if requirepass == input_requirepass {
                self.authenticated = true;
                return true;
            }
            return false;
        } else {
            true
        }
    }

    /**
     * 拦截器，是否登录
     * 
     * @param command 当前命令【只读】
     */
    pub fn is_logged_in(&mut self, command: &Command) -> bool {
        match command {
            Command::Auth(_) => true,
            _ => {
                if self.args.requirepass.is_some() {
                    self.authenticated
                } else {
                    true
                }
            },
        }
    }

    pub fn change_sender(&mut self, idx: usize) {
        self.db_sender = self.db_manager.get_sender(idx);
    }

    pub async fn run(&mut self) {

        let mut buf = [0; 1024]; 

        loop {
            
            let n = match self.stream.read(&mut buf).await {
                Ok(n) => {
                    if n == 0 {
                        return;
                    } 
                    n
                }
                Err(e) => {
                    if e.raw_os_error() == Some(10054) {
                        return; 
                    } 
                    return;
                }
            };

            let bytes = &buf[0..n];
            let frame = Frame::parse_from_bytes(bytes).unwrap();
            let command = match Command::parse_from_frame(frame) {
                Ok(cmd) => cmd,
                Err(e) => {
                    let frame = Frame::Error(e.to_string());
                    if let Err(e) = self.stream.write_all(&frame.as_bytes()).await {
                        eprintln!("failed to write to socket; err = {:?}", e);
                        return;
                    }
                    continue;
                }
            };

            match command {
                Command::Auth(_) => {},
                _ => { 
                    if self.args.requirepass.is_some() {
                        if self.authenticated == false {
                            let f = Frame::Error("NOAUTH Authentication required.".to_string());
                            if let Err(e) = self.stream.write_all(&f.as_bytes()).await {
                                eprintln!("Failed to write to socket; err = {:?}", e);
                            }
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
                Ok(f) => {
                    if let Err(e) = self.stream.write_all(&f.as_bytes()).await {
                        eprintln!("Failed to write to socket; err = {:?}", e);
                        return;
                    }
                }
                Err(e) => {
                    println!("Failed to receive; err = {:?}", e);
                }
            }
        }
    }
}
