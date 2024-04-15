use std::{collections::HashMap, net::TcpStream, sync::{Arc, Mutex}};
use std::io::Write;
use crate::{command_strategy::CommandStrategy, db::db::Redis, session::session::Session, RedisConfig};

/*
 * Rename 命令
 */
pub struct RenameCommand {}

impl CommandStrategy for RenameCommand {
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

        let old_key = fragments[4].to_string();
        let new_key = fragments[6].to_string();

        let rename_result = redis_ref.rename(db_index, &old_key, &new_key);

        match rename_result {
            Ok(_) => {
                stream.write(b"+OK\r\n").unwrap(); // 成功返回 OK
            }
            Err(_) => {
                stream.write(b"-ERR\r\n").unwrap(); // 失败返回 ERR
            }
        }
    }
}