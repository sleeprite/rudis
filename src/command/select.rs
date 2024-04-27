use std::io::Write;
use std::{
    collections::HashMap,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::tools::resp::RespValue;
use crate::interface::command_strategy::CommandStrategy;
use crate::{
    db::db::Redis, session::session::Session, RedisConfig,
};

/*
 * Select 命令
 */
pub struct SelectCommand {}

impl CommandStrategy for SelectCommand {
    fn execute(
        &self,
        stream: &mut TcpStream,
        fragments: &Vec<&str>,
        _redis: &Arc<Mutex<Redis>>,
        _redis_config: &Arc<RedisConfig>,
        sessions: &Arc<Mutex<HashMap<String, Session>>>,
    ) {
        /*
         * 验证语法
         */
        if fragments.len() < 4 {
            let response_bytes = &RespValue::Error("ERR wrong number of arguments for 'select' command".to_string()).to_bytes();
            stream.write(response_bytes).unwrap();
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
                let response_bytes = &RespValue::Error("ERR invalid DB index".to_string()).to_bytes();
                stream.write(response_bytes).unwrap();
                return;
            }
        };

        {
            let mut session_ref = sessions.lock().unwrap();
            let session_id = stream.peer_addr().unwrap().to_string();
            if let Some(session) = session_ref.get_mut(&session_id) {
                session.set_selected_database(db_index)
            }
        }

        let response_bytes = &RespValue::SimpleString("OK".to_string()).to_bytes();
        stream.write(response_bytes).unwrap();
    }
}
