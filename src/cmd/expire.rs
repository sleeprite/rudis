use anyhow::Error;

use crate::{db::Db, frame::Frame};

pub struct Expire {}

impl Expire {

    pub fn parse_from_frame(_frame: Frame) -> Result<Self, Error> {
        Ok(Expire { })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        Ok(Frame::Ok)
    }
}