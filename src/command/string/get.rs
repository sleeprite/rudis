
use std::{collections::HashMap, net::TcpStream, sync::Arc};
use parking_lot::Mutex;
use std::io::Write;

use crate::{db::db::Db, interface::command_type::CommandType, session::session::Session, tools::resp::RespValue, RudisConfig};
use crate::interface::command_strategy::CommandStrategy;

/*
 * Get 命令
 */
pub struct GetCommand {}

impl CommandStrategy for GetCommand {
    fn execute(
        &self,
        stream: Option<&mut TcpStream>,
        fragments: &[&str],
        db: &Arc<Mutex<Db>>,
        _rudis_config: &Arc<RudisConfig>,
        sessions: &Arc<Mutex<HashMap<String, Session>>>,
        session_id: &str
    ) {
        
        let mut db_ref = db.lock();
        let db_index = {
            let sessions_ref = sessions.lock();
            if let Some(session) = sessions_ref.get(session_id) {
                session.get_selected_database()
            } else {
                return;
            }
        };

        let key = match fragments.get(4) {
            Some(fragment) => fragment.to_string(),
            None => {
                if let Some(stream) = stream { 
                    let response_bytes = &RespValue::Error("ERR wrong number of arguments for 'get' command".to_string()).to_bytes();
                    match stream.write(response_bytes) {
                        Ok(_bytes_written) => {},
                        Err(e) => {
                            eprintln!("Failed to write to stream: {}", e);
                        },
                    };
                }
                return;
            },
        };

        db_ref.check_ttl(db_index, &key);

        match db_ref.get(db_index, &key) {
            Ok(Some(value)) => {
                if let Some(stream) = stream { 
                    let response_bytes = &RespValue::BulkString(value.to_string()).to_bytes();
                    match stream.write(response_bytes) {
                        Ok(_bytes_written) => {},
                        Err(e) => {
                            eprintln!("Failed to write to stream: {}", e);
                        },
                    };
                }
            },
            Ok(None) => {
                if let Some(stream) = stream { 
                    let response_bytes = &RespValue::Null.to_bytes();
                    match stream.write(response_bytes) {
                        Ok(_bytes_written) => {},
                        Err(e) => {
                            eprintln!("Failed to write to stream: {}", e);
                        },
                    };
                }
            },
            Err(err_msg) => {
                if let Some(stream) = stream { 
                    let response_bytes = &RespValue::Error(err_msg.to_string()).to_bytes();
                    match stream.write(response_bytes) {
                        Ok(_bytes_written) => {},
                        Err(e) => {
                            eprintln!("Failed to write to stream: {}", e);
                        },
                    };
                }
            }
        }
    }

        
    fn command_type(&self) -> crate::interface::command_type::CommandType {
        CommandType::Read
    }
}