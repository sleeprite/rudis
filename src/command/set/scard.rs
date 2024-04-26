use std::io::Write;
use std::{
    collections::HashMap,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::session::session::Session;
use crate::tools::resp::RespValue;
use crate::{command_strategy::CommandStrategy, db::db::Redis, RedisConfig};

pub struct ScardCommand {}

impl CommandStrategy for ScardCommand {
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
            if let Some(cardinality) = redis_ref.scard(db_index, &key.to_string()) {
                let response_value = RespValue::Integer(cardinality as i64).to_bytes();
                stream.write(&response_value).unwrap();
            } else {
                let response_value = RespValue::Integer(0).to_bytes();
                stream.write(&response_value).unwrap();
            }
        } else {
            let response_value = RespValue::Error("ERR wrong number of arguments for 'scard' command".to_string()).to_bytes();
            stream.write(&response_value).unwrap();
        }
    }
}