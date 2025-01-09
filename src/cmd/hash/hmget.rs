use anyhow::Error;
use crate::{db::{Db, Structure}, frame::Frame};

pub struct Hmget {
    key: String,
    fields: Vec<String>,
}

impl Hmget {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();

        if args.len() < 3 {
            return Err(Error::msg("ERR wrong number of arguments for 'hmget' command"));
        }

        let key = args[1].to_string();
        let fields = args[2..].iter().map(|arg| arg.to_string()).collect();

        Ok(Hmget {
            key,
            fields,
        })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.get(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::Hash(hash) => {
                        let mut values = Vec::new();

                        for field in &self.fields {
                            if let Some(value) = hash.get(field) {
                                values.push(Frame::BulkString(Some(value.clone())));
                            } else {
                                values.push(Frame::Null);
                            }
                        }

                        Ok(Frame::Array(values))
                    },
                    _ => {
                        let f = "ERR Operation against a key holding the wrong kind of value";
                        Ok(Frame::Error(f.to_string()))
                    }
                }
            },
            None => {
                let null_values = vec![Frame::Null; self.fields.len()];
                Ok(Frame::Array(null_values))
            }
        }
    }
}