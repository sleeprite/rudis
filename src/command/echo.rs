use std::{collections::HashMap, net::TcpStream, sync::{Arc, Mutex}};
use std::io::Write;

use crate::{command_strategy::CommandStrategy, db::db::Redis, session::session::Session, tools::reponse::RespValue, RedisConfig};

/*
 * Echo 命令
 */
pub struct EchoCommand {}

impl CommandStrategy for EchoCommand {
    fn execute(
        &self,
        stream: &mut TcpStream,
        fragments: &Vec<&str>,
        _redis: &Arc<Mutex<Redis>>,
        _redis_config: &Arc<RedisConfig>,
        _sessions: &Arc<Mutex<HashMap<String, Session>>>,
    ) {
        let keyword = fragments[4].to_string();
        let response_bytes = &RespValue::BulkString(keyword).to_bytes();
        stream.write(response_bytes).unwrap();
    }
}