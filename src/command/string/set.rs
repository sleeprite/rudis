use std::io::Write;
use std::{
    collections::HashMap,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::interface::command_strategy::CommandStrategy;
use crate::interface::command_type::CommandType;
use crate::tools::resp::RespValue;
use crate::{
    db::db::Redis, session::session::Session,
    tools::date::current_millis, RedisConfig,
};

/*
 * Set 命令
 */
pub struct SetCommand {}

impl CommandStrategy for SetCommand {
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
        let value = fragments[6].to_string();
        if fragments.len() > 8 {
            if let Some(ttl) = fragments.get(10).and_then(|t| t.parse::<i64>().ok()) {
                let ttl_millis = match fragments[8].to_uppercase().as_str() {
                    "EX" => ttl * 1000,
                    _ => ttl
                };
                let expire_at = current_millis() + ttl_millis;
                redis_ref.set_with_ttl(db_index, key.clone(), value.clone(), expire_at);
            }
        } else {
            redis_ref.set(db_index, key, value);
        }

        if let Some(stream) = stream { 
            let response_bytes = &RespValue::Ok.to_bytes();
            stream.write(response_bytes).unwrap();
        }
    }

        
    fn command_type(&self) -> crate::interface::command_type::CommandType {
        return CommandType::Write;
    }
}