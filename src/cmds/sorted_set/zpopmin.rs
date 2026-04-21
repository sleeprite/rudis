use anyhow::Error;

use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Zpopmin {
    key: String,
    count: Option<i64>,
}

impl Zpopmin {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        if args.len() < 2 || args.len() > 3 {
            return Err(Error::msg("ERR wrong number of arguments for 'zpopmin' command"));
        }
        let key = args[1].to_string();
        let count = if args.len() == 3 {
            let c = args[2].parse::<i64>()
                .map_err(|_| Error::msg("ERR value is not an integer or out of range"))?;
            if c < 0 {
                return Err(Error::msg("ERR value is out of range, must be positive"));
            }
            Some(c)
        } else {
            None
        };
        Ok(Zpopmin { key, count })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        let popped: Vec<(String, f64)> = match db.records.get_mut(&self.key) {
            Some(Structure::SortedSet(set)) => {
                let take = match self.count {
                    None => 1usize,
                    Some(0) => return Ok(Frame::Array(vec![])),
                    Some(c) => (c as usize).min(set.len()),
                };
                if take == 0 {
                    return Ok(Frame::Array(vec![]));
                }
                // 跳表升序，前 take 个就是最小分数成员
                let victims = set.range(0, take - 1);
                for (m, _) in &victims {
                    set.remove(m);
                }
                victims
            }
            Some(_) => {
                return Ok(Frame::Error("ERR Operation against a key holding the wrong kind of value".to_string()));
            }
            None => Vec::new(),
        };

        // 集合清空后删键
        if let Some(Structure::SortedSet(set)) = db.records.get(&self.key) {
            if set.is_empty() {
                db.records.remove(&self.key);
            }
        }

        let mut out = Vec::with_capacity(popped.len() * 2);
        for (member, score) in popped {
            out.push(Frame::BulkString(member));
            out.push(Frame::BulkString(score.to_string()));
        }
        Ok(Frame::Array(out))
    }
}
