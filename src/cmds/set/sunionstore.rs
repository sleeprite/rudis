use std::collections::HashSet;

use anyhow::Error;
use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Sunionstore {
    destination: String,
    keys: Vec<String>,
}

impl Sunionstore {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();

        // 至少需要三个参数（命令名、目标集合键、一个或多个源集合键）
        if args.len() < 3 {
            return Err(Error::msg("ERR wrong number of arguments for 'sunionstore' command"));
        }

        let destination = args[1].to_string();
        let keys = args[2..].iter().map(|arg| arg.to_string()).collect();

        Ok(Sunionstore { destination, keys })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        let destination = self.destination;
        let keys = self.keys;

        // 创建一个空的 HashSet 用于存储并集结果
        let mut result_set = HashSet::new();

        for key in keys {
            if let Some(structure) = db.records.get(&key) {
                match structure {
                    Structure::Set(set) => {
                        for member in set.iter() {
                            result_set.insert(member.clone());
                        }
                    }
                    _ => {
                        let f = "ERR Operation against a key holding the wrong kind of value";
                        return Ok(Frame::Error(f.to_string()));
                    }
                }
            }
        }
        db.records.insert(destination, Structure::Set(result_set.clone()));
        Ok(Frame::Integer(result_set.len() as i64))
    }
}