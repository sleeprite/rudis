
use std::{collections::HashMap, net::TcpStream, sync::{Arc, Mutex}};
use std::io::Write;

use crate::{db::db::Redis, interface::command_type::CommandType, session::session::Session, tools::resp::RespValue, RedisConfig};
use crate::interface::command_strategy::CommandStrategy;

/*
 * Append 命令
 */
pub struct AppendCommand {}

impl CommandStrategy for AppendCommand {
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

        // 检测是否过期
        redis_ref.check_ttl(db_index, &key);

        let len = match redis_ref.append(db_index, key, value) {
            Ok(len) => len as i64,
            Err(err) => {
                if let Some(stream) = stream { 
                    let response_value = RespValue::Error(err).to_bytes();
                    stream.write(&response_value).unwrap();
                }
                return;
            }
        };

        if let Some(stream) = stream { 
            let response_value = RespValue::Integer(len).to_bytes();
            stream.write(&response_value).unwrap();
        }
    }

        
    fn command_type(&self) -> crate::interface::command_type::CommandType {
        return CommandType::Write;
    }
}