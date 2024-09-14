use anyhow::Error;

use crate::{db::Db, frame::Frame};

pub struct Del {
    key: String,
}

impl Del {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let key = "username".to_string();
        Ok(Del { key }) 
    }

    pub fn apply(self, db: &Db) -> Result<Frame, Error> {
        Ok(Frame::Ok)
    }
}