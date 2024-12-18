use anyhow::Error;

use crate::{db::Db, frame::Frame};

pub struct Ttl {}

impl Ttl {

    pub fn parse_from_frame(_frame: Frame) -> Result<Self, Error> {        
        Ok(Ttl {})
    }

    pub fn apply(self, _db: &Db) -> Result<Frame, Error> {
        Ok(Frame::Ok)
    }
}