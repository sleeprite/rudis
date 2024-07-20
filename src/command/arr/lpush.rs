use std::io::Write;
use std::{
    collections::HashMap,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::interface::command_type::CommandType;
use crate::session::session::Session;
use crate::tools::resp::RespValue;
use crate::{db::db::Redis, RedisConfig};
use crate::interface::command_strategy::CommandStrategy;

pub struct LpushCommand {}

impl CommandStrategy for LpushCommand {
    fn execute(
        &self,
        stream: Option<&mut TcpStream>,
        fragments: &[&str],
        redis: &Arc<Mutex<Redis>>,
        _redis_config: &Arc<RedisConfig>,
        sessions: &Arc<Mutex<HashMap<String, Session>>>,
        session_id: &str
    ) {
        let mut redis_ref = redis.lock().unwrap();

        let db_index = {
            let sessions_ref = sessions.lock().unwrap();
            if let Some(session) = sessions_ref.get(session_id) {
                session.get_selected_database()
            } else {
                return;
            }
        };

        let key = fragments[4].to_string();
        let values: Vec<String> = fragments[6..].iter().enumerate().filter(|(i, _)| *i % 2 == 0).map(|(_, &x)| x.to_string()).collect();
        
        redis_ref.check_ttl(db_index, &key);
        redis_ref.lpush(db_index, key.clone(), values); 

        if let Some(stream) = stream {
            let response_bytes = &RespValue::Ok.to_bytes();
            stream.write(response_bytes).unwrap();
        }
    }

    fn command_type(&self) -> crate::interface::command_type::CommandType {
        CommandType::Write
    }
}
