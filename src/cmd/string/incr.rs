use anyhow::Error;
use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Incr {
    key: String,
}

impl Incr {
    
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        if args.len() != 2 {
            return Err(Error::msg("ERR wrong number of arguments for 'incr' command"));
        }
        let key = args[1].to_string(); // 键
        Ok(Incr { key })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.get_mut(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::String(str) => {
                        match str.parse::<i64>() {
                            Ok(mut num) => {
                                num += 1;
                                *str = num.to_string();
                                Ok(Frame::Integer(num))
                            },
                            Err(_) => {
                                let f = "ERR value is not an integer or out of range";
                                Ok(Frame::Error(f.to_string()))
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
                db.insert(self.key.clone(), Structure::String("1".to_string()));
                Ok(Frame::Integer(1))
            }
        }
    }
}