use anyhow::Error;

use crate::{db::Db, frame::Frame};

pub struct Dump {
    pub background: bool
}

impl Dump {
    
    pub fn parse_from_frame(_frame: Frame) -> Result<Self, Error> {
        Ok(Dump {
            background: false
        })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        if self.background { db.bg_save_rdb_file(); } 
        else {
            db.save_rdb_file();
        }
        Ok(Frame::Ok)
    }
}