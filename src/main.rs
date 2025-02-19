use clap::Parser;
use rudis_server::args::Args;
use rudis_server::db::DbGuard;
use rudis_server::server::Server;
use std::sync::Arc;

#[tokio::main]
async fn main() {

    let args = Arc::new(Args::parse());
    std::env::set_var("RUST_LOG", &args.loglevel);
    env_logger::init();

    let db_guard = Arc::new(DbGuard::new(args.clone()));
    let server = Server::new(args.clone(), db_guard);
    server.start().await;
}