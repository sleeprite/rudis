use clap::Parser;
use rudis_server::args::Args;
use rudis_server::db::DbManager;
use rudis_server::network::server::Server;
use std::sync::Arc;

#[tokio::main]
async fn main() {

    let args = Arc::new(Args::parse());
    std::env::set_var("RUST_LOG", &args.loglevel);
    env_logger::init();

    let db_manager = Arc::new(DbManager::new(args.clone()));
    let server = Server::new(args.clone(), db_manager);
    server.start().await;
}