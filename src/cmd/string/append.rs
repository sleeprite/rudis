use anyhow::Error;
use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Append {
    key: String,
    val: String,
}

impl Append {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {

        let key = frame.get_arg(1);
        let val = frame.get_arg(2);

        if key.is_none() || val.is_none() {
            return Err(Error::msg("ERR wrong number of arguments for 'append' command"));
        }

        let key_str = key.unwrap().to_string(); // 键
        let val_str = val.unwrap().to_string(); // 值

        Ok(Append {
            key: key_str,
            val: val_str,
        })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        let empty_value = String::new(); 
        let existing_value = match db.get(&self.key) {
            Some(Structure::String(s)) => s,
            Some(_) => return Err(Error::msg("ERR wrong type for 'append' command")),
            None => &empty_value,
        };
        let new_value = format!("{}{}", existing_value, self.val);
        db.insert(self.key, Structure::String(new_value));
        Ok(Frame::Ok)
    }
}