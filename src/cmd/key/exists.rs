use anyhow::Error;
use crate::{db::Db, frame::Frame};

pub struct Exists {
    key: String,
}

impl Exists {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let key = frame.get_arg(1);

        if key.is_none() {
            return Err(Error::msg("ERR wrong number of arguments for 'exists' command"));
        }

        let key_str = key.unwrap().to_string(); // 键

        Ok(Exists {
            key: key_str,
        })
    }

    pub fn apply(self, db: &Db) -> Result<Frame, Error> {
        if db.exists(&self.key) {
            Ok(Frame::Integer(1))
        } else {
            Ok(Frame::Integer(0))
        }
    }
}