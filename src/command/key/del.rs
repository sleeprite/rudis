
use std::{collections::HashMap, net::TcpStream, sync::{Arc, Mutex}};
use std::io::Write;

use crate::{command_strategy::CommandStrategy, db::db::Redis, session::session::Session, tools::resp::RespValue, RedisConfig};

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

        let del_index = 4;
        let mut del_count = 0;
        redis_ref.check_all_ttl(db_index);

        for key in fragments.iter().skip(del_index).step_by(2) {
            let key_string = key.to_string();
            let is_del = redis_ref.del(db_index, &key_string, false);
            if is_del {
                del_count += 1;
            }
        }

        let response_bytes = &RespValue::Integer(del_count).to_bytes();
        stream.write(response_bytes).unwrap();
    }
}
