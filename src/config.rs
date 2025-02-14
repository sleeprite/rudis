use clap::Parser;

#[derive(Parser)]
#[command(version, author, about, long_about = None)]
pub struct Config {

    /**
     * 认证密码
     */
    #[arg(short, long)] 
    pub requirepass: Option<String>,

    /**
     * 绑定地址
     */
    #[arg(short, long, default_value = "127.0.0.1")] 
    pub bind: String,

    /**
     * 数据库
     */
    #[arg(short, long, default_value = "16")]
    pub databases: usize,

    /**
     * 监听端口
     */
    #[arg(short, long, default_value = "6379")]
    pub port: String,

    /**
     * 日志级别
     */
    #[arg(short, long, default_value = "info")] 
    pub loglevel: String

}