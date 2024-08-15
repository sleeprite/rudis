use std::{net::TcpStream, sync::Arc};

use ahash::AHashMap;
use parking_lot::Mutex;

use crate::{db::db_config::RudisConfig, session::session::Session};
use crate::db::db::Db;

use super::command_type::CommandType;

/*
 * 命令策略接口
 *
 * @param stream 通讯流
 * @param fragments 消息内容
 * @param redis 数据库实例
 * @param rudis_config 数据库配置
 * @param sessions 会话列表
 */
pub trait CommandStrategy {

    // 命令逻辑
    fn execute(
        &self,
        stream: Option<&mut TcpStream>,
        fragments: &[&str],
        db: &Arc<Mutex<Db>>,
        rudis_config: &Arc<RudisConfig>,
        sessions: &Arc<Mutex<AHashMap<String, Session>>>,
        session_id: &str,
    );

    // 命令类型
    fn command_type(&self) -> CommandType; 
    
}