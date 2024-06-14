use std::io::Write;
use std::{
    collections::HashMap,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::interface::command_strategy::CommandStrategy;
use crate::interface::command_type::CommandType;
use crate::tools::resp::RespValue;
use crate::{
    db::db::Redis, session::session::Session,
    tools::date::current_millis, RedisConfig,
};

/*
 * Set 命令
 */
pub struct SetCommand {}

impl CommandStrategy for SetCommand {
    fn execute(
        &self,
        stream: Option<&mut TcpStream>,
        fragments: &Vec<&str>,
        redis: &Arc<Mutex<Redis>>,
        _redis_config: &Arc<RedisConfig>,
        sessions: &Arc<Mutex<HashMap<String, Session>>>,
        session_id: &String
    ) {
        let mut redis_ref = redis.lock().unwrap();

        let db_index = {
            let sessions_ref = sessions.lock().unwrap();
            if let Some(session) = sessions_ref.get(session_id) {
                session.get_selected_database()
            } else {
                return;
            }
        };

        let key = fragments[4].to_string();

        if fragments.contains(&"NX") {
            let is_exists = redis_ref.exists(db_index, &key);
            if is_exists{
                if let Some(stream) = stream { 
                    let response_bytes = &RespValue::Null.to_bytes();
                    stream.write(response_bytes).unwrap();
                    return;
                }
            }
        }

        if fragments.contains(&"XX") {
            let is_exists = redis_ref.exists(db_index, &key);
            if !is_exists{
                if let Some(stream) = stream { 
                    let response_bytes = &RespValue::Null.to_bytes();
                    stream.write(response_bytes).unwrap();
                    return;
                }
            }
        }

        let mut ttl_index = None;
        let mut ttl_unit = None;
        for (index, f) in fragments.iter().enumerate().rev() {
            if f.eq_ignore_ascii_case("PX") || f.eq_ignore_ascii_case("EX") {
                ttl_index = Some(index);
                ttl_unit = Some(fragments[index].to_uppercase());
                break;
            }
        }

        let mut expire_at = -1;
        if let Some(ttl_index) = ttl_index {
            if let Some(ttl_str) = fragments.get(ttl_index + 1) {
                if let Ok(ttl) = ttl_str.parse::<i64>() {
                    let ttl_millis = match ttl_unit.unwrap().as_str() {
                        "EX" => ttl * 1000,
                        _ => ttl
                    };
        
                    expire_at = current_millis() + ttl_millis;
                }
            }
        } 

        let value = fragments[6].to_string();
        redis_ref.set_with_ttl(db_index, key.clone(), value.clone(), expire_at);

        if let Some(stream) = stream { 
            let response_bytes = &RespValue::Ok.to_bytes();
            stream.write(response_bytes).unwrap();
        }
    }

        
    fn command_type(&self) -> crate::interface::command_type::CommandType {
        return CommandType::Write;
    }
}