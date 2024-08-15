use std::{collections::HashMap, net::TcpStream, sync::Arc};
use parking_lot::Mutex;
use std::io::Write;
use crate::{db::db::Db, interface::command_type::CommandType, session::session::Session, tools::resp::RespValue, RudisConfig};
use crate::interface::command_strategy::CommandStrategy;
/*
 * Move 命令
 */
pub struct MoveCommand {}

impl CommandStrategy for MoveCommand {
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
        let dest_db_index: usize = fragments[6].parse().unwrap();

        db_ref.check_ttl(db_index, &key);

        let move_result = db_ref.move_key(db_index, &key, dest_db_index);

        if move_result {
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
        CommandType::Write
    }
}