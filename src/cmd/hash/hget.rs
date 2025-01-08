use anyhow::Error;
use crate::{db::{Db, Structure}, frame::Frame};

pub struct Hget {
    key: String,
    field: String,
}

impl Hget {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {

        let key = frame.get_arg(1);
        let field = frame.get_arg(2);

        if key.is_none() || field.is_none() {
            return Err(Error::msg("ERR wrong number of arguments for 'hget' command"));
        }

        let final_key = key.unwrap().to_string(); // 键
        let final_field = field.unwrap().to_string(); // 字段

        Ok(Hget {
            key: final_key,
            field: final_field,
        })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.get(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::Hash(hash) => {
                        match hash.get(&self.field) {
                            Some(value) => Ok(Frame::BulkString(Some(value.clone()))),
                            None => Ok(Frame::Null),
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