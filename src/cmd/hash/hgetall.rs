use anyhow::Error;
use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Hgetall {
    key: String,
}

impl Hgetall {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {

        let key = frame.get_arg(1);

        if key.is_none() {
            return Err(Error::msg("ERR wrong number of arguments for 'hgetall' command"));
        }

        let final_key = key.unwrap().to_string(); // 键

        Ok(Hgetall {
            key: final_key,
        })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.get(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::Hash(hash) => {
                        let mut result = Vec::new();
                        for (field, value) in hash.iter() {
                            result.push(Frame::BulkString(field.clone()));
                            result.push(Frame::BulkString(value.clone()));
                        }
                        Ok(Frame::Array(result))
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