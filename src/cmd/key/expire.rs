use anyhow::Error;

use crate::{store::db::Db, frame::Frame};

pub struct Expire {
    key: String,
    ttl: u64
}

impl Expire {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {

        let args = frame.get_args(); 

        if args.len() < 3 {
            return Err(Error::msg("ERR wrong number of arguments for 'expire' command"));
        }

        let key = args[1].to_string();

        let ttl = match args[2].parse::<u64>() {
            Ok(val) => val * 1000, // 秒 -> 毫秒
            Err(_) => {
                return Err(Error::msg("ERR value is not an integer or out of range"));
            }
        };

        Ok(Expire { 
            key: key, 
            ttl: ttl 
        })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        db.expire(self.key.clone(), self.ttl);
        Ok(Frame::Ok)
    }
}