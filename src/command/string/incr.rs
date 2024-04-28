
use std::{collections::HashMap, net::TcpStream, sync::{Arc, Mutex}};
use std::io::Write;

use crate::{db::db::Redis, interface::command_type::CommandType, session::session::Session, tools::resp::RespValue, RedisConfig};
use crate::interface::command_strategy::CommandStrategy;

/*
 * Incr 命令
 */

pub struct IncrCommand {}

impl CommandStrategy for IncrCommand {
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
        
        match redis_ref.incr(db_index, key, 1) {
            Ok(result) => {
                if let Some(stream) = stream { 
                    let response_value = RespValue::Integer(result).to_bytes();
                    stream.write(&response_value).unwrap();
                }
            }
            Err(err) => {
                if let Some(stream) = stream { 
                    let response_value = RespValue::Error(err).to_bytes();
                    stream.write(&response_value).unwrap();
                }
            }
        }
    }

        
    fn command_type(&self) -> crate::interface::command_type::CommandType {
        return CommandType::Write;
    }
}