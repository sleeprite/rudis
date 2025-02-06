use anyhow::Error;
use crate::{db::{Db, Structure}, frame::Frame};

pub struct Incrby {
    key: String,
    increment: i64,
}

impl Incrby {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        if args.len() != 3 {
            return Err(Error::msg("ERR wrong number of arguments for 'incrby' command"));
        }
        let key = args[1].to_string(); // 键
        let increment = args[2].parse::<i64>().map_err(|_| {
            Error::msg("ERR value is not an integer or out of range")
        })?;
        Ok(Incrby { key, increment })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.get_mut(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::String(str) => {
                        match str.parse::<i64>() {
                            Ok(mut num) => {
                                num += self.increment;
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
                let new_value = self.increment;
                db.insert(self.key.clone(), Structure::String(new_value.to_string()));
                Ok(Frame::Integer(new_value))
            }
        }
    }
}