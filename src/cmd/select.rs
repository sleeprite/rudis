use anyhow::Error;

use crate::frame::Frame;

pub struct Select {}

impl Select {

    pub fn parse_from_frame(_frame: Frame) -> Result<Self, Error> {
        Ok(Select { })
    }

    pub fn apply(self) -> Result<Frame, Error> {
        Ok(Frame::Ok)
    }
}