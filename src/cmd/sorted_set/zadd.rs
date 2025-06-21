use std::collections::BTreeMap;

use anyhow::Error;

use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Zadd {
    key: String,
    members: Vec<(f64, String)>, // 成员及其分数
}

impl Zadd {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        if args.len() < 4 || args.len() % 2 != 0 {
            return Err(Error::msg("ERR wrong number of arguments for 'zadd' command"));
        }
        
        let key = args[1].to_string(); // 键
        let mut members = Vec::new();

        for chunk in args[2..].chunks(2) {
            if chunk.len() != 2 {
                return Err(Error::msg("ERR wrong number of arguments for 'zadd' command"));
            }
            let score = chunk[0].parse::<f64>().map_err(|_| Error::msg("ERR score is not a valid float"))?;
            let member = chunk[1].to_string();
            members.push((score, member));
        }

        Ok(Zadd { key, members })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        let mut added_count = 0;

        match db.records.get_mut(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::SortedSet(set) => {
                        for (score, member) in self.members {
                            if set.insert(member, score).is_none() {
                                added_count += 1; // 成员新增成功
                            }
                        }
                    },
                    _ => {
                        let f = "ERR Operation against a key holding the wrong kind of value";
                        return Ok(Frame::Error(f.to_string()));
                    }
                }
            },
            None => { // 键不存在，创建一个新的有序集合并插入所有成员
                let mut set = BTreeMap::new();
                for (score, member) in self.members {
                    set.insert(member, score);
                    added_count += 1; // 成员新增成功
                }
                db.records.insert(self.key, Structure::SortedSet(set));
            }
        }

        Ok(Frame::Integer(added_count as i64))
    }
}