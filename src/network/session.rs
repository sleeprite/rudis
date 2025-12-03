use std::sync::atomic::{AtomicUsize, Ordering};

use tokio::sync::mpsc::Sender;
use crate::{frame::Frame, network::{connection::Connection, session_role::SessionRole}, store::db::DatabaseMessage};

static SESSION_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone)]
pub struct Session {
    id: usize,
    certification: bool,
    sender: Sender<DatabaseMessage>,
    pub connection: Connection,
    current_db: usize,
    role: SessionRole,
    // 事务相关字段
    in_transaction: bool,
    transaction_frames: Vec<Frame>
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
            connection,
            role: SessionRole::Other,
            in_transaction: false,
            transaction_frames: Vec::new()
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

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn set_role(&mut self, role: SessionRole) {
        self.role = role;
    }

    pub fn get_role(&self) -> &SessionRole {
        &self.role
    }

    // 事务相关方法
    pub fn start_transaction(&mut self) {
        self.in_transaction = true;
        self.transaction_frames.clear();
    }

    pub fn is_in_transaction(&self) -> bool {
        self.in_transaction
    }

    pub fn add_transaction_frame(&mut self, frame: Frame) {
        self.transaction_frames.push(frame);
    }

    pub fn get_transaction_frames(&self) -> &Vec<Frame> {
        &self.transaction_frames
    }

    pub fn clear_transaction(&mut self) {
        self.in_transaction = false;
        self.transaction_frames.clear();
    }

    pub fn get_transaction_frames_mut(&mut self) -> &mut Vec<Frame> {
        &mut self.transaction_frames
    }
}