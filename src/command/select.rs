use std::io::Write;
use std::{
    collections::HashMap,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::interface::command_type::CommandType;
use crate::tools::resp::RespValue;
use crate::interface::command_strategy::{CommandStrategy, ParseError};
use crate::{
    db::db::Redis, session::session::Session, RedisConfig,
};

/*
 * Select 命令
 */
pub struct SelectCommand {}

impl CommandStrategy for SelectCommand {

    fn parse(&self, stream: Option<&mut TcpStream>, fragments: &[&str]) -> Result<(), ParseError> {
        Ok(())
    }

    fn execute(
        &self,
        stream: Option<&mut TcpStream>,
        fragments: &[&str],
        _redis: &Arc<Mutex<Redis>>,
        _redis_config: &Arc<RedisConfig>,
        sessions: &Arc<Mutex<HashMap<String, Session>>>,
        session_id: &str
    ) {
        /*
         * 验证语法
         */
        if fragments.len() < 4 {
            if let Some(stream) = stream { 
                let response_bytes = &RespValue::Error("ERR wrong number of arguments for 'select' command".to_string()).to_bytes();
                match stream.write(response_bytes) {
                    Ok(_bytes_written) => {
                        // Response successful
                    },
                    Err(e) => {
                        eprintln!("Failed to write to stream: {}", e);
                    },
                };
            }
            return;
        }

        /*
         * 解析命令
         *
         * @param db_index 数据库索引
         */
        let db_index = match fragments[4].parse::<usize>() {
            Ok(index) => index,
            Err(_) => {
                if let Some(stream) = stream { 
                    let response_bytes = &RespValue::Error("ERR invalid DB index".to_string()).to_bytes();
                    match stream.write(response_bytes) {
                        Ok(_bytes_written) => {
                            // Response successful
                        },
                        Err(e) => {
                            eprintln!("Failed to write to stream: {}", e);
                        },
                    };
                }
                return;
            }
        };

        {
            let mut session_ref = sessions.lock().unwrap();
            if let Some(session) = session_ref.get_mut(session_id) {
                session.set_selected_database(db_index)
            }
        }

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
