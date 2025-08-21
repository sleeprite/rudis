use std::{fs, path::PathBuf};

use anyhow::Result;
use tokio::{fs::OpenOptions, io::AsyncWriteExt, sync::mpsc::{self, Receiver, Sender}};

use crate::{frame::Frame};

pub struct AofFile {
    sender: Sender<Frame>,
    file_path: PathBuf
}

impl AofFile {
    
    /// 创建 AOF 处理实例
    pub fn new(file_path: PathBuf) -> Self {
        let (sender, receiver) = mpsc::channel(1024);
        let aof_file = AofFile {
            sender,
            file_path: file_path.clone(), // 保存文件路径
        };
        tokio::spawn(Self::persist_loop(file_path, receiver));
        aof_file
    }

    /// 获取 AOF 发送通道
    pub fn get_sender(&self) -> Sender<Frame> {
        self.sender.clone()
    }

    pub async fn read_all_frames(&self) -> Result<Vec<Frame>> {
        if !self.file_path.exists() {
            return Ok(Vec::new());
        }

        let content = tokio::fs::read(&self.file_path).await?;
        let mut frames = Vec::new();
        let mut start = 0;
        let separator = b"\r\n\r\n";

        // 遍历内容查找分隔符
        while let Some(pos) =&content[start..].windows(separator.len()).position(|window| window == separator) {
            let end = start + pos;
            let frame_data = &content[start..end];
        
            if !frame_data.is_empty() {
                let frame_str = String::from_utf8_lossy(frame_data);
                println!("Raw frame data: {}", frame_str);
                if let Ok(frame) = Frame::parse_from_bytes(frame_data) {
                    frames.push(frame);
                }
            }   
        
            // 跳过分隔符（4字节）
            start = end + separator.len();
        }

        Ok(frames)
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
            if let Err(e) =  file.write_all(b"\r\n").await {
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