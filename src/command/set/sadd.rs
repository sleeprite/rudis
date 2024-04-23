use std::io::Write;
use std::{
    collections::HashMap,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::session::session::Session;
use crate::tools::reponse::RespValue;
use crate::{command_strategy::CommandStrategy, db::db::Redis, RedisConfig};

pub struct SaddCommand {}

impl CommandStrategy for SaddCommand {
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
        let members: Vec<String> = fragments[6..].iter().enumerate().filter(|(i, _)| *i % 2 == 0).map(|(_, &x)| x.to_string()).collect();
        let result = redis_ref.sadd(db_index, key.clone(), members, false);
        
        match result {
            Ok(value) => {
                let response_bytes = &RespValue::Integer(value).to_bytes();
                stream.write(response_bytes).unwrap();
            },
            Err(err_msg) => {
                let response_bytes = &RespValue::Error(err_msg.to_string()).to_bytes();
                stream.write(response_bytes).unwrap();
            }
        }

    }
}