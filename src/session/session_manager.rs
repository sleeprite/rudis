use std::sync::Arc;

use ahash::AHashMap;
use parking_lot::Mutex;

use crate::db::db_config::RudisConfig;

use super::session::Session;

pub struct SessionManager {
    sessions: Arc<Mutex<AHashMap<String, Session>>>,
    config: Arc<RudisConfig>,
}

impl SessionManager {

    // 构造函数，初始化 SessionManager
    pub fn new(config: Arc<RudisConfig>) -> Self {
        SessionManager {
            sessions: Arc::new(Mutex::new(AHashMap::new())),
            config,
        }
    }

    // 创建会话的方法
    pub fn create_session(&self, session_id: String) -> bool {
        let mut sessions_ref = self.sessions.lock();
        if self.config.maxclients == 0 || sessions_ref.len() < self.config.maxclients {
            sessions_ref.insert(session_id, Session::new());
            true
        } else {
            false
        }
    }

    // 安全认证的方法
    pub fn authenticate(&self, session_id: &str, command: &str) -> bool {
        let sessions_ref = self.sessions.lock();
        let session = sessions_ref.get(session_id).unwrap();
        let is_not_auth_command = command.to_uppercase() != "AUTH";
        let is_not_auth = !session.get_authenticated();

        if self.config.password.is_some() && is_not_auth && is_not_auth_command {
            false
        } else {
            true
        }
    }

    /*
     * 销毁会话
     *
     * @param session_id 会话编号
     */
    pub fn destroy_session(&self, session_id: &str) {
        let mut sessions_ref = self.sessions.lock();
        sessions_ref.remove(session_id);
    }

    // 返回一个会话的引用
    pub fn get_sessions(&self) -> Arc<Mutex<AHashMap<String, Session>>> {
        Arc::clone(&self.sessions)
    }
}