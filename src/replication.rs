use std::{sync::Arc};

use anyhow::{Error, Result};
use tokio::net::TcpStream;

use crate::args::Args;

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
}