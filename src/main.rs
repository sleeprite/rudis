use rudis_server::command::Command;
use rudis_server::db::DbRepository;
use rudis_server::frame::Frame;
use rudis_server::message::Message;
use std::process::id;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::oneshot;
use clap::Parser;

/*
 * 启动服务
 */
pub fn println_banner(port: String) {
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
    "#, version, port, pid);
    println!("{}", pattern);
}

#[derive(Parser)]
#[command(version, author, about, long_about = None)]
struct Args {

    #[arg(short, long, default_value = "127.0.0.1")]
    bind: String,

    #[arg(short, long, default_value = "3306")]
    port: String,

    #[arg(short, long, default_value = "16")]
    databases: usize

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Args::parse();
    let listener = TcpListener::bind(format!("{}:{}", args.bind, args.port)).await?;
    let repository = Arc::new(DbRepository::new(args.databases));
    
    println_banner(args.port);

    loop {
        let (mut socket, _) = listener.accept().await?;
        let repository_clone: Arc<DbRepository> = repository.clone();

        tokio::spawn(async move {
            let mut buf = [0; 1024];
            loop {

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

                let bytes = &buf[0..n];
                let frame = Frame::parse_from_bytes(bytes).unwrap();
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

                let result = match command {
                    Command::Select(select) => select.apply(),
                    Command::Auth(auth) => auth.apply(),
                    _ => {
                        let (sender, receiver) = oneshot::channel();
                        let target_sender = repository_clone.get(0);
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
                        if let Err(e) = socket.write_all(&f.as_bytes()).await {
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
    }
}