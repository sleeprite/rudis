use std::collections::HashMap;

use anyhow::Error;
use crate::{db::{Db, Structure}, frame::Frame};

pub struct Hmset {
    key: String,
    fields: HashMap<String, String>,
}

impl Hmset {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {

        let key = frame.get_arg(1);
        
        if key.is_none() {
            return Err(Error::msg("ERR wrong number of arguments for 'hmset' command"));
        }

        let args = frame.get_args();

        if args.len() % 2 != 0 {
            return Err(Error::msg("ERR wrong number of arguments for 'hmset' command"));
        }

        let mut fields = HashMap::new();

        for i in (2..args.len()).step_by(2) {
            let field = args[i].to_string();
            let value = args[i + 1].to_string();
            fields.insert(field, value);
        }

        Ok(Hmset {
            key: key.unwrap().to_string(),
            fields,
        })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.get_mut(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::Hash(hash) => {
                        for (field, value) in self.fields {
                            hash.insert(field, value);
                        }
                        Ok(Frame::SimpleString("OK".to_string()))
                    },
                    _ => {
                        let f = "ERR Operation against a key holding the wrong kind of value";
                        Ok(Frame::Error(f.to_string()))
                    }
                }
            },
            None => {
                db.insert(self.key, Structure::Hash(self.fields));
                Ok(Frame::SimpleString("OK".to_string()))
            }
        }
    }
}