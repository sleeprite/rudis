
use std::{collections::HashMap, net::TcpStream, sync::{Arc, Mutex}};
use std::io::Write;

use crate::{command_strategy::CommandStrategy, db::db::Redis, session::session::Session, RedisConfig};

/*
 * Auth 命令
 */
pub struct AuthCommand {}

impl CommandStrategy for AuthCommand {
    fn execute(
        &self,
        stream: &mut TcpStream,
        fragments: &Vec<&str>,
        _redis: &Arc<Mutex<Redis>>,
        _redis_config: &Arc<RedisConfig>,
        sessions: &Arc<Mutex<HashMap<String, Session>>>,
    ) {
        if fragments.len() < 3 {
            stream
                .write(b"-ERR wrong number of arguments for 'auth' command\r\n")
                .unwrap();
            return;
        }

        let password = fragments[4];

        match &(*_redis_config).password {
            Some(p) => {
                if password != p {
                    let session_id = stream.peer_addr().unwrap().to_string();
                    let mut session_ref = sessions.lock().unwrap();
                    if let Some(session) = session_ref.get_mut(&session_id) {
                        session.set_authenticated(false);
                    }
                    stream.write(b"-ERR invalid password\r\n").unwrap();
                    return;
                }
            }
            None => {
                println!("No password set.");
            }
        }

        let session_id = stream.peer_addr().unwrap().to_string();
        let mut session_ref = sessions.lock().unwrap();
        if let Some(session) = session_ref.get_mut(&session_id) {
            session.set_authenticated(true);
        }

        stream.write(b"+OK\r\n").unwrap();
    }
}