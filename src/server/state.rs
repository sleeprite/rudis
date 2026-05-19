use std::sync::Arc;
use tokio::sync::Mutex;
use crate::server::blocking::BlockingQueueManager;

/// 全局状态容器
/// 
/// 用于持有和管理服务器的所有全局/异步状态资源，如：
///
/// - BlockingQueueManager (List BLPOP/BRPOP)
/// 
/// 设计理念：
///
/// 1. 解耦：Server 和 Handler 不需要直接持有具体的 Manager
/// 2. 扩展：添加新功能时只需修改此结构体，不破坏 Server 签名
#[derive(Clone)]
pub struct ServerState {
    /// 列表阻塞管理器 (List BLPOP/BRPOP)
    pub blocking_list: Arc<Mutex<BlockingQueueManager>>,
    // pub blocking_stream: Arc<Mutex<StreamManager>>,
    // pub pubsub: Arc<Mutex<PubSubManager>>,
}

impl ServerState {
    pub fn new() -> Self {
        let blocking_list = Arc::new(Mutex::new(BlockingQueueManager::new()));
        
        // 启动超时清理任务
        let blocking_manager_clone = blocking_list.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
            loop {
                interval.tick().await;
                let mut manager = blocking_manager_clone.lock().await;
                manager.cleanup_timeout_requests();
            }
        });

        ServerState {
            blocking_list,
        }
    }

    /// 清理会话相关的所有资源
    /// 
    /// 当客户端断开连接时调用，负责清理该会话在各个子系统中的状态
    /// - BlockingQueueManager: 清理未完成的阻塞请求
    /// - (未来) PubSubManager: 取消订阅
    /// - (未来) StreamManager: 清理消费者状态
    pub async fn cleanup_session(&self, session_id: usize) {
        
        // 1. 清理 List 阻塞请求
        {
            let mut blocking_manager = self.blocking_list.lock().await;
            blocking_manager.cleanup_session(session_id);
        }
        
        // 2. 未来：清理 Pub/Sub
        // if let Some(pubsub) = &self.pubsub {
        //     pubsub.lock().await.unsubscribe_all(session_id);
        // }
    }
}
