use anyhow::Error;
use crate::{db::{Db, Structure}, frame::Frame};

pub struct Srem {
    key: String,
    members: Vec<String>,
}

impl Srem {
    
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {

        let args = frame.get_args();

        if args.len() < 3 {
            return Err(Error::msg("ERR wrong number of arguments for 'srem' command"));
        }

        let key = args[1].to_string(); // 键
        let members = args[2..].iter().map(|arg| arg.to_string()).collect(); // 要移除的成员

        Ok(Srem { key, members })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.records.get_mut(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::Set(set) => {
                        let mut removed_count = 0;
                        for member in &self.members {
                            if set.remove(member) {
                                removed_count += 1;
                            }
                        }
                        Ok(Frame::Integer(removed_count as i64))
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