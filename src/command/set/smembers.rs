use std::io::Write;
use std::{
    collections::HashMap,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::interface::command_type::CommandType;
use crate::session::session::Session;
use crate::{db::db::Redis, RedisConfig};
use crate::interface::command_strategy::CommandStrategy;

pub struct SmembersCommand {}

impl CommandStrategy for SmembersCommand {
    fn execute(
        &self,
        stream: Option<&mut TcpStream>,
        fragments: &Vec<&str>,
        redis: &Arc<Mutex<Redis>>,
        _redis_config: &Arc<RedisConfig>,
        sessions: &Arc<Mutex<HashMap<String, Session>>>,
        session_id: &String
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

        // 检测过期
        redis_ref.check_all_ttl(db_index);

        if let Some(key) = fragments.get(4) {
            if let Some(members) = redis_ref.smembers(db_index, &key.to_string()) {
                if let Some(stream) = stream { 
                    let response = format!("*{}\r\n", members.len());
                    stream.write(response.as_bytes()).unwrap();
                    for member in members {
                        let response = format!("${}\r\n{}\r\n", member.len(), member);
                        stream.write(response.as_bytes()).unwrap();
                    }
                }
            } else {
                if let Some(stream) = stream { 
                    let response = format!("*0\r\n");
                    stream.write(response.as_bytes()).unwrap();
                }
            }
        } else {
            if let Some(stream) = stream { 
                let response = "-ERR wrong number of arguments for 'smembers' command\r\n";
                stream.write(response.as_bytes()).unwrap();
            }
        }
    }

        
    fn command_type(&self) -> crate::interface::command_type::CommandType {
        return CommandType::Read;
    }
}