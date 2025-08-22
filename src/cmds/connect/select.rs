use anyhow::Error;

use crate::{frame::Frame, server::Handler};

pub struct Select {
    db_index: usize,
}

impl Select {

    pub fn get_db_index(&self) -> usize {
        self.db_index
    }

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {

        let db_string = frame.get_arg(1);
        let db_string = match db_string {
            Some(s) => s,
            None => {
                return Err(Error::msg("ERR wrong number of arguments for 'select' command"))
            }
        };
        
        let db_index: usize = match db_string.parse::<usize>() {
            Ok(num) => num,
            Err(_) => {
                return Err(Error::msg("ERR invalid DB index"));
            }
        };
        
        Ok(Select { db_index })
    }

    pub fn apply(self, handler:&mut Handler) -> Result<Frame, Error> {
        match handler.change_sender(self.db_index) {
            Ok(_) => Ok(Frame::Ok),
            Err(e) => {
                Ok(Frame::Error(e.to_string()))
            }
        }
    }
}