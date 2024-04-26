use std::io::Write;
use std::{
    collections::HashMap,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::tools::resp::RespValue;
use crate::session::session::Session;
use crate::{command_strategy::CommandStrategy, db::db::Redis, RedisConfig};

pub struct HmsetCommand {}

impl CommandStrategy for HmsetCommand {
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
        let mut values = HashMap::new();

        for i in (6..fragments.len()).step_by(4) {
            let field = fragments[i].to_string();
            let value = fragments[i + 2].to_string();
            values.insert(field, value);
        }

        match redis_ref.hmset(db_index, key.clone(), values) {
            Ok(()) => {
                let response_bytes = &RespValue::SimpleString("OK".to_string()).to_bytes();
                stream.write(response_bytes).unwrap();
            }
            Err(err_msg) => {
                let response_bytes = &RespValue::Error(err_msg.to_string()).to_bytes();
                stream.write(response_bytes).unwrap();

            }
        }
    }
}