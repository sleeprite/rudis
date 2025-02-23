use anyhow::Error;

use crate::{db::Db, frame::Frame};

pub struct Dump {}

impl Dump {
    
    pub fn parse_from_frame(_frame: Frame) -> Result<Self, Error> {
        Ok(Dump {})
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        db.save_rdb_file();
        Ok(Frame::Ok)
    }
}