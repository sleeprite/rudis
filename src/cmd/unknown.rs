use anyhow::Error;

use crate::{db::Db, frame::Frame};

pub struct Unknown {}

impl Unknown {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        Ok(Unknown {})
    }

    pub fn apply(self, db: &Db) -> Result<Frame, Error> {
        Ok(Frame::Ok)
    }
}