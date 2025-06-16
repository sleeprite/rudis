use std::{sync::Arc};

use anyhow::{Error, Result};
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::db::{DatabaseManager, DatabaseMessage};
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
    
    pub async fn connect(&mut self) -> Result<()> {
        self.state = ReplicationState::Connecting;
        match self.args.replicaof.as_ref() {
            Some(addr) => {
                match TcpStream::connect(addr).await {
                    Ok(mut _stream) => {
                        self.stream = Some(_stream);
                        self.ping().await?; // 1. 发送PING命令进行握手
                        self.replconf().await?; // 2. 发送REPLCONF命令配置从节点
                        self.psync().await?; // 3. 发送PSYNC命令启动同步
                        self.receive_rdb_file().await?; // 4. 接收RDB_FILE内容
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
        let listening_port_str = String::from("listening-port");
        let ip_address_str = String::from("ip-address");
        
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
     * 接受 PSYNC 响应
     */
    async fn receive_rdb_file(&mut self) -> Result<()> {
        let stream: &mut TcpStream = self.stream.as_mut().unwrap();
        let mut buffer = [0; 1024];
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
}