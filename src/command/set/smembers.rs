use std::io::Write;
use std::{
    collections::HashMap,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::interface::command_type::CommandType;
use crate::session::session::Session;
use crate::{db::db::Redis, RudisConfig};
use crate::interface::command_strategy::CommandStrategy;

pub struct SmembersCommand {}

impl CommandStrategy for SmembersCommand {
    fn execute(
        &self,
        stream: Option<&mut TcpStream>,
        fragments: &[&str],
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

        if let Some(key) = fragments.get(4) {
            redis_ref.check_all_ttl(db_index);
            if let Some(members) = redis_ref.smembers(db_index, key.as_ref()) {
                if let Some(stream) = stream { 
                    let response = format!("*{}\r\n", members.len());
                    match stream.write(response.as_bytes()) {
                        Ok(_bytes_written) => {
                            // Response successful
                        },
                        Err(e) => {
                            eprintln!("Failed to write to stream: {}", e);
                        },
                    };
                    for member in members {
                        let response = format!("${}\r\n{}\r\n", member.len(), member);
                        match stream.write(response.as_bytes()) {
                            Ok(_bytes_written) => {
                                // Response successful
                            },
                            Err(e) => {
                                eprintln!("Failed to write to stream: {}", e);
                            },
                        };
                    }
                }
            } else if let Some(stream) = stream { 
                let response = "*0\r\n".to_string();
                match stream.write(response.as_bytes()) {
                    Ok(_bytes_written) => {
                        // Response successful
                    },
                    Err(e) => {
                        eprintln!("Failed to write to stream: {}", e);
                    },
                };
            }
        } else if let Some(stream) = stream { 
            let response = "-ERR wrong number of arguments for 'smembers' command\r\n";
            match stream.write(response.as_bytes()) {
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