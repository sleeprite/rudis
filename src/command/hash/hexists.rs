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

pub struct HexistsCommand {}

impl CommandStrategy for HexistsCommand {
    fn execute(
        &self,
        stream: Option<&mut TcpStream>,
        fragments: &Vec<&str>,
        redis: &Arc<Mutex<Redis>>,
        _redis_config: &Arc<RedisConfig>,
        sessions: &Arc<Mutex<HashMap<String, Session>>>,
        session_id: &String,
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
        let field = fragments[6].to_string();

        redis_ref.check_ttl(db_index, &key);

        match redis_ref.hexists(db_index, &key, &field) {
            Ok(exists) => {
                if let Some(stream) = stream {
                    if exists {
                        stream.write(b":1\r\n").unwrap();
                    } else {
                        stream.write(b":0\r\n").unwrap();
                    }
                }
            }
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
