use anyhow::Error;

use crate::{store::db::{Db, Structure}, frame::Frame};
use super::range_util::{parse_score_bound, score_in_range, ScoreBound};

pub struct Zremrangebyscore {
    key: String,
    min: ScoreBound,
    max: ScoreBound,
}

impl Zremrangebyscore {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        if args.len() != 4 {
            return Err(Error::msg("ERR wrong number of arguments for 'zremrangebyscore' command"));
        }
        let key = args[1].to_string();
        let min = parse_score_bound(&args[2])?;
        let max = parse_score_bound(&args[3])?;
        Ok(Zremrangebyscore { key, min, max })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        let removed = match db.records.get_mut(&self.key) {
            Some(Structure::SortedSet(set)) => {
                let victims: Vec<String> = set.iter()
                    .filter(|(_, score)| score_in_range(**score, &self.min, &self.max))
                    .map(|(m, _)| m.clone())
                    .collect();

                let mut removed = 0i64;
                for m in &victims {
                    if set.remove(m) {
                        removed += 1;
                    }
                }
                removed
            }
            Some(_) => {
                return Ok(Frame::Error("ERR Operation against a key holding the wrong kind of value".to_string()));
            }
            None => 0,
        };

        if removed > 0 {
            if let Some(Structure::SortedSet(set)) = db.records.get(&self.key) {
                if set.is_empty() {
                    db.records.remove(&self.key);
                }
            }
        }

        Ok(Frame::Integer(removed))
    }
}
