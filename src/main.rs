use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use std::process::id;
use std::sync::Arc;
use parking_lot::Mutex;

use clap::Parser;
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
use crate::session::session::Session;

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
    let address = SocketAddr::new(socket_addr.ip(), socket_addr.port());
    let sessions: Arc<Mutex<HashMap<String, Session>>> = Arc::new(Mutex::new(HashMap::new()));
    let db = Arc::new(Mutex::new(Db::new(rudis_config.clone())));
    let aof = Arc::new(Mutex::new(Aof::new(rudis_config.clone(), db.clone())));
    let rdb = Arc::new(Mutex::new(Rdb::new(rudis_config.clone(), db.clone())));
    let listener = TcpListener::bind(address).unwrap();

    println_banner(port);

    if rudis_config.appendonly {
        aof.lock().load();
    } else {
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
                let sessions_clone = Arc::clone(&sessions);
                let rdb_count_clone = Arc::clone(&arc_rdb_count);
                let aof_clone = Arc::clone(&aof);
                tokio::spawn(async move {
                    connection(
                        stream,
                        db_clone,
                        rudis_config_clone,
                        sessions_clone,
                        rdb_count_clone,
                        aof_clone,
                    ).await;
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
    sessions: Arc<Mutex<HashMap<String, Session>>>,
    rdb_count: Arc<Mutex<RdbCount>>,
    aof: Arc<Mutex<Aof>>,
) {
    /*
     * 声明变量
     *
     * @param session_id 会话编号
     * @param command_strategies 命令集
     * @param buff_list 消息列表
     * @param read_size 读取长度
     * @param buff 缓冲区
     */
    let command_strategies = init_command_strategies();
    let session_id = stream.peer_addr().unwrap().to_string();
    let mut buff = [0; 512];
    let mut buff_list = Vec::new();
    let mut read_size = 0;

    {
        /*
         * 创建会话
         *
         * （1）判定 session 数量是否超出阈值 {maxclients}
         * （2）满足：响应 ERR max number of clients reached 错误
         * （3）否则：创建 session 会话
         */
        let mut sessions_ref = sessions.lock();
        if rudis_config.maxclients == 0 || sessions_ref.len() < rudis_config.maxclients {
            sessions_ref.insert(session_id.clone(), Session::new());
        } else {
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

                    {
                        /*
                         * 安全认证【前置拦截】
                         *
                         * 如果配置了密码，该命令不是 auth 指令，且用户未登录
                         */
                        let sessions_ref = sessions.lock();
                        let session = sessions_ref.get(&session_id).unwrap();
                        let is_not_auth_command = command.to_uppercase() != "AUTH";
                        let is_not_auth = !session.get_authenticated();
                        if rudis_config.password.is_some() && is_not_auth && is_not_auth_command {
                            let response_value = "ERR Authentication required".to_string();
                            let response_bytes = &RespValue::Error(response_value).to_bytes();
                            match stream.write(response_bytes) {
                                Ok(_bytes_written) => {}
                                Err(e) => {
                                    eprintln!("Failed to write to stream: {}", e);
                                }
                            };
                            continue 'main;
                        }
                    }

                    /*
                     * 匹配命令
                     *
                     * 利用策略模式，根据 command 获取具体实现，
                     * 否则响应 PONG 内容。
                     */
                    let uppercase_command = command.to_uppercase();
                    if let Some(strategy) = command_strategies.get(uppercase_command.as_str())
                    {
                        // 执行命令
                        strategy.execute(
                            Some(&mut stream),
                            &fragments,
                            &db,
                            &rudis_config,
                            &sessions,
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
                /*
                 * 销毁会话
                 *
                 * @param session_id 会话编号
                 */
                let mut session_manager_ref = sessions.lock();
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
    "#,
        version,
        port,
        id()
    );
    println!("{}", pattern);
}

#[cfg(test)]
mod tests {
    use crate::db::db_config::RudisConfig;
    use crate::tools::cli;
    use clap::Parser;
    #[test]
    fn test_config_file() {
        let port = 42800;
        let config_file_path = "./release/linux/rudis-server.properties";
        let arg_string = format!(
            "rudis-server \
            -p {} \
            --config {}",
            port, config_file_path
        );
        let args: Vec<&str> = arg_string.split(' ').collect();
        let cli = cli::Cli::parse_from(args);
        let config: RudisConfig = cli.into();
        assert_eq!(config.port, port);
        assert_eq!(config.maxclients, 1000);
        assert_eq!(config.password, None);
        assert_eq!(config.save.unwrap().iter().map(|x| format!("{}/{}",x.0,x.1)).collect::<Vec<String>>().join(" "), "60/1 20/2");
    }
    #[test]
    fn test_cli() {
        let bind = "192.168.1.2";
        let port = 6379;
        let password = "123456";
        let databases = 1;
        let dbfilename = "123.rdb";
        let appendfilename = "321.aof";
        let appendonly = "false";
        let hz = 2;
        let appendfsync = "asd";
        let maxclients = 100;
        let dir = "/home/rudis";
        let save_1 = "60/1";
        let save_2 = "20/2";
        let save = "60/1 20/2";
        let arg_string = format!(
            "rudis-server \
            --bind {} \
            -p {} \
            --password {} \
            --databases {} \
            --dbfilename {} \
            --appendfilename {} \
            --hz {} \
            --appendfsync {} \
            --maxclients {} \
            --dir {} \
            --save {} \
            --save {} \
            --appendonly {}",
            bind,
            port,
            password,
            databases,
            dbfilename,
            appendfilename,
            hz,
            appendfsync,
            maxclients,
            dir,
            save_1,
            save_2,
            appendonly
        );

        let args: Vec<&str> = arg_string.split(' ').collect();
        let cli = cli::Cli::parse_from(args);
        let config: RudisConfig = cli.into();
        assert_eq!(config.bind, bind.to_string());
        assert_eq!(config.port, port);
        assert_eq!(config.password, Some(password.to_string()));
        assert_eq!(config.databases, databases);
        assert_eq!(config.dbfilename, Some(dbfilename.to_string()));
        assert_eq!(config.appendfilename, Some(appendfilename.to_string()));
        assert_eq!(config.appendonly.to_string(), appendonly);
        assert_eq!(config.appendfsync, Some(appendfsync.to_string()));
        assert_eq!(config.maxclients, maxclients);
        assert_eq!(config.dir, dir.to_string());
        assert!(config.save.is_some());
        assert_eq!(config.save.unwrap().iter().map(|x| format!("{}/{}",x.0,x.1)).collect::<Vec<String>>().join(" "), save);
    }
}
