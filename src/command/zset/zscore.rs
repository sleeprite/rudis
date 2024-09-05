use std::io::Write;
use std::{
    net::TcpStream,
    sync::Arc,
};

use ahash::AHashMap;
use parking_lot::Mutex;

use crate::interface::command_strategy::CommandStrategy;
use crate::interface::command_type::CommandType;
use crate::session::session::Session;
use crate::tools::resp::RespValue;
use crate::{db::db::Db, RudisConfig};

pub struct ZscoreCommand {}

impl CommandStrategy for ZscoreCommand {
    fn execute(
        &self,
        stream: Option<&mut TcpStream>,
        fragments: &[&str],
        db: &Arc<Mutex<Db>>,
        _rudis_config: &Arc<RudisConfig>,
        sessions: &Arc<Mutex<AHashMap<String, Session>>>,
        session_id: &str,
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
        let member = fragments[6].to_string();


        db_ref.check_all_ttl(db_index);

        let result = db_ref.zscore(db_index, &key, &member);
        match result {
            Ok(card) => {
                if let Some(stream) = stream {
                    if let Some(card) = card {
                        let resp_value = RespValue::BulkString(card.to_string()).as_bytes();
                        match stream.write(&resp_value) {
                            Ok(_bytes_written) => {},
                            Err(e) => {
                                eprintln!("Failed to write to stream: {}", e);
                            },
                        };
                    } else {
                        let resp_value = RespValue::Null.as_bytes();
                        match stream.write(&resp_value) {
                            Ok(_bytes_written) => {},
                            Err(e) => {
                                eprintln!("Failed to write to stream: {}", e);
                            },
                        };
                    }
                }
            }
            Err(err) => {
                if let Some(stream) = stream {
                    let resp_value = &RespValue::Error(err).as_bytes();
                    match stream.write(resp_value) {
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
        CommandType::Read
    }
}
