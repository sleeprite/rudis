
use std::{collections::HashMap, net::TcpStream, sync::{Arc, Mutex}};
use std::io::Write;

use crate::{command_strategy::CommandStrategy, db::db::Redis, session::session::Session, tools::resp::RespValue, RedisConfig};

/*
 * Get 命令
 */
pub struct GetCommand {}

impl CommandStrategy for GetCommand {
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
        redis_ref.check_ttl(db_index, &key);

        match redis_ref.get(db_index, &key) {
            Ok(Some(value)) => {
                let response_bytes = &RespValue::BulkString(value.to_string()).to_bytes();
                stream.write(response_bytes).unwrap();
            },
            Ok(None) => {
                stream.write(b"$-1\r\n").unwrap();
            },
            Err(err_msg) => {
                let response_bytes = &RespValue::Error(err_msg.to_string()).to_bytes();
                stream.write(response_bytes).unwrap();
            }
        }
    }
}