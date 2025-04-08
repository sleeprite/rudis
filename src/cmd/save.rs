use std::sync::Arc;

use anyhow::Error;
use tokio::sync::oneshot;
use crate::{command::Command, db::{DbManager, DbMessage}, frame::Frame};

use super::saverdb::Saverdb;

pub struct Save {}

impl Save {

    pub fn parse_from_frame(_frame: Frame) -> Result<Self, Error> {
        Ok(Save { })
    }

    pub async fn apply(self, db_manager: Arc<DbManager>) -> Result<Frame, Error> {
        let senders = db_manager.get_senders();
        for target_sender in senders {
            let (sender, _receiver) = oneshot::channel(); // 创建通道
            match target_sender.send(DbMessage {
                command: Command::Saverdb(Saverdb { background: false }),
                sender: sender
            }).await {
                Ok(()) => {}
                Err(e) => {
                    eprintln!("Failed to write to socket; err = {:?}", e);
                }
            };
        }
        Ok(Frame::Ok)
    }
}