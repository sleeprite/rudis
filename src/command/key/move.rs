use std::{collections::HashMap, net::TcpStream, sync::{Arc, Mutex}};
use std::io::Write;
use crate::{command_strategy::CommandStrategy, db::db::Redis, session::session::Session, RedisConfig};

/*
 * Move 命令
 */
pub struct MoveCommand {}

impl CommandStrategy for MoveCommand {
    fn execute(
        &self,
        stream: &mut TcpStream,
        fragments: &Vec<&str>,
        redis: &Arc<Mutex<Redis>>,
        _redis_config: &Arc<RedisConfig>,
        sessions: &Arc<Mutex<HashMap<String, Session>>>,
    ) {
        let mut redis_ref = redis.lock().unwrap();

        let db_index = {
            let sessions_ref = sessions.lock().unwrap();
            if let Some(session) = sessions_ref.get(&stream.peer_addr().unwrap().to_string()) {
                session.get_selected_database()
            } else {
                return;
            }
        };

        let key = fragments[4].to_string();
        let dest_db_index: usize = fragments[6].parse().unwrap(); // 解析目标数据库索引

        let move_result = redis_ref.move_key(db_index, &key, dest_db_index);

        match move_result {
            Ok(_) => {
                stream.write(b"+OK\r\n").unwrap(); // 成功返回 OK
            }
            Err(_) => {
                stream.write(b"-ERR\r\n").unwrap(); // 失败返回 ERR
            }
        }
    }
}