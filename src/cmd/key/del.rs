use anyhow::Error;

use crate::{db::Db, frame::Frame};

pub struct Del {
    keys: Vec<String>,
}

impl Del {

    /**
     * 获取键的集合
     * 
     * @param frame 命令帧
     */
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let keys = frame.get_args_from_index(1);

        if keys.is_empty() {
            return Err(Error::msg("ERR wrong number of arguments for 'del' command"));
        } 
       
        Ok(Del { 
            keys: keys 
        })
    }

/**
     * 执行命令逻辑
     * 
     * @param db 数据库
     */
    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        let mut counter: usize = 0; // 使用 usize 作为计数器
        for key in self.keys {
            match db.remove(&key) {
                Some(_) => counter += 1, // 如果键存在，增加计数器
                None => { 
                    // 键不存在，不增加计数器
                },
            }
        } 
        // 将 usize 转换为 i64 以符合 Frame::Integer 的要求
        Ok(Frame::Integer(counter as i64))
    }
}