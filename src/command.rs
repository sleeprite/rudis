use anyhow::Error;

use crate::{cmd::{del::Del, select::Select, string::{get::Get, set::Set}, unknown::Unknown}, frame::Frame};

// 命令
pub enum Command {
    Set(Set),
    Get(Get),
    Del(Del),
    Unknown(Unknown),
    Select(Select)
}

impl Command {
    
    pub fn parse_from_frame(frame: Frame)  -> Result<Self, Error>  {
        let command_name = frame.get(0).unwrap();
        let command = match command_name.to_uppercase().as_str() {
            "SET" => Command::Set(Set::parse_from_frame(frame)?),
            "GET" => Command::Get(Get::parse_from_frame(frame)?),
            "DEL" => Command::Del(Del::parse_from_frame(frame)?),
            _ => Command::Unknown(Unknown::parse_from_frame(frame)?),
        };
        Ok(command)
    }
}