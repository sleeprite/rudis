use std::{collections::HashMap, net::TcpStream, sync::{Arc, Mutex}};

use crate::{db::db_config::RedisConfig, session::session::Session};
use crate::db::db::Redis;

/*
 * 命令策略接口
 *
 * @param stream 流
 * @param fragments 消息片段
 * @param redis 数据库实例
 * @param redis_config 数据库配置
 * @param sessions 会话管理
 */
pub trait CommandStrategy {
    fn execute(
        &self,
        stream: &mut TcpStream,
        fragments: &Vec<&str>,
        redis: &Arc<Mutex<Redis>>,
        redis_config: &Arc<RedisConfig>,
        sessions: &Arc<Mutex<HashMap<String, Session>>>,
    );
}