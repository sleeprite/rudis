use anyhow::Error;

use crate::frame::Frame;

pub struct Ping;

impl Ping {

    pub fn parse_from_frame(_frame: Frame) -> Result<Self, Error> {
        Ok(Ping)
    }

    pub fn apply(self) -> Result<Frame, Error> {
        Ok(Frame::SimpleString("PONG".to_string()))
    }
}