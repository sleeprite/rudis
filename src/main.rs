use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

mod command;
mod command_strategy;
mod db;
mod session;
mod tools;

use command::arr::lindex::LindexCommand;
use command::arr::llen::LlenCommand;
use command::arr::lpop::LpopCommand;
use command::arr::lpush::LpushCommand;
use command::arr::lrange::LrangeCommand;
use command::arr::rpop::RpopCommand;
use command::arr::rpush::RpushCommand;

use command::key::del::DelCommand;
use command::key::exists::ExistsCommand;
use command::key::expire::ExpireCommand;
use command::key::r#move::MoveCommand;
use command::key::rename::RenameCommand;
use command::key::keys::KeysCommand;

use command::key::ttl::TtlCommand;
use command::string::decr::DecrCommand;
use command::string::incr::IncrCommand;
use command::string::append::AppendCommand;
use command::string::get::GetCommand;
use command::string::set::SetCommand;

use command::auth::AuthCommand;
use command::dbsize::DBSizeCommand;
use command::echo::EchoCommand;
use command::flushall::FlushAllCommand;
use command::flushdb::FlushDbCommand;
use command::select::SelectCommand;
use command_strategy::CommandStrategy;
use tools::reponse::RespValue;

use crate::db::db::Redis;
use crate::db::db_config::RedisConfig;
use crate::session::session::Session;

