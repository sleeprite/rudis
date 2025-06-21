use std::collections::HashSet;

use anyhow::Error;
use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Sadd {
    key: String,
    members: Vec<String>,
}

impl Sadd {
    
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {

        let args = frame.get_args();

        if args.len() < 3 {
            return Err(Error::msg("ERR wrong number of arguments for 'sadd' command"));
        }

        let key = args[1].to_string(); // 键
        let members: Vec<String> = args.iter().skip(2).map(|v| v.to_string()).collect(); // 成员

        Ok(Sadd { key, members })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.records.get_mut(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::Set(set) => {
                        let mut added_count = 0;
                        for member in self.members {
                            if set.insert(member) {
                                added_count += 1;
                            }
                        }
                        Ok(Frame::Integer(added_count as i64))
                    },
                    _ => {
                        let f = "ERR Operation against a key holding the wrong kind of value";
                        Ok(Frame::Error(f.to_string()))
                    }
                }
            },
            None => {
                let mut set = HashSet::new(); 
                let mut added_count = 0;
                for member in self.members {
                    if set.insert(member) {
                        added_count += 1;
                    }
                }
                db.records.insert(self.key.clone(), Structure::Set(set));
                Ok(Frame::Integer(added_count as i64))
            }
        }
    }
}