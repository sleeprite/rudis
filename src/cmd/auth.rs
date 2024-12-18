use std::sync::Arc;

use anyhow::Error;

use crate::{frame::Frame, session::SessionManager};

pub struct Auth {}

impl Auth {

    pub fn parse_from_frame(_frame: Frame) -> Result<Self, Error> {
        Ok(Auth {})
    }

    pub fn apply(self, _session_manager: Arc<SessionManager>, _session_id: Arc<String>) -> Result<Frame, Error> {
        Ok(Frame::Ok)
    }
}