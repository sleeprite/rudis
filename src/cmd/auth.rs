use anyhow::Error;

use crate::frame::Frame;

pub struct Auth {}

impl Auth {

    pub fn parse_from_frame(_frame: Frame) -> Result<Self, Error> {
        Ok(Auth { })
    }

    pub fn apply(self) -> Result<Frame, Error> {
        Ok(Frame::Ok)
    }
}