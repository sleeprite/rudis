use anyhow::Error;
use crate::{db::{Db, Structure}, frame::Frame};

pub struct Lpush {
    key: String,
    values: Vec<String>,
}

impl Lpush {
    
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        if args.len() < 3 {
            return Err(Error::msg("ERR wrong number of arguments for 'lpush' command"));
        }

        let key = args[1].to_string(); // 键
        let values: Vec<String> = args.iter().skip(2).map(|v| v.to_string()).collect(); // 值

        Ok(Lpush { key, values })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        // todo
        Ok(Frame::Integer(0))
    }
}