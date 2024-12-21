use std::sync::Arc;

use anyhow::Error;

use crate::{args::Args, frame::Frame, session::SessionManager};

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

    pub fn apply(self, session_manager: Arc<SessionManager>, session_id: &String, args: Arc<Args>) -> Result<Frame, Error> {
        match args.as_ref().requirepass.as_ref() {
            None => Ok(Frame::Ok),
            Some(server_requirepass) => {
                if self.requirepass == *server_requirepass { 
                    session_manager.set(session_id, Some(true), None);
                    Ok(Frame::Ok)
                } else {
                    let err_msg: String = "ERR invalid password".to_string(); 
                    Ok(Frame::Error(err_msg))
                }
            }
        }
    }
}
