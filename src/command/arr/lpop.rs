use std::io::Write;
use std::{
    collections::HashMap,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::session::session::Session;
use crate::tools::reponse::RespValue;
use crate::{command_strategy::CommandStrategy, db::db::Redis, RedisConfig};

pub struct LpopCommand {}

impl CommandStrategy for LpopCommand {
    fn execute(
        &self,
        stream: &mut TcpStream,
        fragments: &Vec<&str>,
        redis: &Arc<Mutex<Redis>>,
        _redis_config: &Arc<RedisConfig>,
        sessions: &Arc<Mutex<HashMap<String, Session>>>,
    ) {
        let mut redis_ref = redis.lock().unwrap();

        let db_index = {
            let sessions_ref = sessions.lock().unwrap();
            if let Some(session) = sessions_ref.get(&stream.peer_addr().unwrap().to_string()) {
                session.get_selected_database()
            } else {
                return;
            }
        };

        let key = fragments[4].to_string();
        let value = match redis_ref.lpop(db_index, key.clone()) {
            Some(v) => v,
            None => return, // If key does not exist or list is empty, return early
        };

        let response_bytes = &RespValue::BulkString(value).to_bytes();
        stream.write(response_bytes).unwrap();
    }
}