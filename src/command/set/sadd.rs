use std::io::Write;
use std::{
    net::TcpStream,
    sync::Arc,
};

use ahash::AHashMap;
use parking_lot::Mutex;

use crate::interface::command_type::CommandType;
use crate::session::session::Session;
use crate::tools::resp::RespValue;
use crate::{db::db::Db, RudisConfig};
use crate::interface::command_strategy::CommandStrategy;

pub struct SaddCommand {}

impl CommandStrategy for SaddCommand {
    fn execute(
        &self,
        stream: Option<&mut TcpStream>,
        fragments: &[&str],
        db: &Arc<Mutex<Db>>,
        _rudis_config: &Arc<RudisConfig>,
        sessions: &Arc<Mutex<AHashMap<String, Session>>>,
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
        let members: Vec<String> = fragments[6..].iter().enumerate().filter(|(i, _)| *i % 2 == 0).map(|(_, &x)| x.to_string()).collect();
        
        db_ref.check_ttl(db_index, &key);
        
        let result = db_ref.sadd(db_index, key.clone(), members);
        
        match result {
            Ok(value) => {
                if let Some(stream) = stream { 
                    let response_bytes = &RespValue::Integer(value).as_bytes();
                    match stream.write(response_bytes) {
                        Ok(_bytes_written) => {},
                        Err(e) => {
                            eprintln!("Failed to write to stream: {}", e);
                        },
                    };
                }
            },
            Err(err_msg) => {
                if let Some(stream) = stream { 
                    let response_bytes = &RespValue::Error(err_msg.to_string()).as_bytes();
                    match stream.write(response_bytes) {
                        Ok(_bytes_written) => {},
                        Err(e) => {
                            eprintln!("Failed to write to stream: {}", e);
                        },
                    };
                }
            }
        }

    }

    
    fn command_type(&self) -> crate::interface::command_type::CommandType {
        CommandType::Write
    }
}