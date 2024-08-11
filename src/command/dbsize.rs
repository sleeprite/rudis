
use std::{collections::HashMap, net::TcpStream, sync::{Arc, Mutex}};
use std::io::Write;

use crate::{db::db::Redis, interface::command_type::CommandType, session::session::Session, tools::resp::RespValue, RudisConfig};
use crate::interface::command_strategy::CommandStrategy;

/*
 * DBSize 命令
 */
pub struct DBSizeCommand {}

impl CommandStrategy for DBSizeCommand {
    fn execute(
        &self,
        stream: Option<&mut TcpStream>,
        _fragments: &[&str],
        redis: &Arc<Mutex<Redis>>,
        _rudis_config: &Arc<RudisConfig>,
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
        
        redis_ref.check_all_ttl(db_index);
        let db_size = redis_ref.dbsize(db_index);

        if let Some(stream) = stream { 
            let response_bytes = &RespValue::Integer(db_size as i64).to_bytes();
            match stream.write(response_bytes) {
                Ok(_bytes_written) => {
                    // Response successful
                },
                Err(e) => {
                    eprintln!("Failed to write to stream: {}", e);
                },
            };
        }
    }
     
    fn command_type(&self) -> crate::interface::command_type::CommandType {
        CommandType::Read
    }
}