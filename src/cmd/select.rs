use std::sync::Arc;

use anyhow::Error;

use crate::{frame::Frame, session::SessionManager};

pub struct Select {
    db: usize,
}

impl Select {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {

        let db_string = frame.get_arg(1);

        let db_string = match db_string {
            Some(s) => s,
            None => {
                return Err(Error::msg("message"))
            }
        };

        let db = match db_string.parse::<usize>() {
            Ok(num) => num,
            Err(_) => {
                return Err(Error::msg("message"));
            }
        };

        Ok(Select { db: db })
    }

    pub fn apply(self, session_manager: Arc<SessionManager>, session_id: Arc<String>) -> Result<Frame, Error> {
        Ok(Frame::Ok)
    }
}
