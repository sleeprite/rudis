use anyhow::Error;
use crate::{frame::Frame, server::Handler};
use crate::server::blocking::BlockDirection;

/// BRPOP 命令：阻塞式从列表右端弹出元素
///
/// 这个命令需要在 Handler 中处理，因为：
/// 1. 需要异步等待（tokio::select!）
/// 2. 需要访问 blocking_manager 和 session_manager
/// 3. 需要跨任务通信
///
/// 所有逻辑都在 `apply` 方法中，不依赖 server.rs
#[derive(Clone)]
pub struct Brpop {
    keys: Vec<String>,
    timeout: u64,  // 秒，0 表示永久阻塞（实际限制为 3600 秒）
}

impl Brpop {
    /// 从 Frame 解析 BRPOP 命令
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        
        if args.len() < 3 {
            return Err(Error::msg("ERR wrong number of arguments for 'brpop' command"));
        }

        // 安全限制：防止恶意客户端发送过多的 key
        if args.len() - 2 > 1000 {
            return Err(Error::msg("ERR too many keys for 'brpop' command"));
        }
        
        let keys: Vec<String> = args[1..args.len()-1]
            .iter()
            .map(|s| s.to_string())
            .collect();
        
        let timeout = args.last()
            .ok_or_else(|| Error::msg("ERR timeout required"))?
            .parse::<u64>()
            .map_err(|_| Error::msg("ERR timeout is not an integer"))?;
        
        Ok(Brpop { keys, timeout })
    }

    /// 在 Handler 中执行 BRPOP 命令
    /// 
    /// 流程：
    /// 1. 遍历所有 keys 非阻塞检查，如果某个列表非空，立即返回
    /// 2. 如果所有列表都为空，注册阻塞请求
    /// 3. 使用 tokio::select! 等待结果或超时
    pub async fn apply(&mut self, handler: &mut Handler) -> Result<Frame, Error> {
        use tokio::time::{sleep, Duration};
        
        // 1. 遍历所有 keys，非阻塞检查是否非空
        // Redis BRPOP 语义：按顺序检查所有 key，返回第一个非空列表的结果
        for key in &self.keys {
            let pop_result = self.non_blocking_pop(handler, key).await?;
            
            if let Some(value) = pop_result {
                return Ok(Frame::Array(vec![
                    Frame::BulkString(key.clone()),
                    Frame::BulkString(value),
                ]));
            }
            // 当前 key 对应的列表为空，继续检查下一个 key
        }
        
        // 2. 所有列表都为空，注册阻塞请求
        let timeout = if self.timeout == 0 {
            Some(Duration::from_secs(3600)) // 设置合理上限
        } else {
            Some(Duration::from_secs(self.timeout))
        };
        
        let mut blocking_manager = handler.get_state().blocking_list.lock().await;
        let session_id = handler.get_session().get_id();
        let receiver = blocking_manager.register_blocking_request(
            self.keys.clone(),
            session_id,
            BlockDirection::Right,
            timeout,
        );
        drop(blocking_manager); // 释放锁，避免在等待时持有锁
        
        // 3. 使用 tokio::select! 处理超时
        let timeout_duration = timeout.unwrap_or(Duration::from_secs(3600));
        let blocking_manager_clone = handler.get_state().blocking_list.clone();
        
        tokio::select! {
            result = receiver => {
                match result {
                    Ok(frame) => Ok(frame),
                    Err(_) => Ok(Frame::Null),
                }
            }
            _ = sleep(timeout_duration) => {
                // 超时，清理请求
                let mut manager = blocking_manager_clone.lock().await;
                manager.cleanup_session(session_id);
                Ok(Frame::Null)
            }
        }
    }

    /// 非阻塞弹出：尝试从指定列表弹出一个元素
    /// 
    /// 返回 Some(value) 如果列表非空，返回 None 如果列表为空
    async fn non_blocking_pop(&self, handler: &mut Handler, key: &str) -> Result<Option<String>, Error> {
        let pop_frame = Frame::Array(vec![
            Frame::BulkString("RPOP".to_string()),
            Frame::BulkString(key.to_string()),
        ]);
        
        let pop_cmd = match crate::command::Command::parse_from_frame(pop_frame) {
            Ok(cmd) => cmd,
            Err(_) => return Ok(None),
        };
        
        // 非阻塞执行 RPOP
        let result = handler.apply_db_command(pop_cmd).await?;
        
        if let Frame::BulkString(value) = result {
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
}
