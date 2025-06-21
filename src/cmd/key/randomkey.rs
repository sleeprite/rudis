use anyhow::Error;

use crate::{store::db::Db, frame::Frame};

pub struct RandomKey {

}

impl RandomKey {
    
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        if frame.get_args().len() != 1 {
            return Err(Error::msg("ERR wrong number of arguments for 'randomkey' command"));
        }
        Ok(RandomKey {})
    }

    pub fn apply(self, db: &Db) -> Result<Frame, Error> {
        if let Some(key) = db.random_key() {
            Ok(Frame::BulkString(key))
        } else {
            Ok(Frame::Null)
        }
    }
}