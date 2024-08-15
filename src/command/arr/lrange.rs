use std::io::Write;
use std::{
    collections::HashMap,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::interface::command_type::CommandType;
use crate::session::session::Session;
use crate::{db::db::Db, RudisConfig};
use crate::interface::command_strategy::CommandStrategy;

pub struct LrangeCommand {}

impl CommandStrategy for LrangeCommand {
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

        let key = fragments[4].to_string();
        let start: i64 = fragments[6].parse().unwrap();
        let end: i64 = fragments[8].parse().unwrap();
        
        db_ref.check_ttl(db_index, &key);

        let values = db_ref.lrange(db_index, key.clone(), start, end);

        if let Some(stream) = stream {
            let response = format!("*{}\r\n", values.len());
            match stream.write(response.as_bytes()) {
                Ok(_bytes_written) => {},
                Err(e) => {
                    eprintln!("Failed to write to stream: {}", e);
                },
            };
            for key in values {
                let response = format!("${}\r\n{}\r\n", key.len(), key);
                match stream.write(response.as_bytes()) {
                    Ok(_bytes_written) => {},
                    Err(e) => {
                        eprintln!("Failed to write to stream: {}", e);
                    },
                };
            }
        }
    }

    fn command_type(&self) -> crate::interface::command_type::CommandType {
        CommandType::Read
    }
}