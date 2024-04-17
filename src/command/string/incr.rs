
use std::{collections::HashMap, net::TcpStream, sync::{Arc, Mutex}};
use std::io::Write;

use crate::{command_strategy::CommandStrategy, db::db::Redis, session::session::Session, RedisConfig};

/*
 * Incr 命令
 */
pub struct IncrCommand {}

impl CommandStrategy for IncrCommand {
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

        if let Some(existing_value) = redis_ref.get(db_index, &key.clone()) {
            if let Ok(value) = existing_value.parse::<i64>() {
                let new_value = value + 1;
                redis_ref.set(db_index, key, new_value.to_string(), false);
                stream.write(format!(":{}\r\n", new_value.to_string().len()).as_bytes()).unwrap();
            } else {
                stream.write(b"-ERR value is not an integer\r\n").unwrap();
            }
        } else {
            redis_ref.set(db_index, key, "1".to_string(), false);
            stream.write(b":1\r\n").unwrap();
        }
    }
}