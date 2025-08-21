use tokio::sync::mpsc::Sender;

use crate::store::db::DatabaseMessage;

pub struct Session {
    certification: bool,
    sender: Sender<DatabaseMessage>
}

impl Session {

    pub fn new(certification: bool, sender: Sender<DatabaseMessage>) -> Self {
        Session {
            certification,
            sender
        }
    }

    // Setter for certification with parameter
    pub fn set_certification(&mut self, certification: bool) {
        self.certification = certification;
    }

    pub fn get_certification(&self) -> bool {
        self.certification
    }

    pub fn set_sender(&mut self, sender: Sender<DatabaseMessage>) {
        self.sender = sender;
    }

    pub fn get_sender(&self) -> &Sender<DatabaseMessage> {
        &self.sender
    }

}