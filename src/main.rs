use clap::Parser;
use rudis_server::config::Config;
use rudis_server::db::DbManager;
use rudis_server::server::Server;
use std::sync::Arc;

#[tokio::main]
async fn main() {

    let config = Arc::new(Config::parse());
    std::env::set_var("RUST_LOG", &config.loglevel);
    env_logger::init();

    let db_manager = Arc::new(DbManager::new(config.clone()));
    let server = Server::new(config.clone(), db_manager);
    server.start().await;
}