// Bootstrap.rs
fn main() {


    /*
     * 初始日志框架    
     * 
     * (1) 日志级别
     * (2) 框架加载
     */
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    /*
     * 创建默认配置
     */
    let redis_config = Arc::new(RedisConfig::default());

    let port: u16;
    {
        port = redis_config.port;
    }

    /*
     * 创建通讯服务
     */
    let address = SocketAddr::from(([127, 0, 0, 1], port));
    let session_manager: Arc<Mutex<HashMap<String, Session>>> = Arc::new(Mutex::new(HashMap::new()));
    let redis = Arc::new(Mutex::new(Redis::new(redis_config.clone())));
    let listener = TcpListener::bind(address).unwrap();
    

    /*
     * Banner 动画
     */
    let project_name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    println_banner(project_name, version, port);

    /*
     * 加载本地数据
     */
    if redis_config.appendonly {
        match redis.lock() {
            Ok(mut redis_c) => {
                log::info!("Start loading appendfile");
                redis_c.load_aof();
                
            }
            Err(err) => {
                eprintln!("Failed to acquire lock: {:?}", err);
                return;
            }
        }   
    }
    
    log::info!("Server initialized");
    log::info!("Ready to accept connections");

    // 接收传入的链接
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let redis_clone = Arc::clone(&redis);
                let redis_config_clone = Arc::clone(&redis_config);
                let sessions_manager_clone = Arc::clone(&session_manager);
                thread::spawn(|| {
                    connection(
                        stream,
                        redis_clone,
                        redis_config_clone,
                        sessions_manager_clone,
                    )
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

/*
 * 初始化命令集合
 */
fn init_command_strategies() -> HashMap<&'static str, Box<dyn CommandStrategy>> {
    let mut strategies: HashMap<&'static str, Box<dyn CommandStrategy>> = HashMap::new();

    strategies.insert("echo", Box::new(EchoCommand {}));
    strategies.insert("set", Box::new(SetCommand {}));
    strategies.insert("get", Box::new(GetCommand {}));
    strategies.insert("del", Box::new(DelCommand {}));
    strategies.insert("exists", Box::new(ExistsCommand {}));
    strategies.insert("expire", Box::new(ExpireCommand {}));
    strategies.insert("rename", Box::new(RenameCommand {}));
    strategies.insert("dbsize", Box::new(DBSizeCommand {}));
    strategies.insert("flushall", Box::new(FlushAllCommand {}));
    strategies.insert("flushdb", Box::new(FlushDbCommand {}));
    strategies.insert("auth", Box::new(AuthCommand {}));
    strategies.insert("select", Box::new(SelectCommand {}));
    strategies.insert("llen", Box::new(LlenCommand {}));
    strategies.insert("move", Box::new(MoveCommand {}));
    strategies.insert("keys", Box::new(KeysCommand {}));
    strategies.insert("append", Box::new(AppendCommand {}));
    strategies.insert("lpush", Box::new(LpushCommand {}));
    strategies.insert("rpush", Box::new(RpushCommand {}));
    strategies.insert("lindex", Box::new(LindexCommand {}));
    strategies.insert("lpop", Box::new(LpopCommand {}));
    strategies.insert("rpop", Box::new(RpopCommand {}));
    strategies.insert("incr", Box::new(IncrCommand {}));
    strategies.insert("decr", Box::new(DecrCommand {}));
    strategies.insert("lrange", Box::new(LrangeCommand {}));
    strategies.insert("ttl", Box::new(TtlCommand {}));

    strategies
}

// 处理 Tcp 链接
fn connection(
    mut stream: TcpStream,
    redis: Arc<Mutex<Redis>>,
    redis_config: Arc<RedisConfig>,
    session_manager: Arc<Mutex<HashMap<String, Session>>>,
) {
    
    /*
     * 声明变量
     *
     * @param command_strategies 命令集合
     * @param session_id 会话编号
     * @param buff 消息容器
     */
    let command_strategies = init_command_strategies();
    let session_id = stream.peer_addr().unwrap().to_string();
    let mut buff = [0; 512];

    {
        /*
         * 创建会话
         *
         * @param session_id 会话编号
         */
        let mut session_manager_ref = session_manager.lock().unwrap();
        session_manager_ref.insert(session_id.clone(), Session::new());
    }

    'main: loop {
        match stream.read(&mut buff) {
            Ok(size) => {
                if size == 0 {
                    break 'main;
                }

                /*
                 * 解析命令
                 *
                 * body: 消息体
                 * fragments: 消息片段
                 * command: 命令
                 */

                let body = std::str::from_utf8(&buff[..size]).unwrap();
                let fragments: Vec<&str> = body.split("\r\n").collect();
                let command = fragments[2];

                {
                    /*
                     * 安全认证【前置拦截】
                     */
                    let session_manager_ref = session_manager.lock().unwrap();
                    let session = session_manager_ref.get(&session_id).unwrap();

                    if redis_config.password != None && command != "auth" {
                        if !session.get_authenticated() {
                            let response_value = "ERR Authentication required".to_string();
                            let response_bytes = &RespValue::Error(response_value).to_bytes();
                            stream.write(response_bytes).unwrap();
                            continue 'main; // 跳过当前循环
                        }
                    }
                }

                /*
                 * 执行命令
                 *
                 * 利用策略模式，根据 command 获取具体实现，
                 * 否则响应 PONG 内容。
                 *
                 * TODO 将 所有会话 调整为 当前会话
                 */
                if let Some(strategy) = command_strategies.get(command) {
                    strategy.execute(
                        &mut stream,
                        &fragments,
                        &redis,
                        &redis_config,
                        &session_manager,
                    );
                } else {
                    let response_value = "PONG".to_string();
                    let response_bytes = &RespValue::SimpleString(response_value).to_bytes();
                    stream.write(response_bytes).unwrap();
                }
            }
            Err(_e) => {
                /*
                 * 销毁会话
                 *
                 * @param session_id 会话编号
                 */
                let mut session_manager_ref = session_manager.lock().unwrap();
                session_manager_ref.remove(&session_id);

                break 'main;
            }
        }
    }
}

// 输入启动动画
fn println_banner(project_name: &str, version: &str, port: u16) {
    let pattern = format!(r#"
     /\_____/\
    /  o   o  \          {} {}
   ( ==  ^  == )          
    )         (          Bind: 127.0.0.1:{}
   (           )          
  ( (  )   (  ) )        
 (__(__)___(__)__)
    "#, project_name ,version, port);
    println!("{}", pattern);
}