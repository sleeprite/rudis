use clap::Parser;
use rudis_server::args::Args;
use rudis_server::server::Server;
use std::process::id;
use std::sync::Arc;

#[tokio::main]
async fn main() {

    let args = Arc::new(Args::parse());
    std::env::set_var("RUST_LOG", &args.loglevel);
    env_logger::init();

    server_info(args.clone());
    let mut server = Server::new(args.clone());
    server.start().await;
}

fn server_info(args: Arc<Args>) {
    let pid = id();
    let version = env!("CARGO_PKG_VERSION");
    let role = if args.is_slave() { "slave" } else { "master" };
    let pattern = format!(r#"
         /\_____/\
        /  o   o  \          Rudis {}
       ( ==  ^  == )
        )         (          Bind: {} PID: {}
       (           )
      ( (  )   (  ) )        Role: {}
     (__(__)___(__)__)

    Rudis is a high-performance in memory database.
    "#, version, args.port, pid, role);
    println!("{}", pattern);
}