use anyhow::Error;

use crate::{
    frame::Frame,
    store::db::{Db, Structure},
};

pub struct GetRange {
    key: String,
    start: i64,
    end: i64,
}

impl GetRange {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let key = frame.get_arg(1);
        let start = frame.get_arg(2);
        let end = frame.get_arg(3);

        if key.is_none() || start.is_none() || end.is_none() {
            return Err(Error::msg(
                "ERR wrong number of arguments for 'getrange' command",
            ));
        }

        let final_key = key.unwrap().to_string();
        let final_start = start.unwrap().to_string();
        let final_end = end.unwrap().to_string();

        let start_int = match final_start.parse::<i64>() {
            Ok(n) => n,
            Err(_) => return Err(Error::msg("ERR value is not an integer or out of range")),
        };
        
        let end_int = match final_end.parse::<i64>() {
            Ok(n) => n,
            Err(_) => return Err(Error::msg("ERR value is not an integer or out of range")),
        };

        Ok(GetRange {
            key: final_key,
            start: start_int,
            end: end_int,
        })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.get(&self.key) {
            Some(Structure::String(s)) => {
                let len = s.len() as i64;

                // 处理负数索引
                let start = if self.start < 0 {
                    (len + self.start).max(0)
                } else {
                    self.start
                };
                
                let end = if self.end < 0 {
                    len + self.end
                } else {
                    self.end
                };

                // 确保索引在有效范围内
                let start = start.min(len).max(0) as usize;
                let end = end.min(len).max(0) as usize;

                // 当起始位置超过结束位置时返回空字符串
                if start > end {
                    return Ok(Frame::Null);
                }

                // 处理UTF-8安全截取
                let mut char_indices = s.char_indices();
                let start_byte = match char_indices.nth(start) {
                    Some((idx, _)) => idx,
                    None => return Ok(Frame::Null),
                };
                
                let end_byte = match char_indices.nth(end - start - 1) {
                    Some((idx, _)) => idx,
                    None => s.len(),
                } + s.chars().nth(end).map_or(0, |c| c.len_utf8());

                let substring = &s[start_byte..end_byte.min(s.len())];
                Ok(Frame::BulkString(substring.into()))
            }
            Some(_) => Err(Error::msg("WRONGTYPE Operation against a key holding the wrong kind of value")),
            None => Ok(Frame::Null),
        }
    }
}