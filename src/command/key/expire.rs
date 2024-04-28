use std::{collections::HashMap, net::TcpStream, sync::{Arc, Mutex}};
use std::io::Write;
use crate::{db::db::Redis, interface::command_type::CommandType, session::session::Session, tools::{date::current_millis, resp::RespValue}, RedisConfig};
use crate::interface::command_strategy::CommandStrategy;
pub struct ExpireCommand {}

impl CommandStrategy for ExpireCommand {
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

        let key = fragments[4].to_string();
        let ttl_millis = fragments[6].parse::<i64>().unwrap();
        let expire_at = current_millis() + ttl_millis;
        let result = redis_ref.expire(db_index, key, expire_at);

        if result {
            if let Some(stream) = stream { 
                let response_bytes = &RespValue::Integer(1).to_bytes();
                stream.write(response_bytes).unwrap();
            }
        } else {
            if let Some(stream) = stream { 
                let response_bytes = &RespValue::Integer(0).to_bytes();
                stream.write(response_bytes).unwrap();
            }
        }
    }

    fn command_type(&self) -> crate::interface::command_type::CommandType {
        return CommandType::Write;
    }
}