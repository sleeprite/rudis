use anyhow::Error;

use crate::{db::Db, frame::Frame, structure::Structure};

pub struct Set {
    key: String,
    val: String,
    ttl: Option<u64>
}

impl Set {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error>{

        let key = frame.get(1);
        let val = frame.get(2);

        if key.is_none() {
            return Err(Error::msg("Key is missing"));
        }

        if val.is_none() {
            return Err(Error::msg("Val is missing"));
        }

        let fianl_key = key.unwrap().to_string();
        let final_val = val.unwrap().to_string();

        // （1）遍历 frame 数组，检查是否包含 EX 与 PX 关键字

        // （2）包含则取 idx + 1 位置作为 ttl
        
        // （3）EX 秒，PX 毫秒

        let args = match frame {
            Frame::Array(args) => args,
            _ => return Err(Error::msg("Frame is not an Array variant")),
        };

        let mut ttl: Option<u64> = None; // 默认 ttl 为 0
        for (idx, item) in args.iter().enumerate() {
            if idx > 2 { // 从第三个参数开始检查，因为前两个是 key 和 val
                match item.as_str() {
                    "EX" | "PX" => {
                        if let Some(ttl_value) = args.get(idx + 1) {
                            ttl = match item.as_str() {
                                "EX" => Some(ttl_value.parse::<u64>()? * 1000), // EX 秒
                                "PX" => Some(ttl_value.parse::<u64>()?), // PX 毫秒
                                _ => None,
                            };
                            break; // 找到 ttl 后退出循环
                        }
                    },
                    _ => continue,
                }
            }
        }
        
        Ok(Set { 
            key: fianl_key, 
            val: final_val,
            ttl: ttl
        })
    }

    pub fn apply(self,db: &mut Db) -> Result<Frame, Error> {
        db.insert(self.key.clone(), Structure::String(self.val));
        if let Some(ttl) = self.ttl {
            db.expire(self.key.clone(), ttl);
        }
        Ok(Frame::Ok)
    }
}