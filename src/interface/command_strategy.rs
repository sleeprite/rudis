use std::{collections::HashMap, net::TcpStream, sync::{Arc, Mutex}};
use crate::{db::db_config::RedisConfig, session::session::Session};
use crate::db::db::Redis;
use super::command_type::CommandType;

pub enum ParseError {
    InputError
}

/*
 * 命令策略接口
 */
pub trait CommandStrategy {

    // 解析命令
    fn parse(
        &self,
        stream: Option<&mut TcpStream>,
        fragments: &[&str],
    ) -> Result<(), ParseError>;

    // 命令逻辑
    fn execute(
        &self,
        stream: Option<&mut TcpStream>,
        fragments: &[&str],
        redis: &Arc<Mutex<Redis>>,
        redis_config: &Arc<RedisConfig>,
        sessions: &Arc<Mutex<HashMap<String, Session>>>,
        session_id: &str,
    );

    // 命令类型
    fn command_type(&self) -> CommandType; 
    
}