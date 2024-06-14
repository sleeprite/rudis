
use std::{collections::HashMap, net::TcpStream, sync::{Arc, Mutex}};
use std::io::Write;

use crate::{db::db::Redis, interface::command_type::CommandType, session::session::Session, tools::resp::RespValue, RedisConfig};
use crate::interface::command_strategy::CommandStrategy;

/*
 * Get 命令
 */
pub struct GetCommand {}

impl CommandStrategy for GetCommand {
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
        redis_ref.check_ttl(db_index, &key);

        match redis_ref.get(db_index, &key) {
            Ok(Some(value)) => {
                if let Some(stream) = stream { 
                    let response_bytes = &RespValue::BulkString(value.to_string()).to_bytes();
                    stream.write(response_bytes).unwrap();
                }
            },
            Ok(None) => {
                if let Some(stream) = stream { 
                    let response_bytes = &RespValue::Null.to_bytes();
                    stream.write(response_bytes).unwrap();
                }
            },
            Err(err_msg) => {
                if let Some(stream) = stream { 
                    let response_bytes = &RespValue::Error(err_msg.to_string()).to_bytes();
                    stream.write(response_bytes).unwrap();
                }
            }
        }
    }

        
    fn command_type(&self) -> crate::interface::command_type::CommandType {
        return CommandType::Read;
    }
}