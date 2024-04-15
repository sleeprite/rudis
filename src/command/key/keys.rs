
use std::{collections::HashMap, net::TcpStream, sync::{Arc, Mutex}};
use std::io::Write;

use crate::{command_strategy::CommandStrategy, db::db::Redis, session::session::Session, tools::pattern::match_key, RedisConfig};

/*
 * Keys 命令
 */
pub struct KeysCommand {}

impl CommandStrategy for KeysCommand {
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
        let mut keys_list: Vec<String> = Vec::new();
        for key in redis_ref.databases[db_index].keys() {
            if match_key(key, fragments[4]) {
                keys_list.push(key.clone());
            }
        }

        let response = format!("*{}\r\n", keys_list.len());
        stream.write(response.as_bytes()).unwrap();
        for key in keys_list {
            let response = format!("${}\r\n{}\r\n", key.len(), key);
            stream.write(response.as_bytes()).unwrap();
        }
    }
}