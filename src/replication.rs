use std::{sync::Arc};

use anyhow::{Error, Result};
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::{args::Args, frame::Frame};

/// 复制状态
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ReplicationState {
    /// 未连接
    Disconnected,
    /// 连接中
    Connecting,
    /// 等待PSYNC响应
    WaitPsync,
    /// 接收RDB文件
    ReceivingRdb,
    /// 已连接，等待命令
    Connected,
}

pub struct ReplicationManager {
    pub state: ReplicationState,
    pub stream: Option<TcpStream>,
    pub args: Arc<Args>
}

impl ReplicationManager {

    pub fn new(args: Arc<Args>) -> Self {
        
        Self {
            state: ReplicationState::Disconnected,
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
                        // 1. 发送PING命令进行握手
                        // 2. 发送REPLCONF命令配置从节点
                        self.ping().await?;
                        self.replconf().await?;
                        // 3. 发送PSYNC命令启动同步
                        // 4. 处理PSYNC响应
                        // 5. 进入命令传播模式
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
                log::info!("Received PONG from master");
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
        
        // 等待 REPLCONF 响应
        let mut buffer = [0; 1024];
        let n = stream.read(&mut buffer).await?;
        let response = Frame::parse_from_bytes(&buffer[..n]).unwrap();
        if let Frame::SimpleString(s) = response {
            if s == "OK" {
                log::info!("REPLCONF acknowledged by master");
                return Ok(());
            }
        }

        let msg_str = "Send REPLCONF failure";
        let msg = Error::msg(msg_str);
        Err(msg)
    }
}