use std::{sync::atomic::{AtomicUsize, Ordering}};

use dashmap::DashMap;

use crate::network::session::Session;

/// 高性能会话管理器
pub struct SessionManager {
    sessions: DashMap<usize, Session>,
    next_id: AtomicUsize
}

impl SessionManager {

    // 创建实例
    pub fn new() -> Self {
        Self {
            sessions: DashMap::new(),
            next_id: AtomicUsize::new(1)
        }
    }

    /// 添加会话
    pub fn create_session(&self, session: Session)  {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        self.sessions.insert(id, session);
    }

    /// 移除会话
    pub fn remove_session(&self, session_id: usize) -> bool {
        self.sessions.remove(&session_id).is_some()
    }
}