
use std::{collections::HashMap, net::TcpStream, sync::{Arc, Mutex}};
use std::io::Write;

use crate::{command_strategy::CommandStrategy, db::db::Redis, session::session::Session, RedisConfig};

/*
 * Del 命令
 */
pub struct DelCommand {}

impl CommandStrategy for DelCommand {
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

        let mut start_index = 4;
        let end_index = fragments.len();
        let mut del_count = 0;

        redis_ref.check_all_ttl(db_index);

        while start_index < end_index {
            let key = fragments[start_index].to_string();
            let is_del = redis_ref.del(db_index, &key, false);
            if is_del {
                del_count += 1;
            }
            start_index += 2;
        }

        stream
            .write(format!(":{}\r\n", del_count).as_bytes())
            .unwrap();
    }
}
