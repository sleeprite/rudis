use std::sync::Arc;

use anyhow::Error;

use crate::{frame::Frame, session::SessionManager};

pub struct Auth {
    requirepass: String,
}

impl Auth {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let requirepass = frame.get_arg(1);

        if requirepass.is_none() {
            return Err(Error::msg(
                "ERR wrong number of arguments for 'auth' command",
            ));
        }

        Ok(Auth {
            requirepass: requirepass.unwrap().to_string(),
        })
    }

    pub fn apply(self, session_manager: Arc<SessionManager>, session_id: &String) -> Result<Frame, Error> {
        if session_manager.login(session_id, &self.requirepass) {
            Ok(Frame::Ok)
        } else {
            Ok(Frame::Error("ERR invalid password".to_string()))
        }
    }
}
