use anyhow::Error;

use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Lset {
    key: String,
    index: isize, // 索引，支持负数索引
    value: String, // 要设置的值
}

impl Lset {
    
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {

        let args = frame.get_args();

        if args.len() != 4 {
            return Err(Error::msg("ERR wrong number of arguments for 'lset' command"));
        }

        let key = args[1].to_string(); // 键
        let index = args[2].parse::<isize>().map_err(|_| Error::msg("ERR value is not an integer or out of range"))?; // 索引
        let value = args[3].to_string(); // 要设置的值

        Ok(Lset { key, index, value })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.get_mut(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::List(list) => {
                        if list.is_empty() {
                            Ok(Frame::Error("ERR index out of range".to_string()))
                        } else {
                            let list_len = list.len() as isize;
                            let adjusted_index = if self.index < 0 {
                                list_len + self.index
                            } else {
                                self.index
                            };

                            if adjusted_index < 0 || adjusted_index >= list_len {
                                Ok(Frame::Error("ERR index out of range".to_string()))
                            } else {
                                list[adjusted_index as usize] = self.value;
                                Ok(Frame::SimpleString("OK".to_string()))
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
                let f = "ERR no such key";
                Ok(Frame::Error(f.to_string()))
            }
        }
    }
}