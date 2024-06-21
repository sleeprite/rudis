use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use std::process::id;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

mod persistence;
mod command;
mod db;
mod interface;
mod session;
mod tools;
mod command_strategies;

use command_strategies::init_command_strategies;
use persistence::rdb::RDB;
use tools::resp::RespValue;

use crate::persistence::aof::AOF;
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
    let string_addr = format!("{}:{}", redis_config.bind, port);
    let socket_addr = match string_addr.to_socket_addrs() {
        Ok(mut addr_iter) => addr_iter.next().unwrap(),
        Err(e) => {
            eprintln!("Failed to resolve bind address: {}", e);
            return;
        }
    };
    let address = SocketAddr::new(socket_addr.ip(), socket_addr.port());
    let sessions: Arc<Mutex<HashMap<String, Session>>> = Arc::new(Mutex::new(HashMap::new()));
    let redis = Arc::new(Mutex::new(Redis::new(redis_config.clone())));
    let listener = TcpListener::bind(address).unwrap();
    
    /*
     * 根据 appendfsync 配置，创建 append_only_file 实例 【always】【everysec】
     */
    let append_only_file = Arc::new(Mutex::new(AOF::new(
        redis_config.clone(),
        redis.clone(),
    )));

    let rdb = Arc::new(Mutex::new(RDB::new(
        redis_config.clone(),
        redis.clone(),
    )));

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

    let rc = Arc::clone(&redis);
    let rcc = Arc::clone(&redis_config);

    thread::spawn(move || {
        loop {
            rc.lock().unwrap().check_all_database_ttl();
            thread::sleep(Duration::from_secs(1 / rcc.hz));
        }
    });


    if let Some(save_interval) = &redis_config.save {
        if let Ok(interval) = save_interval.parse::<u64>() {
            thread::spawn(move || {
                loop {
                    rdb.lock().unwrap().save();
                    thread::sleep(Duration::from_secs(interval));
                }
            });      
        }
    }

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

// 处理 Tcp 链接
fn connection(
    mut stream: TcpStream,
    redis: Arc<Mutex<Redis>>,
    redis_config: Arc<RedisConfig>,
    sessions: Arc<Mutex<HashMap<String, Session>>>,
    append_only_file: Arc<Mutex<AOF>>,
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
        if redis_config.maxclients == 0 || sessions_ref.len() < redis_config.maxclients {
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

                    /*
                     * 假定是个影响内存的命令，记录到日志，
                     * 【备份与恢复】中的 “备份”。
                     */
                    match strategy.command_type() {
                        CommandType::Write => {
                            match append_only_file.lock() {
                                Ok(mut append_only_file_ref) => {
                                    append_only_file_ref.save(&fragments.join("\\r\\n"));
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

/*
 * 启动服务
 */
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
