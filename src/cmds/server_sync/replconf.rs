use anyhow::Error;

use crate::{frame::Frame};

pub struct Replconf {
    port: Option<String>,
    addr: Option<String>,
}

impl Replconf {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let mut port = None;
        let mut addr = None;
        if let Frame::Array(frames) = frame {
            if frames.len() < 5 {
                return Err(Error::msg("REPLCONF command requires at least 5 arguments"));
            }
            let cmd = frames[0].to_string().to_uppercase();
            if cmd != "REPLCONF" {
                return Err(Error::msg("Invalid command, expected REPLCONF"));
            }
            let mut index = 1;
            while index < frames.len() {
                let arg = frames[index].to_string().to_uppercase();
                match arg.as_str() {
                    "LISTENING-PORT" if index + 1 < frames.len() => {
                        port = Some(frames[index + 1].to_string());
                        index += 2; 
                    }
                    "IP-ADDRESS" if index + 1 < frames.len() => {
                        addr = Some(frames[index + 1].to_string());
                        index += 2; 
                    }
                    _ => {
                        index += 1;
                    }
                }
            }
        } else {
            return Err(Error::msg("REPLCONF command must be an array frame"));
        }

        if port.is_none() || addr.is_none() {
            return Err(Error::msg("Missing required parameters for REPLCONF"));
        }

        Ok(Replconf { port, addr })
    }

    pub fn apply(self) -> Result<Frame, Error> {

        
        log::info!("Slave 节点信息 - {}:{}", self.addr.unwrap(), self.port.unwrap());
        Ok(Frame::Ok)
    }
}