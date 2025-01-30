use std::sync::Arc;

use anyhow::Error;
use tokio::sync::oneshot;
use crate::{command::Command, db::{DbManager, DbMessage}, frame::Frame};

use super::flushdb::Flushdb;

pub struct Flushall {}

impl Flushall {

    pub fn parse_from_frame(_frame: Frame) -> Result<Self, Error> {
        Ok(Flushall { })
    }

    pub fn apply(self, db_manager: Arc<DbManager>) -> Result<Frame, Error> {
        let senders = db_manager.get_senders();
        for target_sender in senders {
            let (sender, _receiver) = oneshot::channel(); // 创建通道
            let _result = target_sender.send(DbMessage {
                command: Command::Flushdb(Flushdb {}),
                sender: sender
            });
        }
        Ok(Frame::Ok)
    }
}