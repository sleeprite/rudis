use anyhow::Error;

use crate::{db::Db, frame::Frame, utils::atom_integer::AtomInteger};

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
        let mut counter = AtomInteger::new();
        for key in self.keys {
            match db.remove(&key) {
                Some(_) => counter.increment(),
                None => { 
                    // 键不存在，不增加计数器
                },
            }
        } 
        let count = counter.get(); 
        Ok(Frame::Integer(count))
    }
}