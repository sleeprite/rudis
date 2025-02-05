use anyhow::Error;
use crate::{db::{Db, Structure}, frame::Frame};

pub struct Sunion {
    keys: Vec<String>,
}

impl Sunion {
    
    // 解析命令帧
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();

        // 至少需要两个键（一个命令名，一个或多个集合键）
        if args.len() < 2 {
            return Err(Error::msg("ERR wrong number of arguments for 'sunion' command"));
        }

        // 提取所有键
        let keys = args[1..].iter().map(|arg| arg.to_string()).collect();

        Ok(Sunion { keys })
    }

    // 应用命令
    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        let mut result_set = std::collections::HashSet::new();

        // 遍历所有键，获取并集
        for key in self.keys {
            if let Some(structure) = db.records.get(&key) {
                match structure {
                    Structure::Set(set) => {
                        for member in set.iter() {
                            result_set.insert(member.clone());
                        }
                    },
                    _ => {
                        // 如果某个键对应的值不是集合类型，返回错误
                        let f = "ERR Operation against a key holding the wrong kind of value";
                        return Ok(Frame::Error(f.to_string()));
                    }
                }
            }
        }

        // 将结果转换为 Frame::Array
        let members: Vec<Frame> = result_set.into_iter()
            .map(|member| Frame::BulkString(Some(member)))
            .collect();

        Ok(Frame::Array(members))
    }
}