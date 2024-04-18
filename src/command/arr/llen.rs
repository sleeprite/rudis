use std::io::Write;
use std::{
    collections::HashMap,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::session::session::Session;
use crate::tools::reponse::RespValue;
use crate::{command_strategy::CommandStrategy, db::db::Redis, RedisConfig};

pub struct LlenCommand {}

impl CommandStrategy for LlenCommand {
    fn execute(
        &self,
        stream: &mut TcpStream,
        fragments: &Vec<&str>,
        redis: &Arc<Mutex<Redis>>,
        _redis_config: &Arc<RedisConfig>,
        sessions: &Arc<Mutex<HashMap<String, Session>>>,
    ) {
        let redis_ref = redis.lock().unwrap();

        let db_index = {
            let sessions_ref = sessions.lock().unwrap();
            if let Some(session) = sessions_ref.get(&stream.peer_addr().unwrap().to_string()) {
                session.get_selected_database()
            } else {
                return;
            }
        };
        let key = fragments[4].to_string(); 
        let len = redis_ref.llen(db_index, &key.clone());

        let response_bytes = &RespValue::Integer(len as i64).to_bytes();
        stream.write(response_bytes).unwrap();
    }
}