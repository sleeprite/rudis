use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::process::id;
use std::sync::Arc;
use parking_lot::Mutex;

use clap::Parser;
use session::session_manager::SessionManager;
use tokio::time::Duration;

mod command;
mod command_strategies;
mod db;
mod interface;
mod persistence;
mod session;
mod tools;

use command_strategies::init_command_strategies;
use persistence::rdb::Rdb;
use persistence::rdb_count::RdbCount;
use persistence::rdb_scheduler::RdbScheduler;
use tools::resp::RespValue;

use crate::db::db::Db;
use crate::db::db_config::RudisConfig;
use crate::interface::command_type::CommandType;
use crate::persistence::aof::Aof;


#[tokio::main]
async fn main() {

    // 启动参数解析
    let cli = crate::tools::cli::Cli::parse();

    /*
     * 初始日志框架
     *
     * (1) 日志级别
     * (2) 框架加载
     */
    std::env::set_var("RUST_LOG", cli.log_level.as_str().to_lowercase());
    env_logger::init();

    /*
     * 创建默认配置
     */
    let rudis_config: Arc<RudisConfig> = Arc::new(cli.into());

    /*
     * 创建通讯服务
     */
    let port: u16 = rudis_config.port;
    let string_addr = format!("{}:{}", rudis_config.bind, port);
    let socket_addr = match string_addr.to_socket_addrs() {
        Ok(mut addr_iter) => addr_iter.next().unwrap(),
        Err(e) => {
            eprintln!("Failed to resolve bind address: {}", e);
            return;
        }
    };
    let db = Arc::new(Mutex::new(Db::new(rudis_config.clone())));
    let aof = Arc::new(Mutex::new(Aof::new(rudis_config.clone(), db.clone())));
    let rdb = Arc::new(Mutex::new(Rdb::new(rudis_config.clone(), db.clone())));
    let session_manager = Arc::new(SessionManager::new(rudis_config.clone()));
    let listener = TcpListener::bind(socket_addr).unwrap();

    println_banner(port);

    // 数据恢复
    if rudis_config.appendonly {
        log::info!("Start performing AOF recovery");
        aof.lock().load();
    } else {
        log::info!("Start performing RDB recovery");
        rdb.lock().load();
    }

    log::info!("Server initialized");
    log::info!("Ready to accept connections");

    let rc = Arc::clone(&db);
    let rcc = Arc::clone(&rudis_config);

    // 检测过期
    tokio::spawn(async move {
        loop {
            rc.lock().check_all_database_ttl();
            tokio::time::sleep(Duration::from_secs(1 / rcc.hz)).await;
        }
    });

    // 保存策略
    let arc_rdb_count = Arc::new(Mutex::new(RdbCount::new()));
    let arc_rdb_scheduler = Arc::new(Mutex::new(RdbScheduler::new(rdb)));
    if let Some(save_interval) = &rudis_config.save {
        arc_rdb_scheduler.lock().execute(save_interval.clone(), arc_rdb_count.clone());
    }

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let db_clone = Arc::clone(&db);
                let rudis_config_clone = Arc::clone(&rudis_config);
                let session_manager_clone = Arc::clone(&session_manager);
                let rdb_count_clone = Arc::clone(&arc_rdb_count);
                let aof_clone = Arc::clone(&aof);
                tokio::spawn(async move {
                    connection(
                        stream,
                        db_clone,
                        rudis_config_clone,
                        session_manager_clone,
                        rdb_count_clone,
                        aof_clone,
                    )
                    .await;
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

// 处理 Tcp 链接
async fn connection(
    mut stream: TcpStream,
    db: Arc<Mutex<Db>>,
    rudis_config: Arc<RudisConfig>,
    session_manager: Arc<SessionManager>,
    rdb_count: Arc<Mutex<RdbCount>>,
    aof: Arc<Mutex<Aof>>,
) {

    /* 
     * 声明变量
     *
     * command_strategies 命令集
     * session_id 会话编号
     * buff 消息
     * buff_list 完整消息
     * read_size 总读取长度
     */
    let command_strategies = init_command_strategies();
    let session_id = stream.peer_addr().unwrap().to_string();
    let mut buff = [0; 512];
    let mut buff_list = Vec::new();
    let mut read_size = 0;

    /*
     * 创建会话
     */
    if !session_manager.create_session(session_id.clone()) {
        let err = "ERR max number of clients reached".to_string();
        let resp_value = RespValue::Error(err).to_bytes();
        match stream.write(&resp_value) {
            Ok(_bytes_written) => {}
            Err(e) => {
                eprintln!("Failed to write to stream: {}", e);
            }
        }
        return;
    }

    'main: loop {
        match stream.read(&mut buff) {
            Ok(size) => {
                if size == 0 {
                    break 'main;
                }

                buff_list.extend_from_slice(&buff[..size]);
                read_size += size;

                if size < 512 {
                    
                    /*
                     * 解析命令
                     *
                     * body: 消息体
                     * fragments: 消息片段
                     * command: 命令
                     */
                    let bytes = &buff_list[..read_size];
                    let body = std::str::from_utf8(bytes).unwrap();
                    let fragments: Vec<&str> = body.split("\r\n").collect();
                    let command = fragments[2];

                    /*
                     * 安全认证【前置拦截】
                     *
                     * 如果配置了密码，该命令不是 auth 指令，且用户未登录
                     */
                    let is_authenticated = session_manager.authenticate(&session_id, command);
                    if !is_authenticated {
                        let err = "ERR Authentication required".to_string();
                        let response_bytes = &RespValue::Error(err).to_bytes();
                        match stream.write(response_bytes) {
                            Ok(_bytes_written) => {
                                buff_list.clear();
                                buff_list.shrink_to_fit();
                                read_size = 0;
                            }
                            Err(e) => {
                                eprintln!("Failed to write to stream: {}", e);
                            }
                        };
                        continue 'main;
                    }

                    /*
                     * 匹配命令
                     *
                     * 利用策略模式，根据 command 获取具体实现，
                     * 否则响应 PONG 内容。
                     */
                    let uppercase_command = command.to_uppercase();
                    if let Some(strategy) = command_strategies.get(uppercase_command.as_str()) {
                        
                        /*
                         * 执行命令
                         * 
                         * @param stream 流
                         * @param db
                         * @param rudis_config 配置文件
                         * @param sessions 会话列表
                         * @param session_id
                         */
                        strategy.execute(
                            Some(&mut stream),
                            &fragments,
                            &db,
                            &rudis_config,
                            &session_manager.get_sessions(),
                            &session_id,
                        );

                        /*
                         * 假定是个影响内存的命令，记录到日志，
                         *【备份与恢复】中的恢复。
                         */
                        if let CommandType::Write = strategy.command_type() {
                            rdb_count.lock().calc();
                            aof.lock().save(&fragments.join("\\r\\n"));
                        }
                    } else {

                        // 未知的命令
                        let response_value = "PONG".to_string();
                        let response_bytes = &RespValue::SimpleString(response_value).to_bytes();
                        match stream.write(response_bytes) {
                            Ok(_bytes_written) => {}
                            Err(e) => {
                                eprintln!("Failed to write to stream: {}", e);
                            }
                        }
                    }

                    /*
                     * 完成命令的执行后
                     *
                     * buff_list.shrink_to_fit(); 释放内存
                     */
                    buff_list.clear();
                    buff_list.shrink_to_fit();
                    read_size = 0;
                }
            }
            Err(_e) => {
                session_manager.destroy_session(&session_id);
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
    "#, version, port, id());
    println!("{}", pattern);
}