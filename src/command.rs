use anyhow::Error;

use crate::{
    cmd::{
        auth::Auth, dbsize::Dbsize, flushdb::Flushdb, hash::{hdel::Hdel, hexists::Hexists, hget::Hget, hlen::Hlen, hmget::Hmget, hmset::Hmset, hset::Hset, hsetnx::Hsetnx, hstrlen::Hstrlen}, key::{
            del::Del, exists::Exists, expire::Expire, keys::Keys, pttl::Pttl, rename::Rename, ttl::Ttl, r#type::Type
        }, ping::Ping, select::Select, string::{ append::Append, get::Get, mget::Mget, mset::Mset, set::Set, strlen::Strlen}, unknown::Unknown 
    }, frame::Frame
};

// 命令
pub enum Command {
    Auth(Auth),
    Append(Append),
    Dbsize(Dbsize),
    Del(Del),
    Expire(Expire),
    Keys(Keys),
    Flushdb(Flushdb),
    Get(Get),
    Ping(Ping),
    Pttl(Pttl),
    Select(Select),
    Set(Set),
    Ttl(Ttl),
    Unknown(Unknown),
    Mset(Mset),
    Mget(Mget),
    Strlen(Strlen),
    Rename(Rename),
    Exists(Exists),
    Hset(Hset),
    Hget(Hget),
    Type(Type),
    Hmset(Hmset),
    Hexists(Hexists),
    Hstrlen(Hstrlen), 
    Hmget(Hmget),
    Hdel(Hdel),
    Hlen(Hlen),
    Hsetnx(Hsetnx)
}

impl Command {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let command_name = frame.get_arg(0).unwrap();
        let command = match command_name.to_uppercase().as_str() {
            "AUTH" => Command::Auth(Auth::parse_from_frame(frame)?),
            "DEL" => Command::Del(Del::parse_from_frame(frame)?),
            "EXPIRE" => Command::Expire(Expire::parse_from_frame(frame)?),
            "FLUSHDB" => Command::Flushdb(Flushdb::parse_from_frame(frame)?),
            "GET" => Command::Get(Get::parse_from_frame(frame)?),
            "PING" => Command::Ping(Ping::parse_from_frame(frame)?),
            "PTTL" => Command::Pttl(Pttl::parse_from_frame(frame)?),
            "TYPE" => Command::Type(Type::parse_from_frame(frame)?),
            "SELECT" => Command::Select(Select::parse_from_frame(frame)?),
            "SET" => Command::Set(Set::parse_from_frame(frame)?),
            "TTL" => Command::Ttl(Ttl::parse_from_frame(frame)?),
            "RENAME" => Command::Rename(Rename::parse_from_frame(frame)?),
            "EXISTS" => Command::Exists(Exists::parse_from_frame(frame)?),
            "STRLEN" => Command::Strlen(Strlen::parse_from_frame(frame)?),
            "MSET" => Command::Mset(Mset::parse_from_frame(frame)?),
            "MGET" => Command::Mget(Mget::parse_from_frame(frame)?),
            "APPEND" => Command::Append(Append::parse_from_frame(frame)?),
            "DBSIZE" => Command::Dbsize(Dbsize::parse_from_frame(frame)?),
            "HSET" => Command::Hset(Hset::parse_from_frame(frame)?),
            "HGET" => Command::Hget(Hget::parse_from_frame(frame)?),
            "HMSET" => Command::Hmset(Hmset::parse_from_frame(frame)?),
            "HDEL" => Command::Hdel(Hdel::parse_from_frame(frame)?),
            "HEXISTS" => Command::Hexists(Hexists::parse_from_frame(frame)?),
            "HSTRLEN" => Command::Hstrlen(Hstrlen::parse_from_frame(frame)?),
            "KEYS" => Command::Keys(Keys::parse_from_frame(frame)?),
            "HMGET" => Command::Hmget(Hmget::parse_from_frame(frame)?),
            "HLEN" => Command::Hlen(Hlen::parse_from_frame(frame)?),
            "HSETNX" => Command::Hsetnx(Hsetnx::parse_from_frame(frame)?),
            _ => Command::Unknown(Unknown::parse_from_frame(frame)?),
        };
        Ok(command)
    }
}
