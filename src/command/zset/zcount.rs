use std::io::Write;
use std::{
    collections::HashMap,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::interface::command_strategy::CommandStrategy;
use crate::interface::command_type::CommandType;
use crate::session::session::Session;
use crate::tools::resp::RespValue;
use crate::{db::db::Redis, RudisConfig};

pub struct ZcountCommand {}

impl CommandStrategy for ZcountCommand {
    fn execute(
        &self,
        stream: Option<&mut TcpStream>,
        fragments: &[&str],
        redis: &Arc<Mutex<Redis>>,
        _rudis_config: &Arc<RudisConfig>,
        sessions: &Arc<Mutex<HashMap<String, Session>>>,
        session_id: &str,
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
        let min = fragments[6].parse().unwrap();
        let max = fragments[8].parse().unwrap();

        redis_ref.check_all_ttl(db_index);

        let result = redis_ref.zcount(db_index, &key, min, max);
        match result {
            Ok(card) => {
                if let Some(stream) = stream {
                    let resp_value = &RespValue::Integer(card as i64).to_bytes();
                    match stream.write(resp_value) {
                        Ok(_bytes_written) => {
                            // Response successful
                        },
                        Err(e) => {
                            eprintln!("Failed to write to stream: {}", e);
                        },
                    };
                }
            }
            Err(err) => {
                if let Some(stream) = stream {
                    let resp_value = &RespValue::Error(err).to_bytes();
                    match stream.write(resp_value) {
                        Ok(_bytes_written) => {
                            // Response successful
                        },
                        Err(e) => {
                            eprintln!("Failed to write to stream: {}", e);
                        },
                    };
                }
            }
        }
    }

    fn command_type(&self) -> crate::interface::command_type::CommandType {
        CommandType::Read
    }
}
