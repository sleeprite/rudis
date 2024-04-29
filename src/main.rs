use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

mod aof;
mod command;
mod db;
mod interface;
mod session;
mod tools;

use command::arr::lindex::LindexCommand;
use command::arr::llen::LlenCommand;
use command::arr::lpop::LpopCommand;
use command::arr::lpush::LpushCommand;
use command::arr::lrange::LrangeCommand;
use command::arr::rpop::RpopCommand;
use command::arr::rpush::RpushCommand;

use command::hash::hdel::HdelCommand;
use command::hash::hexists::HexistsCommand;
use command::hash::hget::HgetCommand;
use command::hash::hmset::HmsetCommand;

use command::key::del::DelCommand;
use command::key::exists::ExistsCommand;
use command::key::expire::ExpireCommand;
use command::key::keys::KeysCommand;
use command::key::pttl::PttlCommand;
use command::key::r#move::MoveCommand;
use command::key::r#type::TypeCommand;
use command::key::rename::RenameCommand;
use command::key::ttl::TtlCommand;

use command::set::sadd::SaddCommand;
use command::set::scard::ScardCommand;
use command::set::smembers::SmembersCommand;

use command::string::append::AppendCommand;
use command::string::decr::DecrCommand;
use command::string::get::GetCommand;
use command::string::incr::IncrCommand;
use command::string::set::SetCommand;

use command::auth::AuthCommand;
use command::dbsize::DBSizeCommand;
use command::echo::EchoCommand;
use command::flushall::FlushAllCommand;
use command::flushdb::FlushDbCommand;
use command::select::SelectCommand;
use interface::command_strategy::CommandStrategy;
use tools::resp::RespValue;

use crate::aof::aof::AppendOnlyFile;
use crate::db::db::Redis;
use crate::db::db_config::RedisConfig;
use crate::interface::command_type::CommandType;
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

    /*
     * 创建通讯服务
     */
    let port: u16 = redis_config.port;
    let address = SocketAddr::from(([127, 0, 0, 1], port));
    let sessions: Arc<Mutex<HashMap<String, Session>>> = Arc::new(Mutex::new(HashMap::new()));
    let redis = Arc::new(Mutex::new(Redis::new(redis_config.clone())));
    let append_only_file = Arc::new(Mutex::new(AppendOnlyFile::new(redis_config.clone(), redis.clone())));
    let listener = TcpListener::bind(address).unwrap();
    
    let project_name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    println_banner(project_name, version, port);

    /*
     * 加载本地数据
     */
    if redis_config.appendonly {
        match append_only_file.lock() {
            Ok(mut file) => {
                log::info!("Start loading appendfile");
                file.load();
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
                let sessions_clone = Arc::clone(&sessions);
                let append_only_file_clone = Arc::clone(&append_only_file);
                thread::spawn(|| {
                    connection(
                        stream,
                        redis_clone,
                        redis_config_clone,
                        sessions_clone,
                        append_only_file_clone,
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
    strategies.insert("select", Box::new(SelectCommand {}));
    strategies.insert("auth", Box::new(AuthCommand {}));
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
    strategies.insert("pttl", Box::new(PttlCommand {}));
    strategies.insert("type", Box::new(TypeCommand {}));
    strategies.insert("sadd", Box::new(SaddCommand {}));
    strategies.insert("smembers", Box::new(SmembersCommand {}));
    strategies.insert("lrange", Box::new(LrangeCommand {}));
    strategies.insert("scard", Box::new(ScardCommand {}));
    strategies.insert("ttl", Box::new(TtlCommand {}));
    strategies.insert("hmset", Box::new(HmsetCommand {}));
    strategies.insert("hget", Box::new(HgetCommand {}));
    strategies.insert("hdel", Box::new(HdelCommand {}));
    strategies.insert("hexists", Box::new(HexistsCommand {}));

    strategies
}

// 处理 Tcp 链接
fn connection(
    mut stream: TcpStream,
    redis: Arc<Mutex<Redis>>,
    redis_config: Arc<RedisConfig>,
    sessions: Arc<Mutex<HashMap<String, Session>>>,
    append_only_file: Arc<Mutex<AppendOnlyFile>>,
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
         * （1）判定 session 数量是否超出阈值 {maxclients}
         * （2）满足：响应 ERR max number of clients reached 错误
         * （3）否则：创建 session 会话
         */
        let mut sessions_ref = sessions.lock().unwrap();
        if sessions_ref.len() < redis_config.maxclients {
           sessions_ref.insert(session_id.clone(), Session::new());
        } else {
           let err = "ERR max number of clients reached".to_string();
           let resp_value = RespValue::Error(err).to_bytes();
           stream.write(&resp_value).unwrap();
           return;
        }
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

                let bytes = &buff[..size];
                let body = std::str::from_utf8(bytes).unwrap();
                let fragments: Vec<&str> = body.split("\r\n").collect();
                let command = fragments[2];

                {
                    /*
                     * 安全认证【前置拦截】
                     */
                    let sessions_ref = sessions.lock().unwrap();
                    let session = sessions_ref.get(&session_id).unwrap();

                    if redis_config.password != None && command != "auth" {
                        if !session.get_authenticated() {
                            let response_value = "ERR Authentication required".to_string();
                            let response_bytes = &RespValue::Error(response_value).to_bytes();
                            stream.write(response_bytes).unwrap();
                            continue 'main;
                        }
                    }
                }

                /*
                 * 执行命令
                 *
                 * 利用策略模式，根据 command 获取具体实现，
                 * 否则响应 PONG 内容。
                 */
                if let Some(strategy) = command_strategies.get(command) {
                    strategy.execute(Some(&mut stream), &fragments, &redis, &redis_config, &sessions, &session_id);
                    
                    match strategy.command_type() {
                        CommandType::Write => {
                            match append_only_file.lock() {
                                Ok(mut append_only_file_ref) => {
                                    append_only_file_ref.write(&fragments.join("\\r\\n"));
                                }
                                Err(_) => {
                                    eprintln!("Failed to acquire lock on append_only_file");
                                    return;
                                }
                            };
                        }
                        _ => {}
                    }
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
                let mut session_manager_ref = sessions.lock().unwrap();
                session_manager_ref.remove(&session_id);
                break 'main;
            }
        }
    }
}

// 输入启动动画
fn println_banner(project_name: &str, version: &str, port: u16) {
    let pattern = format!(
        r#"
     /\_____/\
    /  o   o  \          {} {}
   ( ==  ^  == )          
    )         (          Bind: 127.0.0.1:{}
   (           )          
  ( (  )   (  ) )        
 (__(__)___(__)__)
    "#,
        project_name, version, port
    );
    println!("{}", pattern);
}
