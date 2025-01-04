use anyhow::Error;

use crate::{db::Db, frame::Frame};

pub struct Dbsize {}

impl Dbsize {
    // 解析命令
    pub fn parse_from_frame(_frame: Frame) -> Result<Self, Error> {
        Ok(Dbsize {})
    }

    // 应用命令
    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        let size = db.records.len();
        Ok(Frame::Integer(size as i64))
    }
}