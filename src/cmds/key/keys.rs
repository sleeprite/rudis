use anyhow::Error;
use crate::{store::db::Db, frame::Frame};

pub struct Keys {
    pattern: String,
}

impl Keys {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args_from_index(1);
        if args.len() != 1 {
            return Err(Error::msg("KEYS command requires exactly one argument"));
        }
        Ok(Keys { pattern: args[0].clone() })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        let keys = db.keys(&self.pattern);
        let results: Vec<Frame> = keys.into_iter().map(|key| Frame::BulkString(key)).collect();
        Ok(Frame::Array(results))
    }
}