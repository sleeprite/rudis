use anyhow::Error;

use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Rpop {
    key: String,
}

impl Rpop {
    
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {

        let args = frame.get_args();

        if args.len() != 2 {
            return Err(Error::msg("ERR wrong number of arguments for 'rpop' command"));
        }

        let key = args[1].to_string(); // 键

        Ok(Rpop { key })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.get_mut(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::List(list) => {
                        if list.is_empty() {
                            Ok(Frame::Null)
                        } else {
                            let value = list.pop(); // 移除列表的最后一个元素
                            match value {
                                Some(val) => Ok(Frame::BulkString(val)),
                                None => Ok(Frame::Null), // 理论上不会执行到这，因为前面已经判断过列表不为空
                            }
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