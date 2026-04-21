use anyhow::Error;

use crate::{store::db::{Db, Structure}, frame::Frame};
use super::range_util::{parse_lex_bound, member_in_lex_range, parse_range_options, apply_limit, LexBound};

pub struct Zrevrangebylex {
    key: String,
    // ZREVRANGEBYLEX 的参数顺序：max 在前，min 在后
    max: LexBound,
    min: LexBound,
    limit: Option<super::range_util::Limit>,
}

impl Zrevrangebylex {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        if args.len() < 4 {
            return Err(Error::msg("ERR wrong number of arguments for 'zrevrangebylex' command"));
        }

        let key = args[1].to_string();
        let max = parse_lex_bound(&args[2])?;
        let min = parse_lex_bound(&args[3])?;

        let tail: Vec<&str> = args[4..].iter().map(|s| s.as_str()).collect();
        let (_with_scores, limit) = parse_range_options(&tail, false)?;

        Ok(Zrevrangebylex { key, max, min, limit })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.records.get(&self.key) {
            Some(Structure::SortedSet(set)) => {
                // 按字典序过滤后反转得到降序
                let mut filtered: Vec<String> = set.members_lex()
                    .into_iter()
                    .filter(|m| member_in_lex_range(m, &self.min, &self.max))
                    .collect();
                filtered.reverse();
                let paged = apply_limit(filtered, self.limit);

                let result = paged.into_iter().map(Frame::BulkString).collect();
                Ok(Frame::Array(result))
            }
            Some(_) => Ok(Frame::Error("ERR Operation against a key holding the wrong kind of value".to_string())),
            None => Ok(Frame::Array(vec![])),
        }
    }
}
