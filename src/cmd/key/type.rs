use anyhow::Error;

use crate::{db::{Db, Structure}, frame::Frame};

pub struct Type {
    key: String,
}

impl Type {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let key = frame.get_arg(1);
        if key.is_none() {
            return Err(Error::msg("ERR wrong number of arguments for 'type' command"));
        }
        let final_key = key.unwrap().to_string();
        Ok(Type { 
            key: final_key 
        })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.get(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::Set(_) => {
                        Ok(Frame::SimpleString("set".to_string()))
                    },
                    Structure::String(_) => {
                        Ok(Frame::SimpleString("string".to_string()))
                    },
                    Structure::SortedSet(_) => {
                        Ok(Frame::SimpleString("zset".to_string()))
                    },
                    Structure::Hash(_) => {
                        Ok(Frame::SimpleString("hash".to_string()))
                    },
                    Structure::List(_) => {
                        Ok(Frame::SimpleString("list".to_string()))
                    }
                }
            },
            None => {
                Ok(Frame::SimpleString("none".to_string()))
            }
        }
    }
}