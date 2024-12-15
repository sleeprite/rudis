use clap::Parser;

#[derive(Parser)]
#[command(version, author, about, long_about = None)]
pub struct Args {

    #[arg(short, long)] 
    pub requirepass: Option<String>,

    #[arg(short, long, default_value = "16")]
    pub databases: usize,

    #[arg(short, long, default_value = "127.0.0.1")] 
    pub bind: String,

    #[arg(short, long, default_value = "6379")]
    pub port: String
}