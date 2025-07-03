use anyhow::Error;

use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Zcount {
    key: String,
    min: f64,
    max: f64,
}

impl Zcount {
    
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        if args.len() != 4 {
            return Err(Error::msg("ERR wrong number of arguments for 'zcount' command"));
        }
        let key = args[1].to_string(); // 键
        let min = args[2].parse::<f64>().map_err(|_| Error::msg("ERR min is not a valid float"))?;
        let max = args[3].parse::<f64>().map_err(|_| Error::msg("ERR max is not a valid float"))?;
        Ok(Zcount { key, min, max })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.records.get(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::SortedSet(set) => {
                        let count = set.values().filter(|&&score| score >= self.min && score <= self.max).count();
                        Ok(Frame::Integer(count as i64))
                    },
                    _ => {
                        let f = "ERR Operation against a key holding the wrong kind of value";
                        Ok(Frame::Error(f.to_string()))
                    }
                }
            },
            None => {
                Ok(Frame::Integer(0)) // 如果键不存在，返回 0
            }
        }
    }
}