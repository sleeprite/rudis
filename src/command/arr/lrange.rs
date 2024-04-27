use std::io::Write;
use std::{
    collections::HashMap,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::session::session::Session;
use crate::{db::db::Redis, RedisConfig};
use crate::interface::command_strategy::CommandStrategy;

pub struct LrangeCommand {}

impl CommandStrategy for LrangeCommand {
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
        let start: i64 = fragments[6].parse().unwrap();
        let end: i64 = fragments[8].parse().unwrap();

        let values = redis_ref.lrange(db_index, key.clone(), start, end);

        let response = format!("*{}\r\n", values.len());
        stream.write(response.as_bytes()).unwrap();
        for key in values {
            let response = format!("${}\r\n{}\r\n", key.len(), key);
            stream.write(response.as_bytes()).unwrap();
        }
    }
}