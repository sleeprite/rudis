use anyhow::Error;

use crate::{frame::Frame, network::server::Handler};

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

    pub fn apply(self, handler:&mut Handler) -> Result<Frame, Error> {
        match handler.login(&self.requirepass) {
            Ok(_) => Ok(Frame::Ok),
            Err(e) => {
                Ok(Frame::Error(e.to_string()))
            }
        }
    }
}
