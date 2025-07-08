use std::path::PathBuf;

use tokio::{fs::OpenOptions, io::AsyncWriteExt, sync::mpsc::{self, Receiver, Sender}};

use crate::{frame::Frame};

pub struct AofFile {
    sender: Sender<Frame>
}

impl AofFile {
    
    /// 创建 AOF 处理实例
    pub fn new(file_path: PathBuf) -> Self {
        let (sender, receiver) = mpsc::channel(1024);
        tokio::spawn(Self::write(file_path, receiver));
        AofFile { sender }
    }

    /// 获取 AOF 发送通道
    pub fn get_sender(&self) -> Sender<Frame> {
        self.sender.clone()
    }

    /// 后台 AOF 写入任务
    pub async fn write(file_path: PathBuf, mut receiver: Receiver<Frame>) {
        let mut file = OpenOptions::new().create(true).append(true).open(file_path).await.unwrap();
        while let Some(frame) = receiver.recv().await {
            if let Err(e) = file.write_all(&frame.as_bytes()).await {
                log::error!("Failed to write command to AOF file: {}", e);
                continue;
            }
            if let Err(e) =  file.write_all(b"\n").await {
                log::error!("Failed to write newline to AOF file: {}", e);
                continue;
            };
            if let Err(e) = file.flush().await {
                log::error!("Failed to flush AOF file: {}", e);
                continue;
            };
        }
    }
}