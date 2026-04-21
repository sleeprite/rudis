use anyhow::Error;

use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Zremrangebyrank {
    key: String,
    start: i64,
    stop: i64,
}

impl Zremrangebyrank {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        if args.len() != 4 {
            return Err(Error::msg("ERR wrong number of arguments for 'zremrangebyrank' command"));
        }
        let key = args[1].to_string();
        let start = args[2].parse::<i64>().map_err(|_| Error::msg("ERR value is not an integer or out of range"))?;
        let stop = args[3].parse::<i64>().map_err(|_| Error::msg("ERR value is not an integer or out of range"))?;
        Ok(Zremrangebyrank { key, start, stop })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        let removed = match db.records.get_mut(&self.key) {
            Some(Structure::SortedSet(set)) => {
                let len = set.len() as i64;
                // 负索引：相对长度；start/stop 都按升序排名
                let start_idx = if self.start < 0 { (len + self.start).max(0) } else { self.start.min(len) };
                let stop_idx = if self.stop < 0 { (len + self.stop).max(-1) } else { self.stop.min(len - 1) };

                if start_idx > stop_idx || start_idx >= len {
                    return Ok(Frame::Integer(0));
                }

                // 收集要删的成员（通过 range），再逐个移除
                let victims: Vec<String> = set
                    .range(start_idx as usize, stop_idx as usize)
                    .into_iter()
                    .map(|(m, _)| m)
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

        // 集合清空后删键
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
