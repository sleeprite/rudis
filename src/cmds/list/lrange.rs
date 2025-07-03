use anyhow::Error;
use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Lrange {
    key: String,
    start: i64,
    stop: i64,
}

impl Lrange {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let key = frame.get_arg(1);
        let start = frame.get_arg(2);
        let stop = frame.get_arg(3);

        if key.is_none() || start.is_none() || stop.is_none() {
            return Err(Error::msg("ERR wrong number of arguments for 'lrange' command"));
        }

        let final_key = key.unwrap().to_string(); // 键

        let start = match start.unwrap().parse::<i64>() {
            Ok(n) => n,
            Err(_) => return Err(Error::msg("ERR value is not an integer or out of range")),
        };

        let stop = match stop.unwrap().parse::<i64>() {
            Ok(n) => n,
            Err(_) => return Err(Error::msg("ERR value is not an integer or out of range")),
        };

        Ok(Lrange {
            key: final_key,
            start,
            stop,
        })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.get(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::List(list) => {

                        let len = list.len() as i64;
                        let start = if self.start < 0 { len + self.start } else { self.start };
                        let stop = if self.stop < 0 { len + self.stop } else { self.stop };

                        let start = std::cmp::max(0, start);
                        let stop = std::cmp::min(len - 1, stop);

                        if start > stop || start >= len || stop < 0 {
                            return Ok(Frame::Array(vec![]));
                        }

                        let result: Vec<Frame> = list.iter()
                            .skip(start as usize)
                            .take((stop - start + 1) as usize)
                            .map(|item| Frame::BulkString(item.clone()))
                            .collect();

                        Ok(Frame::Array(result))
                    },
                    _ => {
                        let f = "ERR Operation against a key holding the wrong kind of value";
                        Ok(Frame::Error(f.to_string()))
                    }
                }
            },
            None => Ok(Frame::Array(vec![])),
        }
    }
}