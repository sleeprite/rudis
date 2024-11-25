use rudis_server::command::Command;
use rudis_server::db::DbManager;
use rudis_server::frame::Frame;
use rudis_server::message::Message;
use rudis_server::session::{Session, SessionManager};
use std::process::id;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::oneshot;
use clap::Parser;

/*
 * 启动服务
 */
fn println_banner(args: Arc<Args>) {
    let version = env!("CARGO_PKG_VERSION");
    let pid = id();
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

#[derive(Parser)]
#[command(version, author, about, long_about = None)]
struct Args {

    #[arg(short, long)] 
    requirepass: Option<String>,

    #[arg(short, long, default_value = "16")]
    databases: usize,

    #[arg(short, long, default_value = "127.0.0.1")] 
    bind: String,

    #[arg(short, long, default_value = "6379")]
    port: String

}

#[tokio::main]
async fn main()  {

    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let args = Arc::new(Args::parse());
    let db_manager = Arc::new(DbManager::new(args.databases));
    let session_manager = Arc::new(SessionManager::new());
    match TcpListener::bind(format!("{}:{}", args.bind, args.port)).await {
        Ok(listener) => {

            println_banner(args.clone());
            log::info!("Server initialized");
            log::info!("Ready to accept connections");
            
            loop {

                match listener.accept().await {
                    Ok((mut stream, _address)) => {
                        
                        // 共享状态
                        let args_clone = args.clone();
                        let db_manager_clone: Arc<DbManager> = db_manager.clone();
                        let session_manager_clone = session_manager.clone();

                        // 创建会话
                        let address = stream.peer_addr().unwrap();
                        let session = Session::new(args_clone.requirepass.is_none(), address);
                        session_manager_clone.register(address.to_string(), session);

                        tokio::spawn(async move {

                            let mut buf = [0; 1024];
                
                            loop {
                                
                                // Read message
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
                                            session_manager_clone.destroy(&session_id);
                                        } else {
                                            eprintln!("failed to read from socket; err = {:?}", e);
                                        }
                                        return;
                                    }
                                };
                
                                // Analyze command frames
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


                                // 登录拦截器 
                                if args_clone.requirepass.is_some() {
                                    
                                    //（1）已登录：继续任务

                                    //（2）未登录：响应错误
                                }
                
                                // Execute command
                                let result = match command {
                                    Command::Select(select) => select.apply(),
                                    Command::Auth(auth) => auth.apply(),
                                    _ => {
                                        
                                        let (sender, receiver) = oneshot::channel();
                                        let target_sender = db_manager_clone.get(0); // 获取 Session 正在操作的数据库
                                        
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
                            }
                        });
                    },
                    Err(_e) => {  
                        // TODO 连接异常
                    }
                }
            }
        },
        Err(_e) => {
            // TODO 创建失败
        }
    }
}