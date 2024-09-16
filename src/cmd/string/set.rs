use anyhow::Error;

use crate::{db::Db, frame::Frame, structure::Structure};

pub struct Set {
    key: String,
    val: String,
}

impl Set {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error>{

        let key = frame.get(1);
        let val = frame.get(2);

        if key.is_none() {
            return Err(Error::msg("Key is missing"));
        }

        if val.is_none() {
            return Err(Error::msg("Val is missing"));
        }

        let fianl_key = key.unwrap().to_string();
        let final_val = val.unwrap().to_string();

        Ok(Set { 
            key: fianl_key, 
            val: final_val 
        })
    }

    pub fn apply(self,db: &mut Db) -> Result<Frame, Error> {
        db.record.insert(self.key, Structure::String(self.val));
        Ok(Frame::Ok)
    }
}