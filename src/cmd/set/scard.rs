use anyhow::Error;
use crate::{db::{Db, Structure}, frame::Frame};

pub struct Scard {
    key: String,
}

impl Scard {
    
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {

        let args = frame.get_args();

        if args.len() != 2 {
            return Err(Error::msg("ERR wrong number of arguments for 'scard' command"));
        }

        let key = args[1].to_string(); // 键

        Ok(Scard { key })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.records.get(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::Set(set) => {
                        Ok(Frame::Integer(set.len() as i64))
                    },
                    _ => {
                        let f = "ERR Operation against a key holding the wrong kind of value";
                        Ok(Frame::Error(f.to_string()))
                    }
                }
            },
            None => {
                Ok(Frame::Integer(0))
            }
        }
    }
}