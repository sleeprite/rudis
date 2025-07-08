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
    
        let value = match db.get(&self.key) {
            Some(Structure::String(s)) => s,
            Some(_) => return Err(Error::msg(
                "WRONGTYPE Operation against a key holding the wrong kind of value"
            )),
            None => return Ok(Frame::Null),
        };

        let len = value.len() as i64;
        let normalize = |index: i64| {
            if index < 0 {
                (len + index).max(0)
            } else {
                index
            }
            .min(len) as usize
        };

        let start = normalize(self.start);
        let end = normalize(self.end);
        
        // 处理无效范围
        if start > end {
            return Ok(Frame::BulkString("".into()));
        }

        // 安全截取字符串 (考虑 UTF-8 边界)
        let substring = match value.char_indices().nth(start) {
            Some((start_idx, _)) => {
                let remainder = &value[start_idx..];
                match remainder.char_indices().nth(end - start) {
                    Some((end_idx, ch)) => &remainder[..end_idx + ch.len_utf8()],
                    None => remainder,
                }
            }
            None => "",
        };

        Ok(Frame::BulkString(substring.into()))
    }
}