use std::{collections::HashMap, net::TcpStream, sync::{Arc, Mutex}};
use std::io::Write;
use crate::{command_strategy::CommandStrategy, db::db::Redis, session::session::Session, tools::reponse::RespValue, RedisConfig};

/*
 * FlushDb 命令
 */
pub struct FlushDbCommand {}

impl CommandStrategy for FlushDbCommand {
    fn execute(
        &self,
        stream: &mut TcpStream,
        _fragments: &Vec<&str>,
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

        redis_ref.flush_db(db_index, false);

        let response_value = "OK".to_string();
        let response_bytes = &RespValue::SimpleString(response_value).to_bytes();
        stream.write(response_bytes).unwrap();
    }
}