use anyhow::Error;
use crate::{db::{Db, Structure}, frame::Frame};

pub struct Hlen {
    key: String,
}

impl Hlen {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {

        let key = frame.get_arg(1);

        if key.is_none() {
            return Err(Error::msg("ERR wrong number of arguments for 'hlen' command"));
        }

        let final_key = key.unwrap().to_string(); // 键

        Ok(Hlen {
            key: final_key,
        })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.get(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::Hash(hash) => {
                        Ok(Frame::Integer(hash.len() as i64))
                    },
                    _ => {
                        let f = "ERR Operation against a key holding the wrong kind of value";
                        Ok(Frame::Error(f.to_string()))
                    }
                }
            },
            None => Ok(Frame::Integer(0)),
        }
    }
}