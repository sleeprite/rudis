use std::{
    collections::HashMap,
    io::Write,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::{
    db::db::Redis,
    interface::command_strategy::CommandStrategy,
    interface::command_type::CommandType,
    session::session::Session,
    tools::resp::RespValue,
    RedisConfig,
};

pub struct HsetCommand {}

impl CommandStrategy for HsetCommand {
    fn execute(
        &self,
        stream: Option<&mut TcpStream>,
        fragments: &Vec<&str>,
        redis: &Arc<Mutex<Redis>>,
        _redis_config: &Arc<RedisConfig>,
        sessions: &Arc<Mutex<HashMap<String, Session>>>,
        session_id: &String,
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
        let field = fragments[6].to_string();
        let value = fragments[8].to_string();

        match redis_ref.hset(db_index, key.clone(), field, value) {
            Ok(result) => {
                if let Some(stream) = stream {
                    let response_bytes = &RespValue::Integer(result as i64).to_bytes();
                    stream.write(response_bytes).unwrap();
                }
            }
            Err(err_msg) => {
                if let Some(stream) = stream {
                    let response_bytes = &RespValue::Error(err_msg.to_string()).to_bytes();
                    stream.write(response_bytes).unwrap();
                }
            }
        }
    }

    fn command_type(&self) -> CommandType {
        CommandType::Write
    }
}