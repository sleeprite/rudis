use std::io::Write;
use std::{
    collections::HashMap,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::session::session::Session;
use crate::tools::reponse::RespValue;
use crate::{command_strategy::CommandStrategy, db::db::Redis, RedisConfig};

pub struct LindexCommand {}

impl CommandStrategy for LindexCommand {
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
        let index: usize = fragments[6].parse().unwrap_or_default(); // Assuming index is provided as parameter

        let result = redis_ref.lindex(db_index, &key.clone(), index as i64); // Assuming you have a method to retrieve value by index

        // Write the result back to the client
        match result {
            Some(value) => {
                let response_bytes = &RespValue::BulkString(value).to_bytes();
                stream.write(response_bytes).unwrap();
            }
            None => {
                let response_bytes = &RespValue::Null.to_bytes();
                stream.write(response_bytes).unwrap();
            }
        }
    }
}