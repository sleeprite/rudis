use anyhow::Error;

use crate::{db::Db, frame::Frame};

pub struct Pexpire {
    key: String,
    ttl: u64
}

impl Pexpire {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {

        let args = frame.get_args(); 

        if args.len() < 3 {
            return Err(Error::msg("ERR wrong number of arguments for 'pexpire' command"));
        }

        let key = args[1].to_string();

        let ttl = match args[2].parse::<u64>() {
            Ok(val) => val, // 毫秒
            Err(_) => {
                return Err(Error::msg("ERR value is not an integer or out of range"));
            }
        };

        Ok(Pexpire { 
            key: key, 
            ttl: ttl 
        })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        db.expire(self.key.clone(), self.ttl);
        Ok(Frame::Ok)
    }
}