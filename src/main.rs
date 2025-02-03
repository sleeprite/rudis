use rudis_server::args::Args;
use rudis_server::db::DbManager;
use rudis_server::handler::Handler;
use std::process::id;
use std::sync::Arc;
use tokio::net::TcpListener;
use clap::Parser;

/*
 * 启动服务
 * 
 * @param args 启动参数
 */
fn server_info(args: Arc<Args>) {
    let pid = id();
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
    "#, version, args.port, pid);
    println!("{}", pattern);
}

#[tokio::main]
async fn main()  {

    let args = Arc::new(Args::parse()); 
    std::env::set_var("RUST_LOG", &args.loglevel);
    env_logger::init();

    let db_manager = Arc::new(DbManager::new(args.clone())); // 数据库管理器

    match TcpListener::bind(format!("{}:{}", args.bind, args.port)).await {
        Ok(listener) => {      
            server_info(args.clone());
            log::info!("Server initialized");
            log::info!("Ready to accept connections");
            loop { 
                match listener.accept().await {
                    Ok((stream, _address)) => {
                        let mut handler =  Handler::new(db_manager.clone(), stream, args.clone());
                        tokio::spawn(async move {
                            handler.run().await;
                        });
                    },
                    Err(e) => {  
                        log::error!("Failed to accept connection: {}", e);
                    }
                }
            }
        },
        Err(_e) => {
            log::error!("Failed to bind to address {}:{}", args.bind, args.port);
            std::process::exit(1); // 退出程序
        }
    }
}