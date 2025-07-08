use anyhow::Error;
use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct GetSet {
    key: String,
    value: String,
}

impl GetSet {
    /// 从 Frame 解析出 GetSet 命令
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        // 确保参数数量正确（key + value）
        if frame.get_args().len() < 3 {
            return Err(Error::msg("ERR wrong number of arguments for 'getset' command"));
        }

        let key = frame.get_arg(1).ok_or(Error::msg("ERR missing key"))?.to_string();
        let value = frame.get_arg(2).ok_or(Error::msg("ERR missing value"))?.to_string();

        Ok(GetSet { key, value })
    }

    /// 应用 GetSet 命令到数据库
    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        // 获取旧值（同时检查类型）
        let old_value = db.get(&self.key).and_then(|structure| {
            match structure {
                Structure::String(s) => Some(s.clone()), // 正确类型：保存字符串值
                _ => None, // 非字符串类型视为不存在（按 Redis 行为）
            }
        });

        // 插入新值（覆盖旧值）
        db.insert(self.key.clone(), Structure::String(self.value.clone()));

        // TODO 是否移除过期时间

        // 返回结果：旧值或 nil
        match old_value {
            Some(val) => Ok(Frame::BulkString(val)),
            None => Ok(Frame::Null),
        }
    }
}