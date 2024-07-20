use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use std::sync::{Arc, Mutex};
use std::process::id;

use tokio::time::Duration;

mod persistence;
mod command;
mod db;
mod interface;
mod session;
mod tools;
mod command_strategies;

use persistence::rdb::Rdb;
use persistence::rdb_count::RdbCount;
use persistence::rdb_scheduler::RdbScheduler;
use command_strategies::init_command_strategies;
use tools::resp::RespValue;

use crate::db::db::Redis;
use crate::db::db_config::RedisConfig;
use crate::interface::command_type::CommandType;
use crate::session::session::Session;
use crate::persistence::aof::Aof;

#[tokio::main]
async fn main() {
    
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
    
    let aof = Arc::new(Mutex::new(Aof::new(
        redis_config.clone(),
        redis.clone(),
    )));

    let rdb = Arc::new(Mutex::new(Rdb::new(
        redis_config.clone(),
        redis.clone(),
    )));

    println_banner(port);

    if redis_config.appendonly {
        match aof.lock() {
            Ok(mut file) => {
                log::info!("Start loading appendfile");
                file.load();
            }
            Err(err) => {
                eprintln!("Failed to acquire lock: {:?}", err);
                return;
            }
        }
    } else {
        match rdb.lock() {
            Ok(mut file) => {
                log::info!("Start loading dump.rdb");
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

    // 检测过期
    tokio::spawn(async move {
        loop {
            rc.lock().unwrap().check_all_database_ttl();
            tokio::time::sleep(Duration::from_secs(1 / rcc.hz)).await;
        }
    });

    // 保存策略
    let arc_rdb_count = Arc::new(Mutex::new(RdbCount::new()));
    let arc_rdb_scheduler = Arc::new(Mutex::new(RdbScheduler::new(rdb)));
    if let Some(save_interval) = &redis_config.save {
        arc_rdb_scheduler.lock().unwrap().execute(save_interval, arc_rdb_count.clone());  
    }

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let redis_clone = Arc::clone(&redis);
                let redis_config_clone = Arc::clone(&redis_config);
                let sessions_clone = Arc::clone(&sessions);
                let rdb_count_clone = Arc::clone(&arc_rdb_count);
                let aof_clone = Arc::clone(&aof);
                tokio::spawn(async move {
                    connection(
                        stream,
                        redis_clone,
                        redis_config_clone,
                        sessions_clone,
                        rdb_count_clone,
                        aof_clone,
                    );
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
    rdb_count: Arc<Mutex<RdbCount>>,
    aof: Arc<Mutex<Aof>>,
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
            match stream.write(&resp_value) {
                Ok(_bytes_written) => {
                    // END
                },
                Err(e) => {
                    eprintln!("Failed to write to stream: {}", e);
                },
            }
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
                     * 
                     * 如果配置了密码，该命令不是 auth 指令，且用户未登录
                     */
                    let sessions_ref = sessions.lock().unwrap();
                    let session = sessions_ref.get(&session_id).unwrap();
                    let is_not_auth_command = command.to_uppercase() != "AUTH";
                    let is_not_auth = !session.get_authenticated();
                    if redis_config.password.is_some() && is_not_auth && is_not_auth_command{
                        let response_value = "ERR Authentication required".to_string();
                        let response_bytes = &RespValue::Error(response_value).to_bytes();
                        match stream.write(response_bytes){
                            Ok(_bytes_written) => {
                                // Response successful
                            },
                            Err(e) => {
                                eprintln!("Failed to write to stream: {}", e);
                            },
                        };
                        continue 'main;
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
                     *【备份与恢复】中的恢复。
                     */
                    if let CommandType::Write = strategy.command_type() {                      
                        rdb_count.lock().unwrap().calc();
                        match aof.lock() {
                            Ok(mut aof_ref) => {
                                aof_ref.save(&fragments.join("\\r\\n"));
                            }
                            Err(_) => {
                                eprintln!("Failed to acquire lock on AOF");
                                return;
                            }
                        };
                    }
                } else {
                    let response_value = "PONG".to_string();
                    let response_bytes = &RespValue::SimpleString(response_value).to_bytes();
                    match stream.write(response_bytes) {
                        Ok(_bytes_written) => {
                            // END
                        },
                        Err(e) => {
                            eprintln!("Failed to write to stream: {}", e);
                        },
                    }
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
