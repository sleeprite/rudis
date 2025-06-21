use anyhow::Error;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{store::db::Db, frame::Frame};

pub struct ExpireAt {
    key: String,
    timestamp: u64,
}

impl ExpireAt {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();

        if args.len() < 3 {
            return Err(Error::msg("ERR wrong number of arguments for 'expireat' command"));
        }

        let key = args[1].to_string();
        let timestamp = match args[2].parse::<u64>() {
            Ok(val) => val,
            Err(_) => {
                return Err(Error::msg("ERR value is not an integer or out of range"));
            }
        };
        Ok(ExpireAt { key, timestamp })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();
        let ttl = if self.timestamp > now {
            ( self.timestamp - now ) * 1000
        } else {
            0
        };
        db.expire(self.key.clone(), ttl);
        Ok(Frame::Ok)
    }
}