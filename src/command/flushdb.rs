use std::{collections::HashMap, net::TcpStream, sync::{Arc, Mutex}};
use std::io::Write;
use crate::{db::db::Db, interface::command_type::CommandType, session::session::Session, tools::resp::RespValue, RudisConfig};
use crate::interface::command_strategy::CommandStrategy;
/*
 * FlushDb 命令
 */
pub struct FlushDbCommand {}

impl CommandStrategy for FlushDbCommand {
    fn execute(
        &self,
        stream: Option<&mut TcpStream>,
        _fragments: &[&str],
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

        db_ref.flush_db(db_index);
        
        if let Some(stream) = stream { 
            let response_bytes = &RespValue::Ok.to_bytes();
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