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
use crate::{ db::db::Db, RudisConfig };

pub struct LindexCommand {}

impl CommandStrategy for LindexCommand {
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
        let index: usize = fragments[6].parse().unwrap_or_default(); 
        
        db_ref.check_ttl(db_index, &key);
        
        let result = db_ref.lindex(db_index, &key.clone(), index as i64);

        let response_bytes = match result {
            Some(value) => RespValue::BulkString(value).to_bytes(),
            None => RespValue::Null.to_bytes(),
        };

        if let Some(stream) = stream {
            match stream.write(&response_bytes) {
                Ok(_bytes_written) => {},
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