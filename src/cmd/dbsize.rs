use anyhow::Error;

use crate::{db::Db, frame::Frame};

pub struct Dbsize {}

impl Dbsize {
    
    pub fn parse_from_frame(_frame: Frame) -> Result<Self, Error> {
        Ok(Dbsize {})
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        let size = db.records.len();
        Ok(Frame::Integer(size as i64))
    }
}