use tokio::sync::mpsc::Sender;

use crate::{ network::connection::Connection, store::db::DatabaseMessage };

pub struct Session {
    certification: bool,
    sender: Sender<DatabaseMessage>,
    pub connection: Connection,
    current_db: usize
}

impl Session {

    pub fn new(certification: bool, sender: Sender<DatabaseMessage>, connection: Connection) -> Self {
        Session {
            certification,
            sender,
            current_db: 0,
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

    pub fn get_sender(&self) -> &Sender<DatabaseMessage> {
        &self.sender
    }

    pub fn set_certification(&mut self, certification: bool) {
        self.certification = certification;
    }

    pub fn get_certification(&self) -> bool {
        self.certification
    }

}