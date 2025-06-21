use std::collections::HashMap;

use anyhow::Error;
use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Hsetnx {
    key: String,
    field: String,
    value: String,
}

impl Hsetnx {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {

        let key = frame.get_arg(1);
        let field = frame.get_arg(2);
        let value = frame.get_arg(3);

        if key.is_none() || field.is_none() || value.is_none() {
            return Err(Error::msg("ERR wrong number of arguments for 'hsetnx' command"));
        }

        let final_key = key.unwrap().to_string(); 
        let final_field = field.unwrap().to_string(); 
        let final_value = value.unwrap().to_string();

        Ok(Hsetnx {
            key: final_key,
            field: final_field,
            value: final_value,
        })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.get_mut(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::Hash(hash) => {
                        if hash.contains_key(&self.field) {
                            Ok(Frame::Integer(0))
                        } else {
                            hash.insert(self.field, self.value);
                            Ok(Frame::Integer(1))
                        }
                    },
                    _ => {
                        let f = "ERR Operation against a key holding the wrong kind of value";
                        Ok(Frame::Error(f.to_string()))
                    }
                }
            },
            None => {
                let hash = HashMap::from([(self.field, self.value)]);
                db.insert(self.key.clone(), Structure::Hash(hash));
                Ok(Frame::Integer(1))
            }
        }
    }
}