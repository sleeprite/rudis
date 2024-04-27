use std::io::Write;
use std::{collections::HashMap, net::TcpStream, sync::{Arc, Mutex}};

use crate::tools::resp::RespValue;
use crate::session::session::Session;
use crate::{db::db::Redis, RedisConfig};
use crate::interface::command_strategy::CommandStrategy;

pub struct HexistsCommand {}

impl CommandStrategy for HexistsCommand {
    fn execute(
        &self,
        stream: &mut TcpStream,
        fragments: &Vec<&str>,
        redis: &Arc<Mutex<Redis>>,
        _redis_config: &Arc<RedisConfig>,
        sessions: &Arc<Mutex<HashMap<String, Session>>>,
    ) {
        let redis_ref = redis.lock().unwrap();
        let db_index = {
            let sessions_ref = sessions.lock().unwrap();
            if let Some(session) = sessions_ref.get(&stream.peer_addr().unwrap().to_string()) {
                session.get_selected_database()
            } else {
                return;
            }
        };

        let key = fragments[4].to_string();
        let field = fragments[6].to_string();

        match redis_ref.hexists(db_index, &key, &field) {
            Ok(exists) => {
                if exists {
                    stream.write(b":1\r\n").unwrap();
                } else {
                    stream.write(b":0\r\n").unwrap();
                }
            },
            Err(err_msg) => {
                let response_bytes = &RespValue::Error(err_msg.to_string()).to_bytes();
                stream.write(response_bytes).unwrap();
            }
        }
    }
}