use std::str::FromStr;

use clap::Parser;

#[derive(Parser)]
#[command(version, author, about, long_about = None)]
pub struct Args {

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
     * 文件路径
     */
    #[arg(default_value = "data/dump-{}.rdb")] 
    pub dbfilename: String,

    /**
     * 安装路径
     */
    #[arg(default_value = "./")] 
    pub dir: String,

    /**
     * 保存策略
     * 
     * Save rules in format 'seconds,changes' (e.g. '900,1 300,10')
     */
    #[clap(long, value_parser, value_delimiter = ' ', num_args = 1..)]
    pub save: Vec<SaveRule>,

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
    pub loglevel: String,
    
}

#[derive(Debug, Clone)]
pub struct SaveRule {
    pub seconds: u64,
    pub changes: u64,
}

impl FromStr for SaveRule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Err("Invalid save rule format. Expected 'seconds,changes'".into());
        }

        Ok(SaveRule {
            seconds: parts[0].parse().map_err(|e| format!("Invalid seconds: {}", e))?,
            changes: parts[1].parse().map_err(|e| format!("Invalid changes: {}", e))?,
        })
    }
}