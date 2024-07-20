use std::{collections::HashMap, net::TcpStream, sync::{Arc, Mutex}};
use std::io::Write;
use crate::{db::db::Redis, interface::command_type::CommandType, session::session::Session, tools::resp::RespValue, RedisConfig};
use crate::interface::command_strategy::CommandStrategy;

pub struct TypeCommand {}

impl CommandStrategy for TypeCommand {
    fn execute(
        &self,
        stream: Option<&mut TcpStream>,
        fragments: &Vec<&str>,
        redis: &Arc<Mutex<Redis>>,
        _redis_config: &Arc<RedisConfig>,
        sessions: &Arc<Mutex<HashMap<String, Session>>>,
        session_id: &String
    ) {
        let mut redis_ref = redis.lock().unwrap();

        let db_index = {
            let sessions_ref = sessions.lock().unwrap();
            if let Some(session) = sessions_ref.get(session_id) {
                session.get_selected_database()
            } else {
                return;
            }
        };

        let key: String = fragments[4].to_string();

        redis_ref.check_ttl(db_index, &key);

        let key_type = redis_ref.key_type(db_index, key);

        if let Some(stream) = stream { 
            let response_bytes = &RespValue::SimpleString(key_type).to_bytes();
            stream.write(response_bytes).unwrap();
        }
    }

    
    fn command_type(&self) -> crate::interface::command_type::CommandType {
        CommandType::Read
    }
}