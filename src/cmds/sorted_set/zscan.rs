use anyhow::Error;
use crate::{store::db::{Db, Structure}, frame::Frame, tools::pattern};

pub struct Zscan {
    key: String,
    cursor: u64,
    pattern: Option<String>,
    count: Option<u64>,
}

impl Zscan {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args_from_index(1);
        if args.len() < 2 {
            return Err(Error::msg("ZSCAN command requires at least two arguments"));
        }

        let key = args[0].clone();
        let cursor = args[1].parse::<u64>()?;

        let mut pattern = None;
        let mut count = None;

        let mut i = 2;
        while i < args.len() {
            let arg = &args[i].to_uppercase();
            if arg == "MATCH" {
                if i + 1 >= args.len() {
                    return Err(Error::msg("MATCH option requires an argument"));
                }
                pattern = Some(args[i + 1].clone());
                i += 2;
            } else if arg == "COUNT" {
                if i + 1 >= args.len() {
                    return Err(Error::msg("COUNT option requires an argument"));
                }
                count = Some(args[i + 1].parse::<u64>()?);
                i += 2;
            } else {
                return Err(Error::msg(format!("Unknown option: {}", args[i])));
            }
        }

        Ok(Zscan { key, cursor, pattern, count })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        let pattern = self.pattern.unwrap_or_else(|| "*".to_string());
        let count = self.count.unwrap_or(10) as usize;

        match db.records.get(&self.key) {
            Some(Structure::SortedSet(set)) => {
                // 收集所有匹配的 (member, score)，按跳表升序
                let matched: Vec<(String, f64)> = set.members_with_scores()
                    .into_iter()
                    .filter(|(m, _)| pattern::is_match(m, &pattern))
                    .collect();

                let start = self.cursor as usize;
                let end = std::cmp::min(start + count, matched.len());

                let page: Vec<(String, f64)> = if start < matched.len() {
                    matched[start..end].to_vec()
                } else {
                    Vec::new()
                };

                let next_cursor = if end >= matched.len() { 0 } else { end as u64 };

                // ZSCAN 返回值的内层数组形式：[member1, score1, member2, score2, ...]
                let mut inner = Vec::with_capacity(page.len() * 2);
                for (m, s) in page {
                    inner.push(Frame::BulkString(m));
                    inner.push(Frame::BulkString(s.to_string()));
                }

                Ok(Frame::Array(vec![
                    Frame::Integer(next_cursor as i64),
                    Frame::Array(inner),
                ]))
            }
            Some(_) => {
                Ok(Frame::Error("ERR Operation against a key holding the wrong kind of value".to_string()))
            }
            None => {
                Ok(Frame::Array(vec![
                    Frame::Integer(0),
                    Frame::Array(vec![]),
                ]))
            }
        }
    }
}
