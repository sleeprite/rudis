use anyhow::Error;

use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Zscore {
    key: String,
    member: String,
}

impl Zscore {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        if args.len() != 3 {
            return Err(Error::msg("ERR wrong number of arguments for 'zscore' command"));
        }
        let key = args[1].to_string(); // 键
        let member = args[2].to_string(); // 成员
        Ok(Zscore { key, member })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.records.get(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::SortedSet(set) => {
                        if let Some(score) = set.get(&self.member) {
                            Ok(Frame::BulkString(score.to_string()))
                        } else {
                            Ok(Frame::Null)
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