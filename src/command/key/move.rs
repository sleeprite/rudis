use std::{collections::HashMap, net::TcpStream, sync::{Arc, Mutex}};
use std::io::Write;
use crate::{db::db::Redis, interface::command_type::CommandType, session::session::Session, tools::resp::RespValue, RedisConfig};
use crate::interface::command_strategy::CommandStrategy;
/*
 * Move 命令
 */
pub struct MoveCommand {}

impl CommandStrategy for MoveCommand {
    fn execute(
        &self,
        stream: Option<&mut TcpStream>,
        fragments: &[&str],
        redis: &Arc<Mutex<Redis>>,
        _redis_config: &Arc<RedisConfig>,
        sessions: &Arc<Mutex<HashMap<String, Session>>>,
        session_id: &str
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
        let dest_db_index: usize = fragments[6].parse().unwrap();

        redis_ref.check_ttl(db_index, &key);

        let move_result = redis_ref.move_key(db_index, &key, dest_db_index);

        if move_result {
            if let Some(stream) = stream {  
                let response_bytes = &RespValue::Integer(1).to_bytes();
                stream.write(response_bytes).unwrap();
            }
        } else if let Some(stream) = stream { 
            let response_bytes = &RespValue::Integer(0).to_bytes();
            stream.write(response_bytes).unwrap();
        } 
        
    }

    fn command_type(&self) -> crate::interface::command_type::CommandType {
        CommandType::Write
    }
}