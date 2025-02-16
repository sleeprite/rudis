use std::process::id;
use std::sync::Arc;
use tokio::net::TcpListener;

use crate::args::Args;
use crate::db::DbManager;
use crate::server_handler::ServerHandler;

pub struct Server {
    args: Arc<Args>,
    db_manager: Arc<DbManager>,
}

impl Server {

    pub fn new(args: Arc<Args>, db_manager: Arc<DbManager>) -> Self {
        Server { args, db_manager }
    }

    pub async fn start(&self) {
        match TcpListener::bind(format!("{}:{}", self.args.bind, self.args.port)).await {
            Ok(listener) => {
                self.server_info();
                log::info!("Server initialized");
                log::info!("Ready to accept connections");
                loop {
                    match listener.accept().await {
                        Ok((stream, _address)) => {
                            let mut handler = ServerHandler::new(self.db_manager.clone(), stream, self.args.clone());
                            tokio::spawn(async move {
                                handler.handle().await;
                            });
                        }
                        Err(e) => {
                            log::error!("Failed to accept connection: {}", e);
                        }
                    }
                }
            }
            Err(_e) => {
                log::error!("Failed to bind to address {}:{}", self.args.bind, self.args.port);
                std::process::exit(1);
            }
        }
    }

    fn server_info(&self) {
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
        "#, version, self.args.port, pid);
        println!("{}", pattern);
    }
}