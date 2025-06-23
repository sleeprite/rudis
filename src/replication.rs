use std::{sync::Arc};

use anyhow::{Error, Result};
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::store::db::{DatabaseMessage};
use crate::store::db_manager::DatabaseManager;
use crate::{args::Args, frame::Frame};

/// 复制状态
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ReplicationState {    
    Connecting,    
    Disconnected, 
    WaitPsync,  
    ReceivingRdb,  
    Connected    
}

pub struct ReplicationManager {
    pub state: ReplicationState,
    pub db_manager: Arc<DatabaseManager>,
    pub stream: Option<TcpStream>,
    pub args: Arc<Args>
}

impl ReplicationManager {

    pub fn new(args: Arc<Args>, db_manager: Arc<DatabaseManager>) -> Self {
        
        Self {
            state: ReplicationState::Disconnected,
            db_manager: db_manager,
            stream: None,
            args
        }
    }
    
    /**
     * 连接到主节点
     */
    pub async fn connect(&mut self) -> Result<()> {
        self.state = ReplicationState::Connecting;
        match self.args.replicaof.as_ref() {
            Some(addr) => {
                match TcpStream::connect(addr).await {
                    Ok(mut _stream) => {
                        self.stream = Some(_stream);
                        self.ping().await?; 
                        self.replconf().await?;
                        self.psync().await?; 
                        self.rdb_file_receiver().await?;
                        self.cmd_receiver().await?;
                        Ok(())
                    },
                    Err(_e) => {
                        self.state = ReplicationState::Disconnected;
                        Err(Error::msg("Connection failed"))
                    }
                }
            },
            None => {
                Ok(())
            }
        }
    }

    /**
     * 发送 PING 命令
     * 
     * @param self
     */
    async fn ping(&mut self) -> Result<()> {

        let stream = self.stream.as_mut().unwrap();
        let frame = Frame::Array(vec![Frame::BulkString("PING".to_string())]);
        stream.write_all(&frame.as_bytes()).await?;
        
        // 等待 PING 响应
        let mut buffer = [0; 1024];
        let n = stream.read(&mut buffer).await?;
        let response = Frame::parse_from_bytes(&buffer[..n]).unwrap();
        if let Frame::SimpleString(s) = response {
            if s == "PONG" {
                return Ok(());
            }
        }

        let msg_str = "Master did not respond with PONG";
        let msg = Error::msg(msg_str);
        Err(msg) // 响应 err 信息
    }

    /**
     * 发送 REPLCONF 命令
     * 
     * @param self
     */
    async fn replconf(&mut self) -> Result<()> {

        let stream = self.stream.as_mut().unwrap();

        let port = self.args.port.to_string();
        let bind = self.args.bind.to_string();
        let replconf_str = String::from("REPLCONF");
        let listening_port_str = String::from("LISTENING-PORT");
        let ip_address_str = String::from("IP-ADDRESS");
        
        let replconf_frame = Frame::Array(vec![
            Frame::BulkString(replconf_str),
            Frame::BulkString(listening_port_str),
            Frame::BulkString(port),
            Frame::BulkString(ip_address_str),
            Frame::BulkString(bind),
        ]);

        stream.write_all(&replconf_frame.as_bytes()).await?;
        let mut buffer = [0; 1024];
        let n = stream.read(&mut buffer).await?;
        let response = Frame::parse_from_bytes(&buffer[..n]).unwrap();
        if let Frame::SimpleString(s) = response {
            if s == "OK" {
                return Ok(());
            }
        }

        Err(Error::msg("REPLCONF failed"))
    }

    /**
     * 发送 PSYNC 命令
     * 
     * @param self
     */
    async fn psync(&mut self) -> Result<()> {
        let stream = self.stream.as_mut().unwrap();
        let psync_frame = Frame::Array(vec![Frame::BulkString("PSYNC".to_string())]);
        stream.write_all(&psync_frame.as_bytes()).await?;
        self.state = ReplicationState::WaitPsync;
        Ok(())
    }

    /**
     * 接收 RDB_FILE 内容
     * 
     * @param self
     */
    async fn rdb_file_receiver(&mut self) -> Result<()> {
        let mut buffer = [0; 1024];
        let stream: &mut TcpStream = self.stream.as_mut().unwrap();
        let n = stream.read(&mut buffer).await?;
        let frame = Frame::parse_from_bytes(&buffer[..n]).unwrap();
        let rdb_file = frame.to_rdb_file().unwrap();
        let senders = self.db_manager.get_senders();
        for (db_index, target_sender) in senders.iter().enumerate() {
            match target_sender.send(DatabaseMessage::Restore(rdb_file.get_database(db_index))).await {
                Ok(()) => {}
                Err(e) => {
                    eprintln!("Failed to write to socket; err = {:?}", e);
                }
            };
        }
        Ok(())
    }

    /**
     * 接收 COMMAND 传播
     * 
     * @param self
     */
    async fn cmd_receiver(&mut self) -> Result<()> {
        let stream = self.stream.as_mut().unwrap();
        let mut buffer = [0; 4096];
        loop {
            let n = stream.read(&mut buffer).await?;
            if n == 0 {
                self.state = ReplicationState::Disconnected;
            }
            match Frame::parse_from_bytes(&buffer[..n]) {
                Ok(frame) => {
                    log::error!("Received master node command:{}", frame.to_string());
                }
                Err(e) => {
                    log::error!("Failed to parse master node command: {}", e);
                }
            }
        }
    }
}