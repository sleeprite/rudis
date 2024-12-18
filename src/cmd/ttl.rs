use anyhow::Error;

use crate::{db::Db, frame::Frame};

pub struct Ttl {
    key: String,
}

impl Ttl {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {      

        let key = frame.get_arg(1);

        if key.is_none() {
            return Err(Error::msg("ERR wrong number of arguments for 'get' command"));
        }

        let fianl_key = key.unwrap().to_string();
        

        Ok(Ttl {
            key: fianl_key
        })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        let millis = db.ttl_millis(&self.key);
        Ok(Frame::Integer(millis))
    }
}