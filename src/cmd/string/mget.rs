use anyhow::Error;
use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Mget {
    keys: Vec<String>,
}

impl Mget {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args_from_index(1);

        Ok(Mget { keys: args })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        let mut result = Vec::new();
        for key in self.keys {
            match db.get(&key) {
                Some(structure) => {
                    match structure {
                        Structure::String(str) => result.push(Frame::BulkString(str.to_string())),
                        _ => result.push(Frame::Null),
                    }   
                } 
                None => result.push(Frame::Null),
            }
        }
        Ok(Frame::Array(result))
    }
}