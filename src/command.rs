use anyhow::Error;

use crate::{
    cmd::{
        dbsize::Dbsize, 
        select::Select, 
        ping::Ping, 
        auth::Auth, 
        flushdb::Flushdb, 
        unknown::Unknown,
        string::{ 
            append::Append, 
            get::Get,
            mget::Mget, 
            mset::Mset, 
            set::Set, 
            strlen::Strlen
        },
        list::{
            lindex::Lindex, 
            llen::Llen, 
            lpop::Lpop, 
            lpush::Lpush, 
            rpop::Rpop, 
            rpush::Rpush
        }, 
        key::{
            del::Del, 
            exists::Exists, 
            expire::Expire, 
            keys::Keys, 
            persist::Persist, 
            pttl::Pttl,
            rename::Rename, 
            ttl::Ttl, 
            r#type::Type
        }, 
        hash::{
            hdel::Hdel, 
            hexists::Hexists, 
            hget::Hget, 
            hgetall::Hgetall, 
            hkeys::Hkeys, 
            hlen::Hlen, 
            hmget::Hmget, 
            hmset::Hmset, 
            hset::Hset, 
            hsetnx::Hsetnx, 
            hstrlen::Hstrlen, 
            hvals::Hvals
        }, 
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
    Hsetnx(Hsetnx),
    Hgetall(Hgetall),
    Hkeys(Hkeys),
    Lindex(Lindex),
    Persist(Persist),
    Rpop(Rpop),
    Lpop(Lpop),
    Llen(Llen),
    Hvals(Hvals),
    Rpush(Rpush),
    Lpush(Lpush)
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
            "HGETALL" => Command::Hgetall(Hgetall::parse_from_frame(frame)?),
            "HSETNX" => Command::Hsetnx(Hsetnx::parse_from_frame(frame)?),
            "HKEYS" => Command::Hkeys(Hkeys::parse_from_frame(frame)?),
            "PERSIST" => Command::Persist(Persist::parse_from_frame(frame)?),
            "LINDEX" => Command::Lindex(Lindex::parse_from_frame(frame)?),
            "RPOP" => Command::Rpop(Rpop::parse_from_frame(frame)?),
            "LPOP" => Command::Lpop(Lpop::parse_from_frame(frame)?),
            "LLEN" => Command::Llen(Llen::parse_from_frame(frame)?),
            "HVALS" => Command::Hvals(Hvals::parse_from_frame(frame)?),
            "RPUSH" => Command::Rpush(Rpush::parse_from_frame(frame)?),
            "LPUSH" => Command::Lpush(Lpush::parse_from_frame(frame)?),
            _ => Command::Unknown(Unknown::parse_from_frame(frame)?),
        };
        Ok(command)
    }
}
