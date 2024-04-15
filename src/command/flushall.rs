use std::{collections::HashMap, net::TcpStream, sync::{Arc, Mutex}};
use std::io::Write;
use crate::{command_strategy::CommandStrategy, db::db::Redis, session::session::Session, RedisConfig};

/*
 * FlushAll 命令
 */
pub struct FlushAllCommand {}

impl CommandStrategy for FlushAllCommand {
    fn execute(
        &self,
        stream: &mut TcpStream,
        _fragments: &Vec<&str>,
        redis: &Arc<Mutex<Redis>>,
        _redis_config: &Arc<RedisConfig>,
        _sessions: &Arc<Mutex<HashMap<String, Session>>>,
    ) {
        let mut redis_ref = redis.lock().unwrap();
        redis_ref.flush_all();
        stream.write_all(b"+OK\r\n").unwrap();
    }
}