use std::io::Write;
use std::{
    collections::HashMap,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::session::session::Session;
use crate::{db::db::Redis, RedisConfig};
use crate::interface::command_strategy::CommandStrategy;

pub struct SmembersCommand {}

impl CommandStrategy for SmembersCommand {
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

        // 检测过期
        redis_ref.check_all_ttl(db_index);

        if let Some(key) = fragments.get(4) {
            if let Some(members) = redis_ref.smembers(db_index, &key.to_string()) {
                let response = format!("*{}\r\n", members.len());
                stream.write(response.as_bytes()).unwrap();
                for member in members {
                    let response = format!("${}\r\n{}\r\n", member.len(), member);
                    stream.write(response.as_bytes()).unwrap();
                }
            } else {
                let response = format!("*0\r\n");
                stream.write(response.as_bytes()).unwrap();
            }
        } else {
            let response = "-ERR wrong number of arguments for 'smembers' command\r\n";
            stream.write(response.as_bytes()).unwrap();
        }
    }
}