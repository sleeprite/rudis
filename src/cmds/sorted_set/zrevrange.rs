use anyhow::Error;

use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Zrevrange {
    key: String,
    start: i64,
    stop: i64,
    with_scores: bool,
}

impl Zrevrange {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        if args.len() < 4 {
            return Err(Error::msg("ERR wrong number of arguments for 'zrevrange' command"));
        }

        let key = args[1].to_string();
        let start = args[2].to_string().parse::<i64>().map_err(|_| Error::msg("ERR value is not an integer or out of range"))?;
        let stop = args[3].to_string().parse::<i64>().map_err(|_| Error::msg("ERR value is not an integer or out of range"))?;

        // 检查是否有 WITHSCORES 参数
        let mut with_scores = false;
        for arg in args.iter().skip(4) {
            let arg_up = arg.to_uppercase();
            if arg_up == "WITHSCORES" {
                with_scores = true;
            } else {
                return Err(Error::msg("ERR syntax error"));
            }
        }

        Ok(Zrevrange { key, start, stop, with_scores })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.records.get(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::SortedSet(set) => {
                        let len = set.len() as i64;

                        // 处理负数索引（相对于长度）
                        let start_idx = if self.start < 0 {
                            (len + self.start).max(0)
                        } else {
                            self.start.min(len)
                        };

                        let stop_idx = if self.stop < 0 {
                            (len + self.stop).max(-1)
                        } else {
                            self.stop.min(len - 1)
                        };

                        if start_idx > stop_idx || start_idx >= len {
                            return Ok(Frame::Array(vec![]));
                        }

                        // ZREVRANGE 的 start/stop 是在"降序后"的索引
                        // 例如降序 [c, b, a]，start=0,stop=1 应该返回 [c, b]
                        // 转换为底层升序跳表的正向索引：
                        //   desc_index i  <=>  asc_index (len - 1 - i)
                        //   desc 的 [start, stop]  <=>  asc 的 [len-1-stop, len-1-start]
                        let asc_start = (len - 1 - stop_idx) as usize;
                        let asc_stop = (len - 1 - start_idx) as usize;
                        let mut selected = set.range(asc_start, asc_stop);
                        // 反转得到降序
                        selected.reverse();

                        let mut result = Vec::new();
                        for (member, score) in selected {
                            result.push(Frame::BulkString(member));
                            if self.with_scores {
                                result.push(Frame::BulkString(score.to_string()));
                            }
                        }

                        Ok(Frame::Array(result))
                    },
                    _ => {
                        let f = "ERR Operation against a key holding the wrong kind of value";
                        Ok(Frame::Error(f.to_string()))
                    }
                }
            },
            None => {
                // 键不存在，返回空数组
                Ok(Frame::Array(vec![]))
            }
        }
    }
}
