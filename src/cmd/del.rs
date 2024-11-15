use anyhow::Error;

use crate::{db::Db, frame::Frame, tools::atom_integer::AtomInteger};

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
        let keys_vec = frame.get_from_to_vec(1);
        if keys_vec.is_none() {
            return Err(Error::msg("No keys provided"));
        }
        let keys = keys_vec.unwrap();
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
                true => counter.increment(),
                false => (), // 键不存在，不增加计数器
            }
        } // 获取计数
        let count = counter.get(); 
        Ok(Frame::Integer(count))
    }
}