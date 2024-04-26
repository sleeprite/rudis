
use std::{collections::HashMap, net::TcpStream, sync::{Arc, Mutex}};
use std::io::Write;

use crate::{command_strategy::CommandStrategy, db::db::Redis, session::session::Session, tools::resp::RespValue, RedisConfig};

/*
 * Append 命令
 */
pub struct AppendCommand {}

impl CommandStrategy for AppendCommand {
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
        let value = fragments[6].to_string();
        let len = match redis_ref.append(db_index, key, value, false) {
            Ok(len) => len as i64,
            Err(err) => {
                let response_value = RespValue::Error(err).to_bytes();
                stream.write(&response_value).unwrap();
                return;
            }
        };

        let response_value = RespValue::Integer(len).to_bytes();
        stream.write(&response_value).unwrap();
    }
}