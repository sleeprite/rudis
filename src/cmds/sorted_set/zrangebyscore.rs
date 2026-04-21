use anyhow::Error;

use crate::{store::db::{Db, Structure}, frame::Frame};
use super::range_util::{parse_score_bound, score_in_range, parse_range_options, apply_limit, ScoreBound};

pub struct Zrangebyscore {
    key: String,
    min: ScoreBound,
    max: ScoreBound,
    with_scores: bool,
    limit: Option<super::range_util::Limit>,
}

impl Zrangebyscore {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        if args.len() < 4 {
            return Err(Error::msg("ERR wrong number of arguments for 'zrangebyscore' command"));
        }

        let key = args[1].to_string();
        let min = parse_score_bound(&args[2])?;
        let max = parse_score_bound(&args[3])?;

        let tail: Vec<&str> = args[4..].iter().map(|s| s.as_str()).collect();
        let (with_scores, limit) = parse_range_options(&tail, true)?;

        Ok(Zrangebyscore { key, min, max, with_scores, limit })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.records.get(&self.key) {
            Some(Structure::SortedSet(set)) => {
                // 底层跳表按 (score, member) 升序迭代
                let filtered: Vec<(String, f64)> = set.iter()
                    .filter(|(_, score)| score_in_range(**score, &self.min, &self.max))
                    .map(|(m, s)| (m.clone(), *s))
                    .collect();
                let paged = apply_limit(filtered, self.limit);

                let mut result = Vec::new();
                for (member, score) in paged {
                    result.push(Frame::BulkString(member));
                    if self.with_scores {
                        result.push(Frame::BulkString(score.to_string()));
                    }
                }
                Ok(Frame::Array(result))
            }
            Some(_) => Ok(Frame::Error("ERR Operation against a key holding the wrong kind of value".to_string())),
            None => Ok(Frame::Array(vec![])),
        }
    }
}
