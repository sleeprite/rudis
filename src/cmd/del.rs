use anyhow::Error;

use crate::{db::Db, frame::Frame};

pub struct Del {
    key: String,
}

impl Del {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let key = frame.get(1);

        if key.is_none() {
            return Err(Error::msg("Key is missing"));
        }

        let fianl_key = key.unwrap().to_string();
        
        Ok(Del { 
            key: fianl_key 
        })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        db.remove(&self.key);
        Ok(Frame::Ok)
    }
}