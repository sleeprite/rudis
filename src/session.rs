use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Clone)]
pub struct Session {
    authenticated: bool,
    address: String,
    db: usize,
}

impl Session {

    pub fn new(authenticated: bool, address: String, db: usize) -> Self {
        Session {
            authenticated,
            address,
            db,
        }
    }

    pub fn is_authenticated(&self) -> bool {
        self.authenticated
    }

    pub fn set_authenticated(&mut self, authenticated: bool) {
        self.authenticated = authenticated;
    }

    pub fn address(&self) -> &String {
        &self.address
    }

    pub fn set_address(&mut self, address: String) {
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
}

impl SessionManager {

    pub fn new() -> Self {
        SessionManager {
            sessions: RwLock::new(HashMap::new()),
        }
    }

    pub fn register(&self, session_id: String, session: Session) {
        let mut sessions = self.sessions.write().unwrap();
        sessions.insert(session_id, session);
    }

    pub fn get(&self, session_id: &str) -> Option<Session> {
        let sessions = self.sessions.read().unwrap();
        sessions.get(session_id).cloned()
    }
}