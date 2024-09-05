
use std::{net::TcpStream, sync::Arc};
use ahash::AHashMap;
use parking_lot::Mutex;
use std::io::Write;

use crate::{db::db::Db, interface::command_type::CommandType, session::session::Session, tools::resp::RespValue, RudisConfig};
use crate::interface::command_strategy::CommandStrategy;

/*
 * Del 命令
 */
pub struct DelCommand {}

impl CommandStrategy for DelCommand {
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

        let del_index = 4;
        let mut del_count = 0;

        db_ref.check_all_ttl(db_index);

        for key in fragments.iter().skip(del_index).step_by(2) {
            let key_string = key.to_string();
            let is_del = db_ref.del(db_index, &key_string);
            if is_del {
                del_count += 1;
            }
        }

        if let Some(stream) = stream {
            let response_bytes = &RespValue::Integer(del_count).as_bytes();
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
