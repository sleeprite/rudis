use anyhow::Error;

use crate::{cmd::{auth::Auth, del::Del, expire::Expire, flushdb::Flushdb, ping::Ping, pttl::Pttl, select::Select, string::{get::Get, set::Set}, ttl::Ttl, unknown::Unknown}, frame::Frame};

// 命令
pub enum Command {
    Pttl(Pttl),
    Set(Set),
    Get(Get),
    Del(Del),
    Ttl(Ttl),
    Expire(Expire),
    Select(Select),
    Unknown(Unknown),
    Flushdb(Flushdb),
    Ping(Ping),
    Auth(Auth),
}

impl Command {
    
    pub fn parse_from_frame(frame: Frame)  -> Result<Self, Error>  {
        let command_name = frame.get_arg(0).unwrap();
        let command = match command_name.to_uppercase().as_str() {
            "SET" => Command::Set(Set::parse_from_frame(frame)?),
            "GET" => Command::Get(Get::parse_from_frame(frame)?),
            "DEL" => Command::Del(Del::parse_from_frame(frame)?),
            "SELECT" => Command::Select(Select::parse_from_frame(frame)?),
            "FLUSHDB" => Command::Flushdb(Flushdb::parse_from_frame(frame)?),
            "EXPIRE" => Command::Expire(Expire::parse_from_frame(frame)?),
            "PTTL" => Command::Pttl(Pttl::parse_from_frame(frame)?),
            "PING" => Command::Ping(Ping::parse_from_frame(frame)?),
            "TTL" => Command::Ttl(Ttl::parse_from_frame(frame)?),
            _ => Command::Unknown(Unknown::parse_from_frame(frame)?),
        };
        Ok(command)
    }

}