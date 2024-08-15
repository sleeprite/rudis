
use std::{collections::HashMap, net::TcpStream, sync::{Arc, Mutex}};
use std::io::Write;

use crate::{db::db::Db, interface::command_type::CommandType, session::session::Session, tools::resp::RespValue, RudisConfig};
use crate::interface::command_strategy::CommandStrategy;

/*
 * Incr 命令
 */

pub struct IncrCommand {}

impl CommandStrategy for IncrCommand {
    fn execute(
        &self,
        stream: Option<&mut TcpStream>,
        fragments: &[&str],
        db: &Arc<Mutex<Db>>,
        _rudis_config: &Arc<RudisConfig>,
        sessions: &Arc<Mutex<HashMap<String, Session>>>,
        session_id: &str
    ) {
        let mut db_ref = db.lock().unwrap();

        let db_index = {
            let sessions_ref = sessions.lock().unwrap();
            if let Some(session) = sessions_ref.get(session_id) {
                session.get_selected_database()
            } else {
                return;
            }
        };

        let key = fragments[4].to_string();

        // 检测是否过期
        db_ref.check_ttl(db_index, &key);
        
        match db_ref.incr(db_index, key, 1) {
            Ok(result) => {
                if let Some(stream) = stream { 
                    let response_value = RespValue::Integer(result).to_bytes();
                    match stream.write(&response_value) {
                        Ok(_bytes_written) => {
                            // Response successful
                        },
                        Err(e) => {
                            eprintln!("Failed to write to stream: {}", e);
                        },
                    };
                }
            }
            Err(err) => {
                if let Some(stream) = stream { 
                    let response_value = RespValue::Error(err).to_bytes();
                    match stream.write(&response_value) {
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

        
    fn command_type(&self) -> crate::interface::command_type::CommandType {
        CommandType::Write
    }
}