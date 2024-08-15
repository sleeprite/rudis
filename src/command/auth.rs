
use std::{collections::HashMap, net::TcpStream, sync::{Arc, Mutex}};
use std::io::Write;

use crate::{db::db::Db, interface::command_type::CommandType, session::session::Session, tools::resp::RespValue, RudisConfig};
use crate::interface::command_strategy::CommandStrategy;

/*
 * Auth 命令
 */
pub struct AuthCommand {}

impl CommandStrategy for AuthCommand {
    fn execute(
        &self,
        stream: Option<&mut TcpStream>,
        fragments: &[&str],
        _db: &Arc<Mutex<Db>>,
        _rudis_config: &Arc<RudisConfig>,
        sessions: &Arc<Mutex<HashMap<String, Session>>>,
        session_id: &str
    ) {
        if fragments.len() < 3 {
            if let Some(stream) = stream { 
                let response_bytes = &RespValue::Error("ERR wrong number of arguments for 'auth' command".to_string()).to_bytes();
                match stream.write(response_bytes) {
                    Ok(_bytes_written) => {
                        // Response successful
                    },
                    Err(e) => {
                        eprintln!("Failed to write to stream: {}", e);
                    },
                };
            }
            return;
        }
        let password = fragments[4];
        match &(_rudis_config).password {
            Some(p) => {
                if password != p {
                    let mut session_ref = sessions.lock().unwrap();
                    if let Some(session) = session_ref.get_mut(session_id) {
                        session.set_authenticated(false);
                    }
                    if let Some(stream) = stream { 
                        let response_bytes = &RespValue::Error("ERR invalid password".to_string()).to_bytes();
                        match stream.write(response_bytes) {
                            Ok(_bytes_written) => {
                                // Response successful
                            },
                            Err(e) => {
                                eprintln!("Failed to write to stream: {}", e);
                            },
                        };
                    }
                    return;
                }
            }
            None => {
                println!("No password set.");
            }
        }
        let mut session_ref = sessions.lock().unwrap();
        
        if let Some(session) = session_ref.get_mut(session_id) {
            session.set_authenticated(true);
        }
        
        if let Some(stream) = stream { 
            let response_bytes = &RespValue::Ok.to_bytes();
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