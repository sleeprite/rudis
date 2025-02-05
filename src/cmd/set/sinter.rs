use anyhow::Error;
use crate::{db::{Db, Structure}, frame::Frame};
use std::collections::HashSet;

pub struct Sinter {
    keys: Vec<String>,
}

impl Sinter {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        if args.len() < 2 {
            return Err(Error::msg("ERR wrong number of arguments for 'sinter' command"));
        }
        let keys: Vec<String> = args[1..].iter().map(|arg| arg.to_string()).collect();
        Ok(Sinter { keys })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {

        let mut iter = self.keys.iter();
        let first = iter.next().unwrap();

        match db.records.get(first) {
            Some(structure) => {
                match structure {
                    Structure::Set(first_set) => {       
                        let mut intersection: HashSet<String> = first_set.clone();
                        for key in iter {
                            match db.records.get(key) {
                                Some(structure) => {
                                    match structure {
                                        Structure::Set(set) => {
                                            intersection = intersection.intersection(set).cloned().collect();
                                        },
                                        _ => {
                                            let f = "ERR Operation against a key holding the wrong kind of value";
                                            return Ok(Frame::Error(f.to_string()));
                                        }
                                    }
                                },
                                None => {
                                    intersection.clear();
                                    break;
                                }
                            }
                        }
                        let mut result = Vec::new();
                        for member in intersection.iter() {
                            result.push(Frame::BulkString(member.clone()));
                        }
                        Ok(Frame::Array(result))
                    },
                    _ => {
                        let f = "ERR Operation against a key holding the wrong kind of value";
                        Ok(Frame::Error(f.to_string()))
                    }
                }
            },
            None => {
                Ok(Frame::Array(Vec::new()))
            }
        }
    }
}