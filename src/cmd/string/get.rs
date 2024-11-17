use anyhow::Error;

use crate::{db::Db, frame::Frame, structure::Structure};

pub struct Get {
    key: String,
}

impl Get {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {

        let key = frame.get_arg(1);

        if key.is_none() {
            return Err(Error::msg("Key is missing"));
        }

        let fianl_key = key.unwrap().to_string();
        
        Ok(Get { 
            key: fianl_key 
        })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        let result_structure = db.get(&self.key);
        match result_structure {
            Some(structure) => {
                match structure {
                    Structure::String(value) => {
                        Ok(Frame::SimpleString(value.to_string()))
                    },
                    _ => {
                        Ok(Frame::Error("Type parsing error".to_string()))
                    }
                }
            },
            None => {
                Ok(Frame::Null)
            }
        }
    }
}