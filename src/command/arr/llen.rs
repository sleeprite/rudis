use std::io::Write;
use std::{
    collections::HashMap,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::interface::command_strategy::CommandStrategy;
use crate::interface::command_type::CommandType;
use crate::session::session::Session;
use crate::tools::resp::RespValue;
use crate::{db::db::Redis, RedisConfig};

pub struct LlenCommand {}

impl CommandStrategy for LlenCommand {
    fn execute(
        &self,
        stream: Option<&mut TcpStream>,
        fragments: &Vec<&str>,
        redis: &Arc<Mutex<Redis>>,
        _redis_config: &Arc<RedisConfig>,
        sessions: &Arc<Mutex<HashMap<String, Session>>>,
        session_id: &String,
    ) {
        let redis_ref = redis.lock().unwrap();

        let db_index = {
            let sessions_ref = sessions.lock().unwrap();
            if let Some(session) = sessions_ref.get(session_id) {
                session.get_selected_database()
            } else {
                return;
            }
        };
        let key = fragments[4].to_string();
        let len = redis_ref.llen(db_index, &key.clone());
        if let Some(stream) = stream {
            let response_bytes = &RespValue::Integer(len as i64).to_bytes();
            stream.write(response_bytes).unwrap();
        }
    }

    fn command_type(&self) -> crate::interface::command_type::CommandType {
        return CommandType::Read;
    }
}
