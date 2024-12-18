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
                return Err(Error::msg("ERR wrong number of arguments for 'select' command"))
            }
        };
        
        let db: usize = match db_string.parse::<usize>() {
            Ok(num) => num,
            Err(_) => {
                return Err(Error::msg("ERR invalid DB index"));
            }
        };
        
        Ok(Select { db: db })
    }

    pub fn apply(self, session_manager: Arc<SessionManager>, session_id: Arc<String>) -> Result<Frame, Error> {
        session_manager.set(&*session_id, None, Some(self.db));
        Ok(Frame::Ok)
    }
}
