use anyhow::Error;

use crate::frame::Frame;

pub struct Client {
    subcommand: String
}

impl Client {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        
        if args.len() < 2 {
            return Err(Error::msg("ERR wrong number of arguments for 'client' command"));
        }

        let subcommand = args[1].to_uppercase();
        // let args: Vec<String> = args.iter().skip(2).map(|s| s.to_string()).collect();
        Ok(Client {
            subcommand,
            // args,
        })
    }

    pub fn apply(self) -> Result<Frame, Error> {
        match self.subcommand.as_str() {
            "SETINFO" => {
                // For SETINFO, we just acknowledge the command without doing anything
                // In a production Redis server, this would track client library info
                Ok(Frame::Ok)
            },
            _ => {
                Ok(Frame::Error(format!("ERR unknown subcommand '{}'", self.subcommand)))
            }
        }
    }
}