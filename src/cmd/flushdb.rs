use anyhow::Error;

use crate::{db::Db, frame::Frame};

pub struct Flushdb {}

impl Flushdb {

    pub fn parse_from_frame(_frame: Frame) -> Result<Self, Error> {      
        Ok(Flushdb {})
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        db.clear(); // 清理数据库
        Ok(Frame::Ok)
    }
}