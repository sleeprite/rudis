use anyhow::Error;
use crate::{db::{Db, Structure}, frame::Frame};

pub struct Mget {
    keys: Vec<String>,
}

impl Mget {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args_from_index(1);

        Ok(Mget { keys: args })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        let mut results = Vec::new();
        for key in self.keys {
            match db.get(&key) {
                Some(Structure::String(val)) => results.push(Frame::BulkString(Some(val.to_string()))),
                Some(_) => results.push(Frame::Null), // 如果值不是字符串类型，返回 Null
                None => results.push(Frame::Null), // 如果键不存在，返回 Null
            }
        }
        Ok(Frame::Array(results))
    }
}