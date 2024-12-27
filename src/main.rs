use rudis_server::args::Args;
use rudis_server::command::Command;
use rudis_server::db::{DbManager, Message};
use rudis_server::frame::Frame;
use rudis_server::session::SessionManager;
use std::process::id;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::oneshot;
use clap::Parser;

/*
 * 启动服务
 * 
 * @param args 启动参数
 */
fn server_info(args: Arc<Args>) {
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
    "#, version, args.port, pid);
    println!("{}", pattern);
}

#[tokio::main]
async fn main()  {

    let args = Arc::new(Args::parse()); 
    std::env::set_var("RUST_LOG", &args.loglevel);
    env_logger::init();

    let session_manager = Arc::new(SessionManager::new(args.clone())); // 会话管理器
    let db_manager = Arc::new(DbManager::new(args.clone())); // 数据库管理器

    match TcpListener::bind(format!("{}:{}", args.bind, args.port)).await {
        Ok(listener) => {
            
            server_info(args.clone());
            log::info!("Server initialized");
            log::info!("Ready to accept connections");
            
            loop { 

                match listener.accept().await {

                    Ok((mut stream, _address)) => {
                
                        let address = stream.peer_addr().unwrap();
                        let session_id = address.to_string(); // 会话编号
                        let session_manager_clone = session_manager.clone();
                        let db_manager_clone: Arc<DbManager> = db_manager.clone();
                        session_manager_clone.register(address);

                        tokio::spawn(async move {

                            let mut buf = [0; 1024];
                
                            loop {
                                
                                let n = match stream.read(&mut buf).await {
                                    Ok(n) => {
                                        if n == 0 {
                                            return;
                                        }
                                        n
                                    }
                                    Err(e) => {
                                        if e.raw_os_error() == Some(10054) {
                                            let session_id = stream.peer_addr().unwrap().to_string();
                                            session_manager_clone.destroy(&session_id); // 销毁会话
                                        } else {
                                            eprintln!("failed to read from socket; err = {:?}", e);
                                        }
                                        return;
                                    }
                                };
                
                                let bytes = &buf[0..n];
                                let frame = Frame::parse_from_bytes(bytes).unwrap();
                                let result_command = Command::parse_from_frame(frame);
                                let command = match result_command {
                                    Ok(cmd) => cmd,
                                    Err(e) => {
                                        let frame = Frame::Error(e.to_string());
                                        if let Err(e) = stream.write_all(&frame.as_bytes()).await {
                                            eprintln!("failed to write to socket; err = {:?}", e);
                                            return;
                                        }
                                        continue; 
                                    }
                                };

                                let is_login = match command {
                                    Command::Auth(_) => true,
                                    _ => {
                                        session_manager_clone.is_login(&session_id)
                                    }
                                };

                                if is_login { 
                                    let session = session_manager_clone.get(&session_id).unwrap();
                                    let result = match command { 
                                        Command::Auth(auth) => auth.apply(session_manager_clone.clone(), &session_id),
                                        Command::Select(select) => select.apply(session_manager_clone.clone(), &session_id), 
                                        Command::Unknown(unknown) => unknown.apply(session_manager_clone.clone(), &session_id),
                                        Command::Ping(ping) => ping.apply(session_manager_clone.clone(), &session_id),
                                        _ => {
                                            
                                            let (sender, receiver) = oneshot::channel(); // 创建通道
                                            let target_sender = db_manager_clone.get(session.db()); 
                                            
                                            match target_sender.send(Message {
                                                sender: sender,
                                                command,
                                            }).await {
                                                Ok(()) => {},
                                                Err(e) => {
                                                    eprintln!("Failed to write to socket; err = {:?}", e);
                                                }
                                            };
    
                                            let result = match receiver.await {
                                                Ok(f) => {
                                                    f
                                                },
                                                Err(e) => {
                                                    Frame::Error(format!("{:?}", e))
                                                }
                                            };
    
                                            Ok(result) 
                                        }
                                    };
                    
                                    // 接收 DB 响应
                                    match result {
                                        Ok(f) => {
                                            if let Err(e) = stream.write_all(&f.as_bytes()).await {
                                                eprintln!("Failed to write to socket; err = {:?}", e);
                                                return;
                                            }
                                        }
                                        Err(e) => {
                                            println!("Failed to receive; err = {:?}", e);
                                        }
                                    }
                                } else {
                                    let f = Frame::Error("NOAUTH Authentication required.".to_string());
                                    if let Err(e) = stream.write_all(&f.as_bytes()).await {
                                        eprintln!("Failed to write to socket; err = {:?}", e);
                                    }
                                    continue;
                                }
                            }
                        });
                    },
                    Err(e) => {  
                        log::error!("Failed to accept connection: {}", e);
                    }
                }
            }
        },
        Err(_e) => {
            log::error!("Failed to bind to address {}:{}", args.bind, args.port);
            std::process::exit(1); // 退出程序
        }
    }
}