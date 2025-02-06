use anyhow::Error;

use crate::frame::Frame;

pub struct Echo {
    message: String,
}

impl Echo {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let message = match frame.get_arg(1) {
            Some(name) => name.to_string(),
            None => return Err(Error::msg("ERR wrong number of arguments for 'echo' command")),
        };
        Ok(Echo {
            message
        })   
    }

    pub fn apply(self) -> Result<Frame, Error> {
        Ok(Frame::BulkString(self.message))
    }
}