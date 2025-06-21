use anyhow::Error;
use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Spop {
    key: String,
    count: Option<usize>,
}

impl Spop {
    
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {

        let args = frame.get_args();

        if args.len() < 2 || args.len() > 3 {
            return Err(Error::msg("ERR wrong number of arguments for 'spop' command"));
        }

        let key = args[1].to_string(); // 键
        let count = if args.len() == 3 {
            match args[2].parse::<usize>() {
                Ok(c) => Some(c),
                Err(_) => return Err(Error::msg("ERR value is not an integer or out of range")),
            }
        } else {
            None
        };

        Ok(Spop { key, count })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.records.get_mut(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::Set(set) => {
                        if set.is_empty() {
                            Ok(Frame::Null)
                        } else {
                            let pop_count = self.count.unwrap_or(1);
                            let mut popped_members = Vec::new();
                            for _ in 0..pop_count {
                                if let Some(member) = set.iter().next().cloned() {
                                    set.remove(&member);
                                    popped_members.push(Frame::BulkString(member));
                                } else {
                                    break;
                                }
                            }
                            if pop_count == 1 {
                                Ok(popped_members.pop().unwrap_or(Frame::Null))
                            } else {
                                Ok(Frame::Array(popped_members))
                            }
                        }
                    },
                    _ => {
                        let f = "ERR Operation against a key holding the wrong kind of value";
                        Ok(Frame::Error(f.to_string()))
                    }
                }
            },
            None => {
                Ok(Frame::Null)
            }
        }
    }
}