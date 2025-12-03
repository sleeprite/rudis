use anyhow::Error;
use crate::{frame::Frame};

pub struct Discard;

impl Discard {
    pub fn parse_from_frame(_frame: Frame) -> Result<Self, Error> {
        Ok(Discard)
    }

    pub fn apply(&self, handler: &mut crate::server::Handler) -> Result<Frame, Error> {
        if !handler.is_in_transaction() {
            return Ok(Frame::Error("ERR DISCARD without MULTI".to_string()));
        }
        handler.clear_transaction();
        Ok(Frame::Ok)
    }
}