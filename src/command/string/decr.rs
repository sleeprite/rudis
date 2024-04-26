
use std::{collections::HashMap, net::TcpStream, sync::{Arc, Mutex}};
use std::io::Write;

use crate::{command_strategy::CommandStrategy, db::db::Redis, session::session::Session, tools::resp::RespValue, RedisConfig};

/*
 * Incr 命令
 */
pub struct DecrCommand {}

impl CommandStrategy for DecrCommand {
    fn execute(
        &self,
        stream: &mut TcpStream,
        fragments: &Vec<&str>,
        redis: &Arc<Mutex<Redis>>,
        _redis_config: &Arc<RedisConfig>,
        sessions: &Arc<Mutex<HashMap<String, Session>>>,
    ) {
        let mut redis_ref = redis.lock().unwrap();

        let db_index = {
            let sessions_ref = sessions.lock().unwrap();
            if let Some(session) = sessions_ref.get(&stream.peer_addr().unwrap().to_string()) {
                session.get_selected_database()
            } else {
                return;
            }
        };

        let key = fragments[4].to_string();
        
        match redis_ref.decr(db_index, key, 1, false) {
            Ok(result) => {
                
                let response_value = RespValue::Integer(result).to_bytes();
                stream.write(&response_value).unwrap();

            }
            Err(err) => {
                let response_value = RespValue::Error(err).to_bytes();
                stream.write(&response_value).unwrap();
            }
        }
    }
}