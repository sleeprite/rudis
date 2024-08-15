use std::io::Write;
use std::{
    collections::HashMap,
    net::TcpStream,
    sync::Arc,
};

use ahash::AHashMap;
use parking_lot::Mutex;

use crate::interface::command_type::CommandType;
use crate::tools::resp::RespValue;
use crate::session::session::Session;
use crate::{db::db::Db, RudisConfig};
use crate::interface::command_strategy::CommandStrategy;

pub struct HmsetCommand {}

impl CommandStrategy for HmsetCommand {
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
        let mut values = HashMap::new();

        for i in (6..fragments.len()).step_by(4) {
            let field = fragments[i].to_string();
            let value = fragments[i + 2].to_string();
            values.insert(field, value);
        }

        match db_ref.hmset(db_index, key.clone(), values) {
            Ok(()) => {
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
            Err(err_msg) => {
                if let Some(stream) = stream {
                    let response_bytes = &RespValue::Error(err_msg.to_string()).to_bytes();
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