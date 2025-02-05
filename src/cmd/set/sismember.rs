use anyhow::Error;
use crate::{db::{Db, Structure}, frame::Frame};

pub struct Sismember {
    key: String,
    member: String,
}

impl Sismember {
    
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        if args.len() != 3 {
            return Err(Error::msg("ERR wrong number of arguments for 'sismember' command"));
        }
        let key = args[1].to_string(); // 键
        let member = args[2].to_string(); // 成员
        Ok(Sismember { key, member })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.records.get(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::Set(set) => {
                        // 判断成员是否存在于集合中
                        let is_member = set.contains(&self.member);
                        Ok(Frame::Integer(is_member as i64)) // 如果存在返回 1，否则返回 0
                    },
                    _ => {
                        let f = "ERR Operation against a key holding the wrong kind of value";
                        Ok(Frame::Error(f.to_string()))
                    }
                }
            },
            None => {
                // 如果键不存在，返回 0
                Ok(Frame::Integer(0))
            }
        }
    }
}