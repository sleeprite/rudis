use std::collections::HashMap;


use crate::{command::{arr::{lindex::LindexCommand, llen::LlenCommand, lpop::LpopCommand, lpush::LpushCommand, lrange::LrangeCommand, rpop::RpopCommand, rpush::RpushCommand}, auth::AuthCommand, dbsize::DBSizeCommand, echo::EchoCommand, flushall::FlushAllCommand, flushdb::FlushDbCommand, hash::{hdel::HdelCommand, hexists::HexistsCommand, hget::HgetCommand, hmset::HmsetCommand, hset::HsetCommand}, key::{del::DelCommand, exists::ExistsCommand, expire::ExpireCommand, keys::KeysCommand, r#move::MoveCommand, pexpire::PexpireCommand, pttl::PttlCommand, rename::RenameCommand, ttl::TtlCommand, r#type::TypeCommand}, select::SelectCommand, set::{sadd::SaddCommand, scard::ScardCommand, smembers::SmembersCommand}, string::{append::AppendCommand, decr::DecrCommand, get::GetCommand, incr::IncrCommand, mset::MsetCommand, set::SetCommand}, zset::{zadd::ZaddCommand, zcard::ZcardCommand, zcount::ZcountCommand, zscore::ZscoreCommand}}, interface::command_strategy::CommandStrategy};

pub fn init_commands() -> HashMap<&'static str, Box<dyn CommandStrategy>> {
    let mut strategies: HashMap<&'static str, Box<dyn CommandStrategy>> = HashMap::new();

    strategies.insert("ECHO", Box::new(EchoCommand {}));
    strategies.insert("SET", Box::new(SetCommand {}));
    strategies.insert("GET", Box::new(GetCommand {}));
    strategies.insert("DEL", Box::new(DelCommand {}));
    strategies.insert("EXISTS", Box::new(ExistsCommand {}));
    strategies.insert("EXPIRE", Box::new(ExpireCommand {}));
    strategies.insert("RENAME", Box::new(RenameCommand {}));
    strategies.insert("DBSIZE", Box::new(DBSizeCommand {}));
    strategies.insert("FLUSHALL", Box::new(FlushAllCommand {}));
    strategies.insert("FLUSHDB", Box::new(FlushDbCommand {}));
    strategies.insert("SELECT", Box::new(SelectCommand {}));
    strategies.insert("AUTH", Box::new(AuthCommand {}));
    strategies.insert("LLEN", Box::new(LlenCommand {}));
    strategies.insert("MOVE", Box::new(MoveCommand {}));
    strategies.insert("KEYS", Box::new(KeysCommand {}));
    strategies.insert("APPEND", Box::new(AppendCommand {}));
    strategies.insert("LPUSH", Box::new(LpushCommand {}));
    strategies.insert("RPUSH", Box::new(RpushCommand {}));
    strategies.insert("LINDEX", Box::new(LindexCommand {}));
    strategies.insert("LPOP", Box::new(LpopCommand {}));
    strategies.insert("RPOP", Box::new(RpopCommand {}));
    strategies.insert("INCR", Box::new(IncrCommand {}));
    strategies.insert("DECR", Box::new(DecrCommand {}));
    strategies.insert("PTTL", Box::new(PttlCommand {}));
    strategies.insert("TYPE", Box::new(TypeCommand {}));
    strategies.insert("SADD", Box::new(SaddCommand {}));
    strategies.insert("SMEMBERS", Box::new(SmembersCommand {}));
    strategies.insert("LRANGE", Box::new(LrangeCommand {}));
    strategies.insert("SCARD", Box::new(ScardCommand {}));
    strategies.insert("TTL", Box::new(TtlCommand {}));
    strategies.insert("HMSET", Box::new(HmsetCommand {}));
    strategies.insert("HGET", Box::new(HgetCommand {}));
    strategies.insert("HDEL", Box::new(HdelCommand {}));
    strategies.insert("HEXISTS", Box::new(HexistsCommand {}));
    strategies.insert("HSET", Box::new(HsetCommand {}));
    strategies.insert("ZADD", Box::new(ZaddCommand {}));
    strategies.insert("ZCOUNT", Box::new(ZcountCommand {}));
    strategies.insert("ZCARD", Box::new(ZcardCommand {}));
    strategies.insert("PEXPIRE", Box::new(PexpireCommand {}));
    strategies.insert("ZSCORE", Box::new(ZscoreCommand {}));
    strategies.insert("MSET", Box::new(MsetCommand {}));

    strategies
}