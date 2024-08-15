use std::{collections::HashMap, net::TcpStream, sync::Arc};
use parking_lot::Mutex;
use std::io::Write;

use crate::{db::db::Db, interface::command_type::CommandType, session::session::Session, tools::resp::RespValue, RudisConfig};
use crate::interface::command_strategy::CommandStrategy;

/*
 * Echo 命令
 */
pub struct EchoCommand {}

impl CommandStrategy for EchoCommand {
    fn execute(
        &self,
        stream: Option<&mut TcpStream>,
        fragments: &[&str],
        _db: &Arc<Mutex<Db>>,
        _rudis_config: &Arc<RudisConfig>,
        _sessions: &Arc<Mutex<HashMap<String, Session>>>,
        _session_id: &str
    ) {
        let keyword = fragments[4].to_string();

        if let Some(stream) = stream { 
            let response_bytes = &RespValue::BulkString(keyword).to_bytes();
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