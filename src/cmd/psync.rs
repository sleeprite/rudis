use std::sync::Arc;

use anyhow::Error;
use crate::{args::Args, db::DatabaseManager, frame::Frame};

pub struct Psync {}

impl Psync {

    pub fn parse_from_frame(_frame: Frame) -> Result<Self, Error> {
        Ok(Psync { })
    }

    pub async fn apply(self, _db_manager: Arc<DatabaseManager>, _args: Arc<Args>) -> Result<Frame, Error> {
        Ok(Frame::Ok)
    }
}