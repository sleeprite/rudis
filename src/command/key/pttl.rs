use std::{collections::HashMap, net::TcpStream, sync::{Arc, Mutex}};
use std::io::Write;
use crate::{command_strategy::CommandStrategy, db::db::Redis, session::session::Session, tools::reponse::RespValue, RedisConfig};

pub struct PttlCommand {}

impl CommandStrategy for PttlCommand {
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
        let ttl_millis = redis_ref.pttl(db_index, key);

        let response_bytes = &RespValue::Integer(ttl_millis).to_bytes();
        stream.write(response_bytes).unwrap();
    }
}