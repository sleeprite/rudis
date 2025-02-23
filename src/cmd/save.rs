use std::sync::Arc;

use anyhow::Error;
use tokio::sync::oneshot;
use crate::{command::Command, db::{DbGuard, DbMessage}, frame::Frame};

use super::dump::Dump;

pub struct Save {}

impl Save {

    pub fn parse_from_frame(_frame: Frame) -> Result<Self, Error> {
        Ok(Save { })
    }

    pub fn apply(self, db_guard: Arc<DbGuard>) -> Result<Frame, Error> {
        let senders = db_guard.get_senders();
        for target_sender in senders {
            let (sender, _receiver) = oneshot::channel(); // 创建通道
            let _result = target_sender.send(DbMessage {
                command: Command::Dump(Dump {}),
                sender: sender
            });
        }
        Ok(Frame::Ok)
    }
}