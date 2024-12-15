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
    args: Arc<Args>
}

impl SessionManager {

    pub fn new(args: Arc<Args>) -> Self {
        SessionManager {
            sessions: RwLock::new(HashMap::new()),
            args
        }
    }

    pub fn destroy(&self, session_id: &str) {
        let mut sessions = self.sessions.write().unwrap();
        sessions.remove(session_id);
    }

    pub fn register(&self, address: SocketAddr) {
        let session_id = address.to_string();
        let mut sessions = self.sessions.write().unwrap();
        let mut session = Session::new(address);
        session.set_authenticated(self.args.requirepass.is_none());
        sessions.insert(session_id, session);
    }

    pub fn get(&self, session_id: &str) -> Option<Session> {
        let sessions = self.sessions.read().unwrap();
        sessions.get(session_id).cloned()
    }

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
}