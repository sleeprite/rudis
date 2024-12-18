use anyhow::Error;

use crate::{cmd::{auth::Auth, del::Del, expire::Expire, pttl::Pttl, select::Select, string::{get::Get, set::Set}, ttl::Ttl, unknown::Unknown}, frame::Frame};

// 命令
pub enum Command {
    Set(Set),
    Get(Get),
    Del(Del),
    Ttl(Ttl),
    Pttl(Pttl),
    Unknown(Unknown),
    Expire(Expire),
    Select(Select),
    Auth(Auth),
}

impl Command {
    
    pub fn parse_from_frame(frame: Frame)  -> Result<Self, Error>  {
        let command_name = frame.get_arg(0).unwrap();
        let command = match command_name.to_uppercase().as_str() {
            "SET" => Command::Set(Set::parse_from_frame(frame)?),
            "GET" => Command::Get(Get::parse_from_frame(frame)?),
            "DEL" => Command::Del(Del::parse_from_frame(frame)?),
            "EXPIRE" => Command::Expire(Expire::parse_from_frame(frame)?),
            "SELECT" => Command::Select(Select::parse_from_frame(frame)?),
            "PTTL" => Command::Pttl(Pttl::parse_from_frame(frame)?),
            "TTL" => Command::Ttl(Ttl::parse_from_frame(frame)?),
            _ => Command::Unknown(Unknown::parse_from_frame(frame)?),
        };
        Ok(command)
    }

}