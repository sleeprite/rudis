use anyhow::Error;
use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Hvals {
    key: String,
}

impl Hvals {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {

        let key = frame.get_arg(1);

        if key.is_none() {
            return Err(Error::msg("ERR wrong number of arguments for 'hvals' command"));
        }

        let final_key = key.unwrap().to_string(); // 键

        Ok(Hvals {
            key: final_key,
        })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.get(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::Hash(hash) => {
                        let mut vals = Vec::new();
                        for val in hash.values() {
                            vals.push(Frame::BulkString(val.clone()));
                        }
                        Ok(Frame::Array(vals))
                    },
                    _ => {
                        let f = "ERR Operation against a key holding the wrong kind of value";
                        Ok(Frame::Error(f.to_string()))
                    }
                }
            },
            None => Ok(Frame::Array(vec![])), 
        }
    }
}