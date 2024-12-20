use std::sync::Arc;

use anyhow::Error;

use crate::{args::Args, frame::Frame, session::SessionManager};

pub struct Auth {
    requirepass: String
}

impl Auth {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        
        let requirepass = frame.get_arg(0);

        if requirepass.is_none() {
            return Err(Error::msg("ERR wrong number of arguments for 'auth' command"));
        }

        Ok(Auth {
            requirepass: requirepass.unwrap().to_string()
        })
    }

    pub fn apply(self, _session_manager: Arc<SessionManager>, _session_id: &String) -> Result<Frame, Error> {

        Ok(Frame::Ok)
    }
}