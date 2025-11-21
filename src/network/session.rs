use std::sync::atomic::{AtomicUsize, Ordering};

use tokio::sync::mpsc::Sender;
use crate::{network::connection::Connection, store::db::DatabaseMessage};

static SESSION_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone)]
pub struct Session {
    id: usize,
    certification: bool,
    sender: Sender<DatabaseMessage>,
    pub connection: Connection,
    current_db: usize,
}

impl Session {
    pub fn new(certification: bool, sender: Sender<DatabaseMessage>, connection: Connection) -> Self {
        let id = SESSION_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let current_db = 0;
        Session {
            id,
            certification,
            sender,
            current_db,
            connection
        }
    }
    
    pub fn set_current_db(&mut self, current_db: usize) {
        self.current_db = current_db;
    }

    pub fn get_current_db(&self) -> usize {
        self.current_db
    }

    pub fn set_sender(&mut self, sender: Sender<DatabaseMessage>) {
        self.sender = sender;
    }

    pub fn get_sender(&self) -> Sender<DatabaseMessage> {
        self.sender.clone()
    }

    pub fn set_certification(&mut self, certification: bool) {
        self.certification = certification;
    }

    pub fn get_certification(&self) -> bool {
        self.certification
    }

    // 新增方法：获取 session ID
    pub fn get_id(&self) -> usize {
        self.id
    }
}