use anyhow::Error;

use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Zcard {
    key: String,
}

impl Zcard {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        if args.len() != 2 {
            return Err(Error::msg("ERR wrong number of arguments for 'zcard' command"));
        }
        let key = args[1].to_string(); // 键
        Ok(Zcard { key })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.records.get(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::SortedSet(set) => {
                        // 返回有序集合的元素数量
                        Ok(Frame::Integer(set.len() as i64))
                    }
                    _ => {
                        // 如果键的类型不是有序集合，返回错误
                        let f = "ERR Operation against a key holding the wrong kind of value";
                        Ok(Frame::Error(f.to_string()))
                    }
                }
            }
            None => {
                // 如果键不存在，返回 0
                Ok(Frame::Integer(0))
            }
        }
    }
}