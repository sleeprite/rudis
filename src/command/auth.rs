
use std::{collections::HashMap, net::TcpStream, sync::{Arc, Mutex}};
use std::io::Write;

use crate::{db::db::Redis, interface::command_type::CommandType, session::session::Session, tools::resp::RespValue, RedisConfig};
use crate::interface::command_strategy::CommandStrategy;

/*
 * Auth 命令
 */
pub struct AuthCommand {}

impl CommandStrategy for AuthCommand {
    fn execute(
        &self,
        stream: Option<&mut TcpStream>,
        fragments: &Vec<&str>,
        _redis: &Arc<Mutex<Redis>>,
        _redis_config: &Arc<RedisConfig>,
        sessions: &Arc<Mutex<HashMap<String, Session>>>,
        session_id: &String
    ) {
        if fragments.len() < 3 {
            if let Some(stream) = stream { 
                let response_bytes = &RespValue::Error("ERR wrong number of arguments for 'auth' command".to_string()).to_bytes();
                stream.write(response_bytes).unwrap();
            }
            return;
        }
        let password = fragments[4];
        match &(*_redis_config).password {
            Some(p) => {
                if password != p {
                    let mut session_ref = sessions.lock().unwrap();
                    if let Some(session) = session_ref.get_mut(session_id) {
                        session.set_authenticated(false);
                    }
                    if let Some(stream) = stream { 
                        let response_bytes = &RespValue::Error("ERR invalid password".to_string()).to_bytes();
                        stream.write(response_bytes).unwrap();
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
            let response_bytes = &RespValue::SimpleString("OK".to_string()).to_bytes();
            stream.write(response_bytes).unwrap();
        }
    }

        
    fn command_type(&self) -> crate::interface::command_type::CommandType {
        return CommandType::Read;
    }
}