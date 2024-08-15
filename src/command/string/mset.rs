use std::io::Write;
use std::{
    collections::HashMap,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::interface::command_strategy::CommandStrategy;
use crate::interface::command_type::CommandType;
use crate::tools::resp::RespValue;
use crate::{
    db::db::Db, session::session::Session,
    RudisConfig,
};

/*
 * MSet 命令
 */
pub struct MsetCommand {}

impl CommandStrategy for MsetCommand {
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

        if fragments.len() < 6 || fragments.len() % 2 != 0 {
            // 参数个数不正确，无法执行 mset
            return;
        }

        let mut data = Vec::new();

        for i in (4..fragments.len()).step_by(4) {
            let key = fragments[i].to_string();
            let value = fragments[i + 2].to_string();
            data.push((key, value));
        }

        db_ref.mset(db_index, data);

        if let Some(stream) = stream { 
            let response_bytes = &RespValue::Ok.to_bytes();
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