use anyhow::Error;
use crate::{frame::Frame};

pub struct Discard;

impl Discard {
    pub fn parse_from_frame(_frame: Frame) -> Result<Self, Error> {
        Ok(Discard)
    }

    pub fn apply(&self, handler: &mut crate::server::Handler) -> Result<Frame, Error> {
        if !handler.get_session_mut().is_in_transaction() {
            return Ok(Frame::Error("ERR DISCARD without MULTI".to_string()));
        }
        handler.get_session_mut().clear_transaction();
        Ok(Frame::Ok)
    }
}