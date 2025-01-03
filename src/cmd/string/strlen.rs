use anyhow::Error;
use crate::{db::{Db, Structure}, frame::Frame};

pub struct Strlen {
    key: String,
}

impl Strlen {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let key = frame.get_arg(1);

        if key.is_none() {
            return Err(Error::msg("ERR wrong number of arguments for 'strlen' command"));
        }

        let final_key = key.unwrap().to_string();

        Ok(Strlen {
            key: final_key,
        })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        let result_structure = db.get(&self.key);
        match result_structure {
            Some(structure) => {
                match structure {
                    Structure::String(value) => {
                        Ok(Frame::Integer(value.len() as i64))
                    },
                    _ => {
                        Ok(Frame::Error("Type parsing error".to_string()))
                    }
                }
            },
            None => {
                Ok(Frame::Integer(0))
            }
        }
    }
}