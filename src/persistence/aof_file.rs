use std::{fs, path::PathBuf};

use tokio::{fs::OpenOptions, io::AsyncWriteExt, sync::mpsc::{self, Receiver, Sender}};

use crate::{frame::Frame};

pub struct AofFile {
    sender: Sender<Frame>
}

impl AofFile {
    
    /// 创建 AOF 处理实例
    pub fn new(file_path: PathBuf) -> Self {
        let (sender, receiver) = mpsc::channel(1024);
        tokio::spawn(Self::persist_loop(file_path, receiver));
        AofFile { sender }
    }

    /// 获取 AOF 发送通道
    pub fn get_sender(&self) -> Sender<Frame> {
        self.sender.clone()
    }

    /// 后台 AOF 写入任务
    pub async fn persist_loop(file_path: PathBuf, mut receiver: Receiver<Frame>) {

        // 确保目录存在
        if let Some(parent) = file_path.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                log::error!("Failed to create AOF directory: {}", e);
                return;  // 目录创建失败时退出任务
            }
        }

        let mut open_options = OpenOptions::new();
        open_options.create(true);
        open_options.append(true);

        // 确保文件存在
        let mut file = match open_options.open(&file_path).await {
            Ok(file) => file,
            Err(e) => {
                log::error!("Failed to open AOF file: {}", e);
                return;  // 文件打开失败时退出任务
            }
        };

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