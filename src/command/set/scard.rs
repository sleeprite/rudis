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
use crate::interface::command_strategy::{CommandStrategy, ParseError};

pub struct ScardCommand {}

impl CommandStrategy for ScardCommand {

    fn parse(&self, stream: Option<&mut TcpStream>, fragments: &[&str]) -> Result<(), ParseError> {
        Ok(())
    }

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

        redis_ref.check_all_ttl(db_index);

        if let Some(cardinality) = redis_ref.scard(db_index, &key.to_string()) {
            if let Some(stream) = stream { 
                let response_value = RespValue::Integer(cardinality as i64).to_bytes();
                match stream.write(&response_value) {
                    Ok(_bytes_written) => {
                        // Response successful
                    },
                    Err(e) => {
                        eprintln!("Failed to write to stream: {}", e);
                    },
                };
            }
        } else if let Some(stream) = stream { 
            let response_value = RespValue::Integer(0).to_bytes();
            match stream.write(&response_value) {
                Ok(_bytes_written) => {
                    // Response successful
                },
                Err(e) => {
                    eprintln!("Failed to write to stream: {}", e);
                },
            };
        }
    }

        
    fn command_type(&self) -> crate::interface::command_type::CommandType {
        CommandType::Read
    }
}