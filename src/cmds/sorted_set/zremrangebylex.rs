use anyhow::Error;

use crate::{store::db::{Db, Structure}, frame::Frame};
use super::range_util::{parse_lex_bound, member_in_lex_range, LexBound};

pub struct Zremrangebylex {
    key: String,
    min: LexBound,
    max: LexBound,
}

impl Zremrangebylex {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        if args.len() != 4 {
            return Err(Error::msg("ERR wrong number of arguments for 'zremrangebylex' command"));
        }
        let key = args[1].to_string();
        let min = parse_lex_bound(&args[2])?;
        let max = parse_lex_bound(&args[3])?;
        Ok(Zremrangebylex { key, min, max })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        let removed = match db.records.get_mut(&self.key) {
            Some(Structure::SortedSet(set)) => {
                let victims: Vec<String> = set.members_lex()
                    .into_iter()
                    .filter(|m| member_in_lex_range(m, &self.min, &self.max))
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
