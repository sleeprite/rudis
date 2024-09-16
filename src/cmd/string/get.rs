use anyhow::Error;

use crate::{db::Db, frame::Frame};

pub struct Get {
    key: String,
}

impl Get {
    
    pub fn parse_from_frame(_frame: Frame) -> Result<Self, Error> {
        let key = "username".to_string();
        Ok(Get { key })
    }

    pub fn apply(self, db: &Db) -> Result<Frame, Error> {
        Ok(Frame::Ok)
    }
}