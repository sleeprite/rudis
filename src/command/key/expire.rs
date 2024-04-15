use std::{collections::HashMap, net::TcpStream, sync::{Arc, Mutex}};
use std::io::Write;
use crate::{command_strategy::CommandStrategy, db::db::Redis, session::session::Session, RedisConfig};

pub struct ExpireCommand {}

impl CommandStrategy for ExpireCommand {
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
        let ttl_millis = fragments[6].parse::<i64>().unwrap();
        let result = redis_ref.expire(db_index, key, ttl_millis);

        if result {
            stream.write(b":1\r\n").unwrap(); // 成功
        } else {
            stream.write(b":0\r\n").unwrap(); // 失败
        }
    }
}