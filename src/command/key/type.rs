use std::{collections::HashMap, net::TcpStream, sync::{Arc, Mutex}};
use std::io::Write;
use crate::{db::db::Db, interface::command_type::CommandType, session::session::Session, tools::resp::RespValue, RudisConfig};
use crate::interface::command_strategy::CommandStrategy;

pub struct TypeCommand {}

impl CommandStrategy for TypeCommand {
    fn execute(
        &self,
        stream: Option<&mut TcpStream>,
        fragments: &[&str],
        db: &Arc<Mutex<Db>>,
        _rudis_config: &Arc<RudisConfig>,
        sessions: &Arc<Mutex<HashMap<String, Session>>>,
        session_id: &str
    ) {
        let mut db_ref = db.lock().unwrap();

        let db_index = {
            let sessions_ref = sessions.lock().unwrap();
            if let Some(session) = sessions_ref.get(session_id) {
                session.get_selected_database()
            } else {
                return;
            }
        };

        let key: String = fragments[4].to_string();

        db_ref.check_ttl(db_index, &key);

        let key_type = db_ref.key_type(db_index, key);

        if let Some(stream) = stream { 
            let response_bytes = &RespValue::SimpleString(key_type).to_bytes();
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
        CommandType::Read
    }
}