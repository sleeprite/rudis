use anyhow::Error;

use crate::{
    cmds::{
        connect::{auth::Auth, client::Client, echo::Echo, ping::Ping, select::Select},         hash::{
            hdel::Hdel, hexists::Hexists, hget::Hget, hgetall::Hgetall, hincrby::Hincrby, hincrbyfloat::HincrbyFloat, hkeys::Hkeys, hlen::Hlen,
            hmget::Hmget, hmset::Hmset, hset::Hset, hsetnx::Hsetnx, hstrlen::Hstrlen, hvals::Hvals, hscan::Hscan,
        }, key::{
            del::Del, exists::Exists, expire::Expire, expireat::ExpireAt, keys::Keys, r#move::Move, persist::Persist, pexpire::Pexpire, pexpireat::PexpireAt, pttl::Pttl, randomkey::RandomKey, rename::Rename, renamenx::Renamenx, scan::Scan, ttl::Ttl, r#type::Type
        }, listing::{
            blpop::Blpop, brpop::Brpop, lindex::Lindex, llen::Llen, lpop::Lpop, lpush::Lpush, lpushx::Lpushx, lrange::Lrange,
            lrem::Lrem, lset::Lset, ltrim::Ltrim, rpop::Rpop, rpush::Rpush, rpushx::Rpushx,
        }, server::{bgsave::Bgsave, dbsize::Dbsize, flushall::Flushall, flushdb::Flushdb, info::Info, save::Save}, server_sync::{psync::Psync, replconf::Replconf}, set::{
            sadd::Sadd, scard::Scard, sdiff::Sdiff, sinter::Sinter, sismember::Sismember, smembers::Smembers, spop::Spop, srem::Srem, sscan::Sscan, sunion::Sunion, sunionstore::Sunionstore, srandmember::Srandmember, sdiffstore::Sdiffstore, sinterstore::Sinterstore, smove::Smove
        }, sorted_set::{
            zadd::Zadd, zcard::Zcard, zcount::Zcount, zincrby::Zincrby, zlexcount::Zlexcount, zrank::Zrank, zrem::Zrem, zscore::Zscore, zrange::Zrange,
            zrevrange::Zrevrange, zrevrank::Zrevrank,
            zrangebyscore::Zrangebyscore, zrevrangebyscore::Zrevrangebyscore,
            zrangebylex::Zrangebylex, zrevrangebylex::Zrevrangebylex,
            zremrangebyrank::Zremrangebyrank, zremrangebyscore::Zremrangebyscore, zremrangebylex::Zremrangebylex,
            zpopmin::Zpopmin, zpopmax::Zpopmax,
            zscan::Zscan,
            zunionstore::Zunionstore, zinterstore::Zinterstore,
        }, string::{
            append::Append, decr::Decr, decrby::Decrby, get::Get, getrange::GetRange, getset::GetSet, incr::Incr, incrby::Incrby, incrbyfloat::IncrbyFloat, mget::Mget, mset::Mset, msetnx::Msetnx, set::Set, setrange::SetRange, strlen::Strlen, setex::Setex, psetex::Psetex, setnx::Setnx, setbit::Setbit, getbit::Getbit, bitcount::Bitcount, bitop::Bitop
        }, transaction::{
            discard::Discard, exec::Exec, multi::Multi
        },         hyperloglog::{
            pfadd::Pfadd, pfcount::Pfcount, pfmerge::Pfmerge
        },         geo::{
            geoadd::Geoadd, geoadd_json::GeoaddJson, geodist::Geodist, geohash::Geohash,
            geopos::Geopos, georadius::Georadius, georadiusbymember::Georadiusbymember,
        }, unknown::Unknown
    },
    frame::Frame,
};
// 命令
pub enum Command {
    Auth(Auth),
    Append(Append),
    Client(Client),
    Dbsize(Dbsize),
    Expire(Expire),
    Del(Del),
    Keys(Keys),
    Flushdb(Flushdb),
    Get(Get),
    GetRange(GetRange),
    Ping(Ping),
    Pttl(Pttl),
    Scan(Scan),
    Select(Select),
    Set(Set),
    SetRange(SetRange),
    Ttl(Ttl),
    Unknown(Unknown),
    Mset(Mset),
    Mget(Mget),
    Msetnx(Msetnx),
    Strlen(Strlen),
    Setex(Setex),
    Psetex(Psetex),
    Setnx(Setnx),
    Setbit(Setbit),
    Getbit(Getbit),
    Bitcount(Bitcount),
    Bitop(Bitop),
    Sunionstore(Sunionstore),
    Renamenx(Renamenx),
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
    Hincrby(Hincrby),
    HincrbyFloat(HincrbyFloat),
    Hkeys(Hkeys),
    Lindex(Lindex),
    Persist(Persist),
    Rpop(Rpop),
    Lpop(Lpop),
    Llen(Llen),
    Hvals(Hvals),
    Hscan(Hscan),
    Rpush(Rpush),
    Lpush(Lpush),
    Sadd(Sadd),
    Sismember(Sismember),
    Smembers(Smembers),
    Scard(Scard),
    Sdiff(Sdiff),
    Sinter(Sinter),
    Spop(Spop),
    Srem(Srem),
    Sdiffstore(Sdiffstore),
    Sinterstore(Sinterstore),
    Smove(Smove),
    Srandmember(Srandmember),
    Flushall(Flushall),
    Lpushx(Lpushx),
    Rpushx(Rpushx),
    Decr(Decr),
    Incr(Incr),
    IncrbyFloat(IncrbyFloat),
    Lset(Lset),
    Ltrim(Ltrim),
    Lrem(Lrem),
    Sunion(Sunion),
    Zcount(Zcount),
    Zadd(Zadd),
    Zincrby(Zincrby),
    Zlexcount(Zlexcount),
    Zscore(Zscore),
    Zcard(Zcard),
    Zrank(Zrank),
    Zrem(Zrem),
    Zrange(Zrange),
    Zrevrange(Zrevrange),
    Zrevrank(Zrevrank),
    Zrangebyscore(Zrangebyscore),
    Zrevrangebyscore(Zrevrangebyscore),
    Zrangebylex(Zrangebylex),
    Zrevrangebylex(Zrevrangebylex),
    Zremrangebyrank(Zremrangebyrank),
    Zremrangebyscore(Zremrangebyscore),
    Zremrangebylex(Zremrangebylex),
    Zpopmin(Zpopmin),
    Zpopmax(Zpopmax),
    Zscan(Zscan),
    Zunionstore(Zunionstore),
    Zinterstore(Zinterstore),
    Incrby(Incrby),
    Decrby(Decrby),
    Echo(Echo),
    ExpireAt(ExpireAt),
    RandomKey(RandomKey),
    PexpireAt(PexpireAt),
    Pexpire(Pexpire),
    Lrange(Lrange),
    Replconf(Replconf),
    Psync(Psync),
    Bgsave(Bgsave),
    Save(Save),
    GetSet(GetSet),
    Info(Info),
    Move(Move),
    Sscan(Sscan),
    // 阻塞列表命令
    Blpop(Blpop),
    Brpop(Brpop),
    // 事务命令
    Multi(Multi),
    Discard(Discard),
    Exec(Exec),
    // HyperLogLog 命令
    Pfadd(Pfadd),
    Pfcount(Pfcount),
    Pfmerge(Pfmerge),
    // GEO 命令
    Geoadd(Geoadd),
    GeoaddJson(GeoaddJson),
    Georadiusbymember(Georadiusbymember),
    Geodist(Geodist),
    Geohash(Geohash),
    Georadius(Georadius),
    Geopos(Geopos),
}
impl Command {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let command_name = frame.get_arg(0).unwrap();
        let command = match command_name.to_uppercase().as_str() {
            "AUTH" => Command::Auth(Auth::parse_from_frame(frame)?),
            "DEL" => Command::Del(Del::parse_from_frame(frame)?),
            "EXPIRE" => Command::Expire(Expire::parse_from_frame(frame)?),
            "FLUSHALL" => Command::Flushall(Flushall::parse_from_frame(frame)?),
            "FLUSHDB" => Command::Flushdb(Flushdb::parse_from_frame(frame)?),
            "GETRANGE" => Command::GetRange(GetRange::parse_from_frame(frame)?),
            "GET" => Command::Get(Get::parse_from_frame(frame)?),
            "PING" => Command::Ping(Ping::parse_from_frame(frame)?),
            "PTTL" => Command::Pttl(Pttl::parse_from_frame(frame)?),
            "TYPE" => Command::Type(Type::parse_from_frame(frame)?),
            "SELECT" => Command::Select(Select::parse_from_frame(frame)?),
            "SET" => Command::Set(Set::parse_from_frame(frame)?),
            "SETRANGE" => Command::SetRange(SetRange::parse_from_frame(frame)?),
            "TTL" => Command::Ttl(Ttl::parse_from_frame(frame)?),
            "RANDOMKEY" => Command::RandomKey(RandomKey::parse_from_frame(frame)?),
            "RENAME" => Command::Rename(Rename::parse_from_frame(frame)?),
            "EXISTS" => Command::Exists(Exists::parse_from_frame(frame)?),
            "STRLEN" => Command::Strlen(Strlen::parse_from_frame(frame)?),
            "MSET" => Command::Mset(Mset::parse_from_frame(frame)?),
            "MGET" => Command::Mget(Mget::parse_from_frame(frame)?),
            "MSETNX" => Command::Msetnx(Msetnx::parse_from_frame(frame)?),
            "APPEND" => Command::Append(Append::parse_from_frame(frame)?),
            "DBSIZE" => Command::Dbsize(Dbsize::parse_from_frame(frame)?),
            "SETEX" => Command::Setex(Setex::parse_from_frame(frame)?),
            "PSETEX" => Command::Psetex(Psetex::parse_from_frame(frame)?),
            "SETNX" => Command::Setnx(Setnx::parse_from_frame(frame)?),
            "SETBIT" => Command::Setbit(Setbit::parse_from_frame(frame)?),
            "GETBIT" => Command::Getbit(Getbit::parse_from_frame(frame)?),
            "BITCOUNT" => Command::Bitcount(Bitcount::parse_from_frame(frame)?),
            "BITOP" => Command::Bitop(Bitop::parse_from_frame(frame)?),
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
            "HSCAN" => Command::Hscan(Hscan::parse_from_frame(frame)?),
            "HINCRBY" => Command::Hincrby(Hincrby::parse_from_frame(frame)?),
            "HINCRBYFLOAT" => Command::HincrbyFloat(HincrbyFloat::parse_from_frame(frame)?),
            "RPUSH" => Command::Rpush(Rpush::parse_from_frame(frame)?),
            "LPUSH" => Command::Lpush(Lpush::parse_from_frame(frame)?),
            "SADD" => Command::Sadd(Sadd::parse_from_frame(frame)?),
            "SCARD" => Command::Scard(Scard::parse_from_frame(frame)?),
            "RENAMENX" => Command::Renamenx(Renamenx::parse_from_frame(frame)?),
            "EXPIREAT" => Command::ExpireAt(ExpireAt::parse_from_frame(frame)?),
            "SUNIONSTORE" => Command::Sunionstore(Sunionstore::parse_from_frame(frame)?),
            "SISMEMBER" => Command::Sismember(Sismember::parse_from_frame(frame)?),
            "SMEMBERS" => Command::Smembers(Smembers::parse_from_frame(frame)?),
            "SPOP" => Command::Spop(Spop::parse_from_frame(frame)?),
            "SREM" => Command::Srem(Srem::parse_from_frame(frame)?),
            "SDIFFSTORE" => Command::Sdiffstore(Sdiffstore::parse_from_frame(frame)?),
            "SINTERSTORE" => Command::Sinterstore(Sinterstore::parse_from_frame(frame)?),
            "SMOVE" => Command::Smove(Smove::parse_from_frame(frame)?),
            "SRANDMEMBER" => Command::Srandmember(Srandmember::parse_from_frame(frame)?),
            "LPUSHX" => Command::Lpushx(Lpushx::parse_from_frame(frame)?),
            "RPUSHX" => Command::Rpushx(Rpushx::parse_from_frame(frame)?),
            "INCR" => Command::Incr(Incr::parse_from_frame(frame)?),
            "DECR" => Command::Decr(Decr::parse_from_frame(frame)?),
            "LSET" => Command::Lset(Lset::parse_from_frame(frame)?),
            "LTRIM" => Command::Ltrim(Ltrim::parse_from_frame(frame)?),
            "LREM" => Command::Lrem(Lrem::parse_from_frame(frame)?),
            "SUNION" => Command::Sunion(Sunion::parse_from_frame(frame)?),
            "ZCOUNT" => Command::Zcount(Zcount::parse_from_frame(frame)?),
            "ZADD" => Command::Zadd(Zadd::parse_from_frame(frame)?),
            "ZINCRBY" => Command::Zincrby(Zincrby::parse_from_frame(frame)?),
            "ZCARD" => Command::Zcard(Zcard::parse_from_frame(frame)?),
            "ZSCORE" => Command::Zscore(Zscore::parse_from_frame(frame)?),
            "ZREM" => Command::Zrem(Zrem::parse_from_frame(frame)?),
            "SDIFF" => Command::Sdiff(Sdiff::parse_from_frame(frame)?),
            "SINTER" => Command::Sinter(Sinter::parse_from_frame(frame)?),
            "ZRANK" => Command::Zrank(Zrank::parse_from_frame(frame)?),
            "ZLEXCOUNT" => Command::Zlexcount(Zlexcount::parse_from_frame(frame)?),
            "ZRANGE" => Command::Zrange(Zrange::parse_from_frame(frame)?),
            "ZREVRANGE" => Command::Zrevrange(Zrevrange::parse_from_frame(frame)?),
            "ZREVRANK" => Command::Zrevrank(Zrevrank::parse_from_frame(frame)?),
            "ZRANGEBYSCORE" => Command::Zrangebyscore(Zrangebyscore::parse_from_frame(frame)?),
            "ZREVRANGEBYSCORE" => Command::Zrevrangebyscore(Zrevrangebyscore::parse_from_frame(frame)?),
            "ZRANGEBYLEX" => Command::Zrangebylex(Zrangebylex::parse_from_frame(frame)?),
            "ZREVRANGEBYLEX" => Command::Zrevrangebylex(Zrevrangebylex::parse_from_frame(frame)?),
            "ZREMRANGEBYRANK" => Command::Zremrangebyrank(Zremrangebyrank::parse_from_frame(frame)?),
            "ZREMRANGEBYSCORE" => Command::Zremrangebyscore(Zremrangebyscore::parse_from_frame(frame)?),
            "ZREMRANGEBYLEX" => Command::Zremrangebylex(Zremrangebylex::parse_from_frame(frame)?),
            "ZPOPMIN" => Command::Zpopmin(Zpopmin::parse_from_frame(frame)?),
            "ZPOPMAX" => Command::Zpopmax(Zpopmax::parse_from_frame(frame)?),
            "ZSCAN" => Command::Zscan(Zscan::parse_from_frame(frame)?),
            "ZUNIONSTORE" => Command::Zunionstore(Zunionstore::parse_from_frame(frame)?),
            "ZINTERSTORE" => Command::Zinterstore(Zinterstore::parse_from_frame(frame)?),
            "INCRBY" => Command::Incrby(Incrby::parse_from_frame(frame)?),
            "INCRBYFLOAT" => Command::IncrbyFloat(IncrbyFloat::parse_from_frame(frame)?),
            "DECRBY" => Command::Decrby(Decrby::parse_from_frame(frame)?),
            "ECHO" => Command::Echo(Echo::parse_from_frame(frame)?),
            "PEXPIRE" => Command::Pexpire(Pexpire::parse_from_frame(frame)?),
            "PEXPIREAT" => Command::PexpireAt(PexpireAt::parse_from_frame(frame)?),
            "REPLCONF" => Command::Replconf(Replconf::parse_from_frame(frame)?),
            "LRANGE" => Command::Lrange(Lrange::parse_from_frame(frame)?),
            "PSYNC" => Command::Psync(Psync::parse_from_frame(frame)?),
            "GETSET" => Command::GetSet(GetSet::parse_from_frame(frame)?),
            "CLIENT" => Command::Client(Client::parse_from_frame(frame)?),
            "INFO" => Command::Info(Info::parse_from_frame(frame)?),
            "MOVE" => Command::Move(Move::parse_from_frame(frame)?),
            "MULTI" => Command::Multi(Multi::parse_from_frame(frame)?),
            "EXEC" => Command::Exec(Exec::parse_from_frame(frame)?),
            "DISCARD" => Command::Discard(Discard::parse_from_frame(frame)?),
            "SCAN" => Command::Scan(Scan::parse_from_frame(frame)?),
            "SSCAN" => Command::Sscan(Sscan::parse_from_frame(frame)?),
            "PFADD" => Command::Pfadd(Pfadd::parse_from_frame(frame)?),
            "PFCOUNT" => Command::Pfcount(Pfcount::parse_from_frame(frame)?),
            "PFMERGE" => Command::Pfmerge(Pfmerge::parse_from_frame(frame)?),
            "GEOADDJSON" => Command::GeoaddJson(GeoaddJson::parse_from_frame(frame)?),
            "GEOADD" => Command::Geoadd(Geoadd::parse_from_frame(frame)?),
            "GEOPOS" => Command::Geopos(Geopos::parse_from_frame(frame)?),
            "GEODIST" => Command::Geodist(Geodist::parse_from_frame(frame)?),
            "GEOHASH" => Command::Geohash(Geohash::parse_from_frame(frame)?),
            "GEORADIUS" => Command::Georadius(Georadius::parse_from_frame(frame)?),
            "GEORADIUSBYMEMBER" => Command::Georadiusbymember(Georadiusbymember::parse_from_frame(frame)?),
            "BLPOP" => Command::Blpop(Blpop::parse_from_frame(frame)?),
            "BRPOP" => Command::Brpop(Brpop::parse_from_frame(frame)?),
            _ => Command::Unknown(Unknown::parse_from_frame(frame)?),
        };
        Ok(command)
    }
    pub fn propagate_aof_if_needed(&self) -> bool {
        match self {
            Command::Del(_) |
            Command::Expire(_) |
            Command::ExpireAt(_) |
            Command::Persist(_) |
            Command::Pexpire(_) |
            Command::PexpireAt(_) |
            Command::Rename(_) |
            Command::Renamenx(_) |
            Command::Append(_) |
            Command::Decr(_) |
            Command::Decrby(_) |
            Command::GetSet(_) |
            Command::Incr(_) |
            Command::Incrby(_) |
            Command::IncrbyFloat(_) |
            Command::Mset(_) |
            Command::Msetnx(_) |
            Command::Set(_) | 
            Command::SetRange(_) |
            Command::Setex(_) |
            Command::Psetex(_) |
            Command::Setnx(_) |
            Command::Setbit(_) |
            Command::Bitop(_) |
            Command::Flushall(_) |
            Command::Flushdb(_) |
            Command::Hdel(_) |
            Command::Hmset(_) |
            Command::Hset(_) |
            Command::Hincrby(_) |
            Command::HincrbyFloat(_) |
            Command::Hsetnx(_) |
            Command::Lpop(_) |
            Command::Lpush(_) |
            Command::Lpushx(_) |
            Command::Lset(_) |
            Command::Ltrim(_) |
            Command::Lrem(_) |
            Command::Rpop(_) |
            Command::Rpush(_) |
            Command::Rpushx(_) |
            Command::Sadd(_) |
            Command::Sdiff(_) |
            Command::Sinter(_) |
            Command::Spop(_) |
            Command::Srem(_) |
            Command::Sdiffstore(_) |
            Command::Sinterstore(_) |
            Command::Smove(_) |
            Command::Sunionstore(_) |
            Command::Zadd(_) |
            Command::Zincrby(_) |
            Command::Zrem(_) |  // 命令兼容：ZREM 可删除 Geo 成员
            Command::Zremrangebyrank(_) |
            Command::Zremrangebyscore(_) |
            Command::Zremrangebylex(_) |
            Command::Zpopmin(_) |
            Command::Zpopmax(_) |
            Command::Zunionstore(_) |
            Command::Zinterstore(_) |
            Command::Move(_) |
            Command::Pfadd(_) |
            Command::Pfmerge(_) |
            Command::Geoadd(_) |
            Command::GeoaddJson(_) |
            _ => false,
        }
    }
}