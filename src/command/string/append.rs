
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

        match redis_ref.get(db_index, &key) {
            Ok(Some(old_value)) => {
                let new_value = format!("{}{}", old_value, value);
                redis_ref.set(db_index, key, new_value.clone(), false);
                stream.write(format!(":{}\r\n", new_value.len()).as_bytes()).unwrap();
            },
            Ok(None) => {
                redis_ref.set(db_index, key, value, false);
                stream.write(b"+OK\r\n").unwrap();
            },
            Err(err_msg) => {
                stream.write(format!("-${}\r\n", err_msg.len()).as_bytes()).unwrap();
                stream.write(err_msg.as_bytes()).unwrap();
                stream.write(b"\r\n").unwrap();
            }
        }
    }
}