
use std::{net::TcpStream, sync::Arc};
use ahash::AHashMap;
use parking_lot::Mutex;
use std::io::Write;

use crate::{db::db::Db, interface::command_type::CommandType, session::session::Session, tools::pattern::match_key, RudisConfig};
use crate::interface::command_strategy::CommandStrategy;
/*
 * Keys 命令
 */
pub struct KeysCommand {}

impl CommandStrategy for KeysCommand {
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

        db_ref.check_all_ttl(db_index);
        let mut keys_list: Vec<String> = Vec::new();
        for key in db_ref.databases[db_index].keys() {
            if match_key(key, fragments[4]) {
                keys_list.push(key.clone());
            }
        }

        if let Some(stream) = stream { 
            let response = format!("*{}\r\n", keys_list.len());
            match stream.write(response.as_bytes()) {
                Ok(_bytes_written) => {},
                Err(e) => {
                    eprintln!("Failed to write to stream: {}", e);
                },
            };
            for key in keys_list {
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