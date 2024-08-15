use std::{collections::HashMap, net::TcpStream, sync::Arc};
use parking_lot::Mutex;
use std::io::Write;
use crate::{db::db::Db, interface::command_type::CommandType, session::session::Session, tools::{date::current_millis, resp::RespValue}, RudisConfig};
use crate::interface::command_strategy::CommandStrategy;
pub struct ExpireCommand {}

impl CommandStrategy for ExpireCommand {
    fn execute(
        &self,
        stream: Option<&mut TcpStream>,
        fragments: &[&str],
        db: &Arc<Mutex<Db>>,
        _rudis_config: &Arc<RudisConfig>,
        sessions: &Arc<Mutex<HashMap<String, Session>>>,
        session_id: &str
    ) {
        let mut db_ref = db.lock();

        let db_index = {
            let sessions_ref = sessions.lock();
            if let Some(session) = sessions_ref.get(session_id) {
                session.get_selected_database()
            } else {
                return;
            }
        };

        let key = fragments[4].to_string();
        let ttl_millis = fragments[6].parse::<i64>().unwrap() * 1000;
        let expire_at: i64 = current_millis() + ttl_millis;

        db_ref.check_ttl(db_index, &key);

        if db_ref.expire(db_index, key, expire_at) {
            if let Some(stream) = stream { 
                let response_bytes = &RespValue::Integer(1).to_bytes();
                match stream.write(response_bytes) {
                    Ok(_bytes_written) => {},
                    Err(e) => {
                        eprintln!("Failed to write to stream: {}", e);
                    },
                };
            }
        } else if let Some(stream) = stream { 
            let response_bytes = &RespValue::Integer(0).to_bytes();
            match stream.write(response_bytes) {
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
        CommandType::Write
    }
}