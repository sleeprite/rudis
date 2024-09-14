use rudis_server::command::Command;
use rudis_server::db::DbRepository;
use rudis_server::frame::Frame;
use rudis_server::message::Message;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::oneshot;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // TCP 监听器 
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