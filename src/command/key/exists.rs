use crate::interface::command_type::CommandType;
use crate::tools::resp::RespValue;
use crate::{
    db::db::Db, session::session::Session, RudisConfig,
};
use crate::interface::command_strategy::CommandStrategy;
use std::io::Write;
use std::{
    collections::HashMap,
    net::TcpStream,
    sync::Arc,
};

use parking_lot::Mutex;

/*
 * Exists 命令
 */
pub struct ExistsCommand {}

impl CommandStrategy for ExistsCommand {
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

        db_ref.check_ttl(db_index, &key);
        
        let is_exists = db_ref.exists(db_index, &key);
        if is_exists {
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
