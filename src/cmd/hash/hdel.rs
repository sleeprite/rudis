use anyhow::Error;
use crate::{db::{Db, Structure}, frame::Frame};

pub struct Hdel {
    key: String,
    fields: Vec<String>,
}

impl Hdel {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();

        if args.len() < 3 {
            return Err(Error::msg("ERR wrong number of arguments for 'hdel' command"));
        }

        let key = args[1].to_string();
        let fields = args[2..].iter().map(|arg| arg.to_string()).collect();

        Ok(Hdel {
            key,
            fields,
        })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.get_mut(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::Hash(hash) => {

                        let mut deleted_count = 0;

                        for field in &self.fields {
                            if hash.remove(field).is_some() {
                                deleted_count += 1;
                            }
                        }

                        Ok(Frame::Integer(deleted_count as i64))
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