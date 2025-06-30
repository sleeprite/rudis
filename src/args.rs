use std::str::FromStr;

use clap::Parser;

#[derive(Parser)]
#[command(version, author, about, long_about = None)]
pub struct Args {

    /**
     * 认证密码
     */
    #[arg(long)] 
    pub requirepass: Option<String>,

    /**
     * 绑定地址
     */
    #[arg(short, long, default_value = "127.0.0.1")]
    pub bind: String,

    /**
     * 文件路径
     */
    #[arg(default_value = "data/dump.rdb")] 
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
     * 监听频率
     */
    #[arg(long, default_value = "10.0")]
    pub hz: f64,

    /**
     * 监听端口
     */
    #[arg(short, long, default_value = "6379")]
    pub port: String,

    /**
     * 当前节点类型指示（用于标识节点在 Redis 集群架构中的角色）
     * 
     * 若为 None，则表明此节点为主节点；反之，若存在值，则表示该节点是从节点，其值对应所从属的主节点的相关信息
     */
    #[arg(long)] 
    pub replicaof: Option<String>,

    /**
     * 日志级别
     */
    #[arg(short, long, default_value = "info")] 
    pub loglevel: String,
    
}

impl Args {

    pub fn is_master(&self) -> bool {
        self.replicaof.is_none()
    }
    
    pub fn is_slave(&self) -> bool {
        self.replicaof.is_some()
    }
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