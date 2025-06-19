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
            port = Some(frames[2].to_string());
            addr = Some(frames[4].to_string());

        } else {
            return Err(Error::msg("REPLCONF command must be an array frame"));
        }
        Ok(Replconf { port, addr })
    }

    pub fn apply(self) -> Result<Frame, Error> {
        log::info!("Slave 节点信息 - {}:{}", self.addr.unwrap(), self.port.unwrap());
        Ok(Frame::Ok)
    }
}