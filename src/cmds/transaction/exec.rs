use anyhow::Error;
use crate::{frame::Frame};

pub struct Exec;

impl Exec {
    pub fn parse_from_frame(_frame: Frame) -> Result<Self, Error> {
        Ok(Exec)
    }

    pub fn apply(&self) -> Result<Frame, Error> {
        // EXEC命令本身不执行任何操作，只是标记需要执行事务
        // 实际的执行将在Handler中处理
        Ok(Frame::Ok)
    }
}