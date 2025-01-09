use anyhow::Error;
use crate::{db::Db, frame::Frame};

pub struct Persist {
    key: String,
}

impl Persist {
    
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let key = frame.get_arg(1);
        if key.is_none() {
            return Err(Error::msg("ERR wrong number of arguments for 'persist' command"));
        }
        let key_str = key.unwrap().to_string(); // 键
        Ok(Persist {
            key: key_str,
        })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        if  db.expire_records.contains_key(&self.key) {
            match db.expire_records.remove(&self.key) {
                Some(_) => Ok(Frame::Integer(1)),
                None => {
                    Ok(Frame::Integer(0))
                }
            }
        } else {
            Ok(Frame::Integer(0))
        }
    }
}