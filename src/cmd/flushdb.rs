use anyhow::Error;

use crate::{store::db::Db, frame::Frame};

pub struct Flushdb {}

impl Flushdb {

    pub fn new() -> Flushdb {
        Flushdb { }
    }

    pub fn parse_from_frame(_frame: Frame) -> Result<Self, Error> {      
        Ok(Flushdb {})
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        db.expire_records.clear();
        db.records.clear();
        Ok(Frame::Ok)
    }
}