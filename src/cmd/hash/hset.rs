use std::collections::HashMap;

use anyhow::Error;
use crate::{db::{Db, Structure}, frame::Frame};

pub struct Hset {
    key: String,
    field: String,
    value: String,
}

impl Hset {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {

        let key = frame.get_arg(1);
        let field = frame.get_arg(2);
        let value = frame.get_arg(3);

        if key.is_none() || field.is_none() || value.is_none() {
            return Err(Error::msg("ERR wrong number of arguments for 'hset' command"));
        }

        let final_key = key.unwrap().to_string(); 
        let final_field = field.unwrap().to_string(); 
        let final_value = value.unwrap().to_string();

        Ok(Hset {
            key: final_key,
            field: final_field,
            value: final_value,
        })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.get(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::Hash(hash) => {
                        let field_exists  = hash.contains_key(&self.field);
                        let mut new_hash = hash.clone();
                        new_hash.insert(self.field, self.value);
                        db.insert(self.key, Structure::Hash(new_hash));
                        if field_exists {
                            return Ok(Frame::Integer(0));
                        } else {
                            return Ok(Frame::Integer(1));
                        }
                    },
                    _ => {
                        let f = "ERR Operation against a key holding the wrong kind of value";
                        Ok(Frame::Error(f.to_string()))
                    }
                }
            },
            None => {
                let mut hash = HashMap::new();
                hash.insert(self.field, self.value);
                db.insert(self.key.clone(), Structure::Hash(hash));
                Ok(Frame::Integer(1))
            }
        }
    }
}