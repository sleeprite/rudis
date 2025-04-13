use std::sync::Arc;

use anyhow::Error;
use tokio::sync::oneshot;
use crate::{args::Args, db::{DatabaseManager, DatabaseMessage}, frame::Frame, persistence::rdb_file::RdbFile};

pub struct Save {}

impl Save {

    pub fn parse_from_frame(_frame: Frame) -> Result<Self, Error> {
        Ok(Save { })
    }

    pub async fn apply(self, db_manager: Arc<DatabaseManager>, args: Arc<Args>) -> Result<Frame, Error> {
        let rdb_file_path = args.dbfilename.clone();
        let mut rdb_file = RdbFile::new(rdb_file_path);
        let senders = db_manager.get_senders();
        for (index, target_sender) in senders.iter().enumerate() {
            let (sender, receiver) = oneshot::channel();
            match target_sender.send(DatabaseMessage::SnapshotRequest(sender)).await {
                Ok(()) => {}
                Err(e) => {
                    eprintln!("Failed to write to socket; err = {:?}", e);
                }
            };
            match receiver.await {
                Ok(snapshot) => {
                    rdb_file.set_database(index, snapshot);
                },
                Err(_) => {}
            };

        }
        let _ = rdb_file.save();
        Ok(Frame::Ok)
    }
}