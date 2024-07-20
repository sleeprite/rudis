use std::{
    collections::HashMap,
    io::Write,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::{
    db::db::Redis,
    interface::command_strategy::CommandStrategy,
    interface::command_type::CommandType,
    session::session::Session,
    tools::resp::RespValue,
    RedisConfig,
};

pub struct ZaddCommand {}

impl CommandStrategy for ZaddCommand {
    fn execute(
        &self,
        stream: Option<&mut TcpStream>,
        fragments: &[&str],
        redis: &Arc<Mutex<Redis>>,
        _redis_config: &Arc<RedisConfig>,
        sessions: &Arc<Mutex<HashMap<String, Session>>>,
        session_id: &str,
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
        let score: usize = fragments[6].parse().unwrap();
        let value = fragments[8].to_string();
        
        redis_ref.check_ttl(db_index, &key);
        
        match redis_ref.zadd(db_index, key.clone(), value, score) {
            Ok(result) => {
                if let Some(stream) = stream {
                    let response_bytes = &RespValue::Integer(result as i64).to_bytes();
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
            Err(err_msg) => {
                if let Some(stream) = stream {
                    let response_bytes = &RespValue::Error(err_msg.to_string()).to_bytes();
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
        }
    }

    fn command_type(&self) -> CommandType {
        CommandType::Write
    }
}