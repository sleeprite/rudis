
use std::{collections::HashMap, net::TcpStream, sync::{Arc, Mutex}};
use std::io::Write;

use crate::{command_strategy::CommandStrategy, db::db::Redis, session::session::Session, RedisConfig};

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

        if let Some(existing_value) = redis_ref.get(db_index, &key.clone()) {
            let new_value = format!("{}{}", existing_value, value);
            redis_ref.set(db_index, key, new_value.clone());
            stream.write(format!(":{}\r\n", new_value.len()).as_bytes()).unwrap();
        } else {
            redis_ref.set(db_index, key, value);
            stream.write(b"+OK\r\n").unwrap();
        }
    }
}