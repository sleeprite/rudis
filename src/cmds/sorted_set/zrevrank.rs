use anyhow::Error;

use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Zrevrank {
    key: String,
    member: String,
}

impl Zrevrank {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        if args.len() != 3 {
            return Err(Error::msg("ERR wrong number of arguments for 'zrevrank' command"));
        }
        let key = args[1].to_string(); // 键
        let member = args[2].to_string(); // 成员
        Ok(Zrevrank { key, member })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.records.get(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::SortedSet(set) => {
                        // 升序排名 asc_rank ∈ [0, len-1]
                        // 降序排名 = len - 1 - asc_rank
                        if let Some(asc_rank) = set.rank(&self.member) {
                            let desc_rank = (set.len() - 1 - asc_rank) as i64;
                            Ok(Frame::Integer(desc_rank))
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
