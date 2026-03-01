use std::str::FromStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use clap::Parser;

#[derive(Parser)]
#[command(version, author, about, long_about = None)]
pub struct Args {

    /// 配置文件路径
    #[arg(short, long, default_value = "rudis.conf")]
    pub config: String,

    /// 认证密码
    #[arg(long)] 
    pub requirepass: Option<String>,

    /// 绑定地址
    #[arg(short, long, default_value = "0.0.0.0")]
    pub bind: String,

    /// 文件路径
    #[arg(default_value = "data/dump.rdb")] 
    pub dbfilename: String,

    /// 安装路径
    #[arg(default_value = "./")] 
    pub dir: String,

    /// 保存策略
    /// Save rules in format 'seconds,changes' (e.g. '900,1 300,10')
    #[clap(long, value_parser, value_delimiter = ' ', num_args = 1..)]
    pub save: Vec<SaveRule>,

    /// 数据库
    #[arg(short, long, default_value = "16")]
    pub databases: usize,

    /// 监听频率
    #[arg(long, default_value = "10.0")]
    pub hz: f64,

    /// 监听端口
    #[arg(short, long, default_value = "6379")]
    pub port: String,

    /// 当前节点类型指示（用于标识节点在 Redis 集群架构中的角色）
    /// 若为 None，则表明此节点为主节点；反之，若存在值，则表示该节点是从节点，其值对应所从属的主节点的相关信息
    #[arg(long)] 
    pub replicaof: Option<String>,

    /// 日志级别
    #[arg(short, long, default_value = "info")] 
    pub loglevel: String,

    /// 持久化配置 - 是否开启
    #[arg(long, default_value = "no")] 
    pub appendonly: String,

    /// 持久化配置 - 数据文件名称
    #[arg(long, default_value = "data/dump.aof")] 
    pub appendfilename: String,

    /// 持久化配置 - 持久化方式
    #[arg(long, default_value = "always")] 
    pub appendfsync: String,

    /// 最大客户端连接数
    #[arg(long, default_value = "0")]
    pub maxclients: usize,

    /// Web 管理界面端口
    #[arg(long, default_value = "7379")]
    pub webport: u16,

    /// Web 管理界面用户名
    #[arg(long, default_value = "admin")]
    pub webuser: String,

    /// Web 管理界面密码
    #[arg(long, default_value = "admin")]
    pub webpass: String,
}

impl Args {

    pub fn is_master(&self) -> bool {
        self.replicaof.is_none()
    }
    
    pub fn is_slave(&self) -> bool {
        self.replicaof.is_some()
    }

    /// 从配置文件中加载配置
    pub fn load() -> Self {
        let mut args = Args::parse();
        if let Ok(config_map) = parse_config_file(&args.config) {
            args.merge_config(config_map);
        }
        args
    }
    
    /// 合并配置文件中的配置
    fn merge_config(&mut self, config_map: HashMap<String, String>) {
       
        // requirepass
        if self.requirepass.is_none() {
            if let Some(pass) = config_map.get("requirepass") {
                self.requirepass = Some(pass.clone());
            }
        }
        
        // bind
        if self.bind == "0.0.0.0" { 
            if let Some(bind) = config_map.get("bind") {
                self.bind = bind.clone();
            }
        }
        
        // dbfilename
        if self.dbfilename == "data/dump.rdb" { 
            if let Some(filename) = config_map.get("dbfilename") {
                self.dbfilename = filename.clone();
            }
        }
        
        // dir
        if self.dir == "./" { 
            if let Some(dir) = config_map.get("dir") {
                self.dir = dir.clone();
            }
        }
        
        // save - 只有在命令行未设置时才使用配置文件的值
        if self.save.is_empty() {
            if let Some(save_rules) = config_map.get("save") {
                self.save = save_rules
                    .split_whitespace()
                    .filter_map(|s| SaveRule::from_str(s).ok())
                    .collect();
            }
        }
        
        // databases
        if self.databases == 16 { 
            if let Some(db) = config_map.get("databases") {
                if let Ok(db_num) = db.parse() {
                    self.databases = db_num;
                }
            }
        }
        
        // hz
        if (self.hz - 10.0).abs() < f64::EPSILON { 
            if let Some(hz) = config_map.get("hz") {
                if let Ok(hz_val) = hz.parse() {
                    self.hz = hz_val;
                }
            }
        }
        
        // port
        if self.port == "6379" { 
            if let Some(port) = config_map.get("port") {
                self.port = port.clone();
            }
        }
        
        // replicaof
        if self.replicaof.is_none() {
            if let Some(replicaof) = config_map.get("replicaof") {
                self.replicaof = Some(replicaof.clone());
            }
        }
        
        // loglevel
        if self.loglevel == "info" { 
            if let Some(level) = config_map.get("loglevel") {
                self.loglevel = level.clone();
            }
        }
        
        // appendonly
        if self.appendonly == "no" { 
            if let Some(ao) = config_map.get("appendonly") {
                self.appendonly = ao.clone();
            }
        }
        
        // appendfilename
        if self.appendfilename == "data/dump.aof" { 
            if let Some(af) = config_map.get("appendfilename") {
                self.appendfilename = af.clone();
            }
        }
        
        // appendfsync
        if self.appendfsync == "always" { 
            if let Some(afs) = config_map.get("appendfsync") {
                self.appendfsync = afs.clone();
            }
        }
        
        // maxclients
        if self.maxclients == 0 { 
            if let Some(maxclients) = config_map.get("maxclients") {
                if let Ok(mc) = maxclients.parse() {
                    self.maxclients = mc;
                }
            }
        }

        // webport
        if self.webport == 7379 {
            if let Some(wp) = config_map.get("webport") {
                if let Ok(port) = wp.parse() {
                    self.webport = port;
                }
            }
        }

        // webuser
        if self.webuser == "admin" {
            if let Some(user) = config_map.get("webuser") {
                self.webuser = user.clone();
            }
        }

        // webpass
        if self.webpass == "admin" {
            if let Some(pass) = config_map.get("webpass") {
                self.webpass = pass.clone();
            }
        }
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

fn parse_config_file(filename: &str) -> Result<HashMap<String, String>, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut config_map = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = parse_config_line(line) {
            config_map.insert(key, value);
        }
    }
    
    Ok(config_map)
}

fn parse_config_line(line: &str) -> Option<(String, String)> {
    
    // 移除行内注释
    let line = match line.find('#') {
        Some(pos) => &line[..pos],
        None => line,
    };
    
    let mut iter = line.splitn(2, |c: char| c.is_whitespace());
    let key = iter.next()?.trim();
    let val = iter.next()?.trim();
    
    if key.is_empty() || val.is_empty() {
        return None;
    } else {
        return Some((key.to_string(), val.to_string()));
    }
}