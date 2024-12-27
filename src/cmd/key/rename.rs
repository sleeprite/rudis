use anyhow::Error;
use crate::{db::Db, frame::Frame};

pub struct Rename {
    old_key: String,
    new_key: String,
}

impl Rename {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {

        let old_key = frame.get_arg(1);
        let new_key = frame.get_arg(2);

        if old_key.is_none() || new_key.is_none() {
            return Err(Error::msg("ERR wrong number of arguments for 'rename' command"));
        }

        let old_key_str = old_key.unwrap().to_string(); // 旧键
        let new_key_str = new_key.unwrap().to_string(); // 新键

        Ok(Rename {
            old_key: old_key_str,
            new_key: new_key_str,
        })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {

        if !db.exists(&self.old_key) {
            return Err(Error::msg("ERR no such key"));
        }

        if db.exists(&self.new_key) {
            return Err(Error::msg("ERR key already exists"));
        }
        
        if let Some(value) = db.remove(&self.old_key) {
            db.insert(self.new_key.clone(), value); 
        }

        Ok(Frame::Ok)
    }
}