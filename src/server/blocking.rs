use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;
use crate::frame::Frame;

/// 阻塞方向
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockDirection {
    Left,   // BLPOP
    Right,  // BRPOP
}

/// 共享的发送端，用于多键阻塞
/// 因为一个客户端可能同时等待多个 key，但只需要被唤醒一次
type SharedSender = Arc<Mutex<Option<oneshot::Sender<Frame>>>>;

/// 阻塞请求信息
pub struct BlockingRequest {
    pub session_id: usize,
    pub key: String,
    pub direction: BlockDirection,
    pub timeout: Option<Duration>,
    pub response_sender: SharedSender,
    pub created_at: Instant,
}

/// 阻塞队列管理器
/// 
/// 管理所有等待 BLPOP/BRPOP 的客户端请求
pub struct BlockingQueueManager {
    // key -> 等待该键的请求队列（FIFO）
    waiting_requests: HashMap<String, VecDeque<BlockingRequest>>,
    // session_id -> keys 映射（用于客户端断开时快速清理）
    session_to_keys: HashMap<usize, Vec<String>>,
}

impl BlockingQueueManager {
    pub fn new() -> Self {
        Self {
            waiting_requests: HashMap::new(),
            session_to_keys: HashMap::new(),
        }
    }
    
    /// 注册阻塞请求
    /// 
    /// 返回一个 receiver，用于等待结果
    pub fn register_blocking_request(
        &mut self,
        keys: Vec<String>,
        session_id: usize,
        direction: BlockDirection,
        timeout: Option<Duration>,
    ) -> oneshot::Receiver<Frame> {
        let (sender, receiver) = oneshot::channel();
        
        // 创建共享发送端
        let shared_sender = Arc::new(Mutex::new(Some(sender)));
        let created_at = Instant::now();
        
        // 记录该会话监听的所有 key
        let mut session_keys = Vec::new();

        // 为每个 key 注册请求
        for key in keys {
            let request = BlockingRequest {
                session_id,
                key: key.clone(),
                direction,
                timeout,
                response_sender: shared_sender.clone(),
                created_at,
            };
            
            // 添加到等待队列（FIFO）
            self.waiting_requests
                .entry(key.clone())
                .or_insert_with(VecDeque::new)
                .push_back(request);
                
            session_keys.push(key);
        }
        
        // 记录 session 到 keys 的映射
        self.session_to_keys.insert(session_id, session_keys);
        
        receiver
    }
    
    /// 尝试唤醒等待的客户端
    /// 
    /// 当 LPUSH/RPUSH 执行时调用此方法
    /// 返回：
    /// - Some(Frame): 成功唤醒一个客户端，返回要发送给该客户端的结果
    /// - None: 没有等待的客户端
    pub fn try_wakeup(
        &mut self,
        key: &str,
        direction: BlockDirection,
        value: String,
    ) -> Option<(usize, Frame)> {
        // 需要在一个循环中尝试唤醒，因为可能遇到"已经失效"的请求（被其他key唤醒了）
        loop {
            let mut found_session_id = None;
            
            if let Some(requests) = self.waiting_requests.get_mut(key) {
                // 查找匹配方向的第一个请求（FIFO）
                let mut found_index = None;
                for (i, req) in requests.iter().enumerate() {
                    if req.direction == direction {
                        found_index = Some(i);
                        break;
                    }
                }
                
                if let Some(index) = found_index {
                    let request = requests.remove(index).unwrap();
                    
                    // 尝试获取发送锁
                    // 如果能 take 到 sender，说明我们是第一个唤醒它的
                    let mut sender_guard = request.response_sender.lock().unwrap();
                    if let Some(sender) = sender_guard.take() {
                        // 构建响应：Array[key, value]
                        let response = Frame::Array(vec![
                            Frame::BulkString(key.to_string()),
                            Frame::BulkString(value),
                        ]);
                        
                        // 发送结果给等待的客户端
                        let _ = sender.send(response.clone());
                        
                        found_session_id = Some((request.session_id, response));
                        
                        // 如果队列为空，移除该键
                        if requests.is_empty() {
                            self.waiting_requests.remove(key);
                        }
                    } else {
                        // 这个请求已经被其他 key 唤醒了（sender 为 None）
                        // 它是无效的，我们已经把它移除了，继续循环尝试下一个
                        if requests.is_empty() {
                            self.waiting_requests.remove(key);
                        }
                        continue;
                    }
                }
            }
            
            // 如果成功唤醒了一个客户端
            if let Some((session_id, response)) = found_session_id {
                // 清理该会话在其他 key 上的等待记录
                self.cleanup_session(session_id);
                return Some((session_id, response));
            } else {
                // 没有找到任何有效请求
                return None;
            }
        }
    }
    
    /// 检查是否有等待该键的客户端
    pub fn has_waiting(&self, key: &str, direction: BlockDirection) -> bool {
        if let Some(requests) = self.waiting_requests.get(key) {
            requests.iter().any(|req| req.direction == direction)
        } else {
            false
        }
    }
    
    /// 清理客户端的所有阻塞请求（客户端断开时调用，或被唤醒后调用）
    pub fn cleanup_session(&mut self, session_id: usize) {
        if let Some(keys) = self.session_to_keys.remove(&session_id) {
            for key in keys {
                if let Some(requests) = self.waiting_requests.get_mut(&key) {
                    // 收集需要清理的请求
                    let mut indices_to_remove = Vec::new();
                    for (i, req) in requests.iter().enumerate() {
                        if req.session_id == session_id {
                            indices_to_remove.push(i);
                        }
                    }
                    
                    // 从后往前移除，避免索引变化
                    for &i in indices_to_remove.iter().rev() {
                        if let Some(req) = requests.remove(i) {
                            // 尝试通知（如果是断开连接导致的清理）
                            // 如果已经被唤醒（sender 为 None），这里什么都不做
                            let mut sender_guard = req.response_sender.lock().unwrap();
                            if let Some(sender) = sender_guard.take() {
                                let _ = sender.send(Frame::Null);
                            }
                        }
                    }
                    
                    if requests.is_empty() {
                        self.waiting_requests.remove(&key);
                    }
                }
            }
        }
    }
    
    /// 清理超时的请求
    /// 
    /// 这个方法应该定期调用（例如每秒一次）
    pub fn cleanup_timeout_requests(&mut self) {
        let now = Instant::now();
        let mut timeout_sessions = Vec::new();
        
        // 1. 扫描所有请求，找出超时的 session_id
        // 注意：这里只做标记，不做修改，避免复杂的借用问题
        for (_, requests) in &self.waiting_requests {
            for req in requests.iter() {
                if let Some(timeout) = req.timeout {
                    if now.duration_since(req.created_at) >= timeout {
                        timeout_sessions.push(req.session_id);
                    }
                }
            }
        }
        
        // 去重
        timeout_sessions.sort_unstable();
        timeout_sessions.dedup();
        
        // 2. 统一清理这些超时的 session
        for session_id in timeout_sessions {
            self.cleanup_session(session_id);
        }
    }
}
