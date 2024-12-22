use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};

use crate::args::Args;

#[derive(Clone)]
pub struct Session {
    authenticated: bool,
    address: SocketAddr,
    db: usize,
}

impl Session {
    pub fn new(address: SocketAddr) -> Self {
        Session {
            authenticated: true,
            address,
            db: 0,
        }
    }

    pub fn is_authenticated(&self) -> bool {
        self.authenticated
    }

    pub fn set_authenticated(&mut self, authenticated: bool) {
        self.authenticated = authenticated;
    }

    pub fn address(&self) -> &SocketAddr {
        &self.address
    }

    pub fn set_address(&mut self, address: SocketAddr) {
        self.address = address;
    }

    pub fn db(&self) -> usize {
        self.db
    }

    pub fn set_db(&mut self, db: usize) {
        self.db = db;
    }
}

pub struct SessionManager {
    sessions: RwLock<HashMap<String, Session>>,
    args: Arc<Args>,
}

impl SessionManager {
    /**
     * 创建会话管理器
     *
     * @param args 启动参数
     */
    pub fn new(args: Arc<Args>) -> Self {
        SessionManager {
            sessions: RwLock::new(HashMap::new()),
            args,
        }
    }

    /**
     * 销毁会话
     *
     * @param session_id 会话编号
     */
    pub fn destroy(&self, session_id: &str) {
        let mut sessions = self.sessions.write().unwrap();
        sessions.remove(session_id);
    }

    /**
     * 注册会话
     */
    pub fn register(&self, address: SocketAddr) {
        let session_id = address.to_string();
        let mut sessions = self.sessions.write().unwrap();
        let mut session = Session::new(address);
        session.set_authenticated(self.args.requirepass.is_none());
        sessions.insert(session_id, session);
    }

    /**
     * 查询会话
     *
     * @param session_id 会话编号
     */
    pub fn get(&self, session_id: &str) -> Option<Session> {
        let sessions = self.sessions.read().unwrap();
        sessions.get(session_id).cloned()
    }

    /**
     * 修改会话
     *
     * @param session_id 编号
     * @param authenticated 是否认证
     * @param db 数据库索引
     */
    pub fn set(&self, session_id: &str, authenticated: Option<bool>, db: Option<usize>) {
        let mut sessions = self.sessions.write().unwrap();
        if let Some(session) = sessions.get_mut(session_id) {
            if let Some(a) = authenticated {
                session.set_authenticated(a);
            }
            if let Some(d) = db {
                session.set_db(d);
            }
        }
    }

    /**
     * 登录逻辑
     *
     * 检查输入的密码 `input_password` 是否与启动参数 `args` 中的 `requirepass` 一致。
     * 如果一致或者没有设置 `requirepass`，则将对应会话编号 `session_id` 的会话认证状态设置为 `true`。
     * 返回密码验证的结果，如果密码正确或无需密码，则返回 `true`；否则返回 `false`。
     *
     * @param session_id     会话编号
     * @param input_password 输入的密码
     * @return bool          密码验证是否成功
     */
    pub fn login(&self, session_id: &str, input_password: &str) -> bool {
        if let Some(ref requirepass) = self.args.requirepass {
            if requirepass == input_password {
                if let Some(session) = self.sessions.write().unwrap().get_mut(session_id) {
                    session.set_authenticated(true);
                }
                return true;
            }
            return false;
        } else {
            true
        }
    }

    /**
     * 是否登录
     * 
     * @param session_id 会话编号
     */
    pub fn is_login(&self, session_id: &str) -> bool {
        if self.args.requirepass.is_none() {
            true
        } else {
            let sessions = self.sessions.read().unwrap();
            if let Some(session) = sessions.get(session_id) {
                session.is_authenticated()
            } else {
                false
            }
        }
    }
    
}
