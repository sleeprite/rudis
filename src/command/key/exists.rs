use crate::tools::resp::RespValue;
use crate::{
    db::db::Redis, session::session::Session, RedisConfig,
};
use crate::interface::command_strategy::CommandStrategy;
use std::io::Write;
use std::{
    collections::HashMap,
    net::TcpStream,
    sync::{Arc, Mutex},
};

/*
 * Exists 命令
 */
pub struct ExistsCommand {}

impl CommandStrategy for ExistsCommand {
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

        redis_ref.check_ttl(db_index, &fragments[4].to_string());
        let is_exists = redis_ref.exists(db_index, &fragments[4].to_string());
        if is_exists {
            let response_bytes = &RespValue::Integer(1).to_bytes();
            stream.write(response_bytes).unwrap();
        } else {
            let response_bytes = &RespValue::Integer(0).to_bytes();
            stream.write(response_bytes).unwrap();
        }
    }
}
