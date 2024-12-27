use anyhow::Error;

use crate::frame::Frame;

pub struct Unknown {
    command_name: String,
    args: String
}

impl Unknown {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {        

        let command_name = match frame.get_arg(0) {
            Some(name) => name.to_string(),
            None => return Err(Error::msg("Failed to get command name")),
        };

        let mut args = String::new();
        for arg in frame.get_args().iter().skip(1) { 
            args.push_str(arg);
            args.push(' '); // 参数之间加上空格
        }

        // 移除最后的空格
        if !args.is_empty() {
            args.pop();
        }
        
        Ok(Unknown {
            command_name,
            args
        })
    }

    pub fn apply(self) -> Result<Frame, Error> {
        Ok(Frame::Error(format!("ERR unknown command `{}`, with args beginning with: `{}`", self.command_name, self.args)))
    }
}