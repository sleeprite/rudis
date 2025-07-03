use anyhow::Error;
use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Lindex {
    key: String,
    index: i64,
}

impl Lindex {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {

        let key = frame.get_arg(1);
        let index = frame.get_arg(2);

        if key.is_none() || index.is_none() {
            return Err(Error::msg("ERR wrong number of arguments for 'lindex' command"));
        }

        let final_key = key.unwrap().to_string(); // 键
        let final_index = index.unwrap().parse::<i64>().map_err(|_| Error::msg("ERR value is not an integer or out of range"))?;

        Ok(Lindex {
            key: final_key,
            index: final_index,
        })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.get(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::List(list) => {
                        if self.index < 0 {
                            let index = list.len() as i64 + self.index;
                            if index < 0 || index as usize >= list.len() {
                                Ok(Frame::Null)
                            } else {
                                Ok(Frame::BulkString(list[index as usize].clone()))
                            }
                        } else if self.index as usize >= list.len() {
                            Ok(Frame::Null)
                        } else {
                            Ok(Frame::BulkString(list[self.index as usize].clone()))
                        }
                    },
                    _ => {
                        let f = "ERR Operation against a key holding the wrong kind of value";
                        Ok(Frame::Error(f.to_string()))
                    }
                }
            },
            None => Ok(Frame::Null),
        }
    }
}