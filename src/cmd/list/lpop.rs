use anyhow::Error;
use crate::{db::{Db, Structure}, frame::Frame};

pub struct Lpop {
    key: String,
}

impl Lpop {
    
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {

        let args = frame.get_args();

        if args.len() != 2 {
            return Err(Error::msg("ERR wrong number of arguments for 'lpop' command"));
        }

        let key = args[1].to_string(); // 键
        
        Ok(Lpop { key })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.get_mut(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::List(list) => {
                        if list.is_empty() {
                            Ok(Frame::Null)
                        } else {
                            let value = list.remove(0); // 移除列表的第一个元素
                            Ok(Frame::BulkString(Some(value)))
                        }
                    },
                    _ => {
                        let f = "ERR Operation against a key holding the wrong kind of value";
                        Ok(Frame::Error(f.to_string()))
                    }
                }
            },
            None => {
                Ok(Frame::Null)
            }
        }
    }
}