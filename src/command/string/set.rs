use std::io::Write;
use std::{
    net::TcpStream,
    sync::Arc,
};

use ahash::AHashMap;
use parking_lot::Mutex;

use crate::interface::command_strategy::CommandStrategy;
use crate::interface::command_type::CommandType;
use crate::tools::resp::RespValue;
use crate::{
    db::db::Db, session::session::Session,
    tools::date::current_millis, RudisConfig,
};

/*
 * Set 命令
 */
pub struct SetCommand {}

impl CommandStrategy for SetCommand {
    fn execute(
        &self,
        stream: Option<&mut TcpStream>,
        fragments: &[&str],
        db: &Arc<Mutex<Db>>,
        _rudis_config: &Arc<RudisConfig>,
        sessions: &Arc<Mutex<AHashMap<String, Session>>>,
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
                    let response_bytes = &RespValue::Error("ERR wrong number of arguments for 'set' command".to_string()).as_bytes();
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

        let value = match fragments.get(6) {
            Some(fragment) => fragment.to_string(),
            None => {
                if let Some(stream) = stream { 
                    let response_bytes = &RespValue::Error("ERR wrong number of arguments for 'set' command".to_string()).as_bytes();
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

        for (index, fragment) in fragments.iter().enumerate() {
            if fragment.to_uppercase() == "NX" {
                if index != 4 && index != 6 {
                    let is_exists = db_ref.exists(db_index, &key);
                    if is_exists {
                        if let Some(stream) = stream {
                            let response_bytes = &RespValue::Null.as_bytes();
                            match stream.write(response_bytes) {
                                Ok(_bytes_written) => {},
                                Err(e) => {
                                    eprintln!("Failed to write to stream: {}", e);
                                },
                            };
                            return;
                        }
                    }
                }
            }
        }

        for (index, fragment) in fragments.iter().enumerate() {
            if fragment.to_uppercase() == "XX" {
                if index != 4 && index != 6 {
                    let is_exists = db_ref.exists(db_index, &key);
                    if !is_exists{
                        if let Some(stream) = stream { 
                            let response_bytes = &RespValue::Null.as_bytes();
                            match stream.write(response_bytes) {
                                Ok(_bytes_written) => {},
                                Err(e) => {
                                    eprintln!("Failed to write to stream: {}", e);
                                },
                            };
                            return;
                        }
                    }
                }
            }
        }

        let mut ttl_index = None;
        let mut ttl_unit = None;

        for (index, f) in fragments.iter().enumerate().rev() {
            if index > 6 {
                if f.to_uppercase().eq_ignore_ascii_case("PX") || 
                   f.to_uppercase().eq_ignore_ascii_case("EX") {
                        ttl_index = Some(index);
                        ttl_unit = Some(fragments[index].to_uppercase());
                        break;
                }
            }
        }

        let mut expire_at = -1;
        if let Some(ttl_index) = ttl_index {
            if let Some(ttl_str) = fragments.get(ttl_index + 2) {
                if let Ok(ttl) = ttl_str.parse::<i64>() {
                    let ttl_millis = match ttl_unit.unwrap().as_str() {
                        "EX" => ttl * 1000,
                        _ => ttl
                    };
                    expire_at = current_millis() + ttl_millis;
                }
            }
        }

        db_ref.set_with_ttl(db_index, key.clone(), value.clone(), expire_at);

        if let Some(stream) = stream { 
            let response_bytes = &RespValue::Ok.as_bytes();
            match stream.write(response_bytes) {
                Ok(_bytes_written) => {},
                Err(e) => {
                    eprintln!("Failed to write to stream: {}", e);
                },
            };
        }
    }

        
    fn command_type(&self) -> crate::interface::command_type::CommandType {
        CommandType::Write
    }
}