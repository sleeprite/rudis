use anyhow::Error;
use crate::{frame::Frame};

pub struct Multi;

impl Multi {
    pub fn parse_from_frame(_frame: Frame) -> Result<Self, Error> {
        Ok(Multi)
    }

    pub fn apply(&self, handler: &mut crate::server::Handler) -> Result<Frame, Error> {
        handler.start_transaction();
        Ok(Frame::Ok)
    }
}