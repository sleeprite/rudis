use anyhow::Error;
use crate::{db::{Db, Structure}, frame::Frame};

pub struct Rpushx {
    key: String,
    values: Vec<String>,
}

impl Rpushx {
    
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        if args.len() < 3 {
            return Err(Error::msg("ERR wrong number of arguments for 'rpushx' command"));
        }
        let key = args[1].to_string(); // 键
        let values: Vec<String> = args.iter().skip(2).map(|v| v.to_string()).collect(); // 值
        Ok(Rpushx { key, values })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.get_mut(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::List(list) => {
                        for value in self.values {
                            list.push(value); // 向引用 mut 中添加数据
                        }
                        Ok(Frame::Integer(list.len() as i64))
                    },
                    _ => {
                        let f = "ERR Operation against a key holding the wrong kind of value";
                        Ok(Frame::Error(f.to_string()))
                    }
                }
            },
            None => {
                // 如果键不存在，则不执行任何操作，返回 0
                Ok(Frame::Integer(0))
            }
        }
    }
}