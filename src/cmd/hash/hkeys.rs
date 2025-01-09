use anyhow::Error;
use crate::{db::{Db, Structure}, frame::Frame};

pub struct Hkeys {
    key: String,
}

impl Hkeys {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {

        let key = frame.get_arg(1);

        if key.is_none() {
            return Err(Error::msg("ERR wrong number of arguments for 'hkeys' command"));
        }

        let final_key = key.unwrap().to_string(); // 键

        Ok(Hkeys {
            key: final_key,
        })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.get(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::Hash(hash) => {
                        let mut keys = Vec::new();
                        for key in hash.keys() {
                            keys.push(Frame::BulkString(Some(key.clone())));
                        }
                        Ok(Frame::Array(keys))
                    },
                    _ => {
                        let f = "ERR Operation against a key holding the wrong kind of value";
                        Ok(Frame::Error(f.to_string()))
                    }
                }
            }, // 当键不存在时返回空列表
            None => Ok(Frame::Array(vec![])), 
        }
    }
}