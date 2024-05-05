use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::process::id;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

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

use command::hash::hset::HsetCommand;
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
    let append_only_file = Arc::new(Mutex::new(AppendOnlyFile::new(
        redis_config.clone(),
        redis.clone(),
    )));
    let listener = TcpListener::bind(address).unwrap();

    println_banner(port);

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

    /*
     * 内存守护线程 
     */
    let redis_c = Arc::clone(&redis);
    thread::spawn(move || {
        loop {
            redis_c.lock().unwrap().check_all_database_ttl();
            thread::sleep(Duration::from_secs(1));
        }
    });

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

    strategies.insert("ECHO", Box::new(EchoCommand {}));
    strategies.insert("SET", Box::new(SetCommand {}));
    strategies.insert("GET", Box::new(GetCommand {}));
    strategies.insert("DEL", Box::new(DelCommand {}));
    strategies.insert("EXISTS", Box::new(ExistsCommand {}));
    strategies.insert("EXPIRE", Box::new(ExpireCommand {}));
    strategies.insert("RENAME", Box::new(RenameCommand {}));
    strategies.insert("DBSIZE", Box::new(DBSizeCommand {}));
    strategies.insert("FLUSHALL", Box::new(FlushAllCommand {}));
    strategies.insert("FLUSHDB", Box::new(FlushDbCommand {}));
    strategies.insert("SELECT", Box::new(SelectCommand {}));
    strategies.insert("AUTH", Box::new(AuthCommand {}));
    strategies.insert("LLEN", Box::new(LlenCommand {}));
    strategies.insert("MOVE", Box::new(MoveCommand {}));
    strategies.insert("KEYS", Box::new(KeysCommand {}));
    strategies.insert("APPEND", Box::new(AppendCommand {}));
    strategies.insert("LPUSH", Box::new(LpushCommand {}));
    strategies.insert("RPUSH", Box::new(RpushCommand {}));
    strategies.insert("LINDEX", Box::new(LindexCommand {}));
    strategies.insert("LPOP", Box::new(LpopCommand {}));
    strategies.insert("RPOP", Box::new(RpopCommand {}));
    strategies.insert("INCR", Box::new(IncrCommand {}));
    strategies.insert("DECR", Box::new(DecrCommand {}));
    strategies.insert("PTTL", Box::new(PttlCommand {}));
    strategies.insert("TYPE", Box::new(TypeCommand {}));
    strategies.insert("SADD", Box::new(SaddCommand {}));
    strategies.insert("SMEMBERS", Box::new(SmembersCommand {}));
    strategies.insert("LRANGE", Box::new(LrangeCommand {}));
    strategies.insert("SCARD", Box::new(ScardCommand {}));
    strategies.insert("TTL", Box::new(TtlCommand {}));
    strategies.insert("HMSET", Box::new(HmsetCommand {}));
    strategies.insert("HGET", Box::new(HgetCommand {}));
    strategies.insert("HDEL", Box::new(HdelCommand {}));
    strategies.insert("HEXISTS", Box::new(HexistsCommand {}));
    strategies.insert("HSET", Box::new(HsetCommand {}));

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
                if let Some(strategy) = command_strategies.get(command.to_uppercase().as_str()) {
                    strategy.execute(
                        Some(&mut stream),
                        &fragments,
                        &redis,
                        &redis_config,
                        &sessions,
                        &session_id,
                    );
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
fn println_banner(port: u16) {
    let version = env!("CARGO_PKG_VERSION");
    let pattern = format!(
    r#"
     /\_____/\
    /  o   o  \          Rudis {}
   ( ==  ^  == )          
    )         (          Bind: {} PID: {} 
   (           )          
  ( (  )   (  ) )        
 (__(__)___(__)__)
    "#,version, port, id());
    println!("{}", pattern);
}
