use std::sync::Arc;

use anyhow::Error;

use crate::{ frame::Frame, session::SessionManager };

pub struct Ping;

impl Ping {

    pub fn parse_from_frame(_frame: Frame) -> Result<Self, Error> {
        Ok(Ping)
    }

    pub fn apply(self, _session_manager: Arc<SessionManager>, _session_id: &String) -> Result<Frame, Error> {
        Ok(Frame::SimpleString("PONG".to_string()))
    }
}