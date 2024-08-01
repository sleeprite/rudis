use std::fs;

/*
 * Redis 配置
 * @param bind 地址
 * @param port 端口
 * @param password 密码
 * @param maxclients 客户端上限
 * @param databases 初始化数据库
 * @param appendfilename 命令持久化文件
 * @param appendonly 是否开启持久化
 */
pub struct RedisConfig {
    pub bind: String,
    pub port: u16,
    pub password: Option<String>,
    pub databases: usize,
    pub dbfilename: Option<String>,
    pub appendfilename: Option<String>,
    pub appendonly: bool,
    pub hz: u64,
    pub appendfsync: Option<String>,
    pub maxclients: usize,
    pub save: Option<String>,
    pub dir: String,
}
// dbfilename dump.rdb

impl From<crate::tools::cli::Cli> for RedisConfig {
    fn from(value: crate::tools::cli::Cli) -> Self {
        let mut redis_config = RedisConfig::default();
        if let Some(config) = value.config {
            if let Ok(content) = fs::read_to_string(config) {
                for line in content.lines() {
                    if let Some((key, value)) = parse_config_line(line) {
                        match key.as_str() {
                            "dir" => redis_config.dir = value.to_string(),
                            "port" => {
                                redis_config.port = value.parse().unwrap_or(redis_config.port)
                            }
                            "bind" => redis_config.bind = value.to_string(),
                            "password" => redis_config.password = Some(value.to_string()),
                            "dbfilename" => redis_config.dbfilename = Some(value.to_string()),
                            "databases" => {
                                redis_config.databases =
                                    value.parse().unwrap_or(redis_config.databases)
                            }
                            "maxclients" => {
                                redis_config.maxclients =
                                    value.parse().unwrap_or(redis_config.maxclients)
                            }
                            "appendonly" => {
                                redis_config.appendonly =
                                    value.parse().unwrap_or(redis_config.appendonly)
                            }
                            "appendfilename" => {
                                redis_config.appendfilename = Some(value.to_string())
                            }
                            "appendfsync" => redis_config.appendfsync = Some(value.to_string()),
                            "save" => redis_config.save = Some(value.to_string()),
                            "hz" => redis_config.hz = value.parse().unwrap_or(redis_config.hz),
                            _ => {}
                        }
                    }
                }
            }
        }

        if let Some(bind) = value.bind {
            redis_config.bind = bind;
        }
        if let Some(port) = value.port {
            redis_config.port = port;
        }
        if let Some(password) = value.password {
            redis_config.password = Some(password);
        }
        if let Some(databases) = value.databases {
            redis_config.databases = databases;
        }
        if let Some(dbfilename) = value.dbfilename {
            redis_config.dbfilename = Some(dbfilename);
        }
        if let Some(appendfilename) = value.appendfilename {
            redis_config.appendfilename = Some(appendfilename);
        }
        if let Some(appendonly) = value.appendonly {
            if appendonly == "true" {
                redis_config.appendonly = true;
            } else {
                redis_config.appendonly = false;
            }
        }
        if let Some(hz) = value.hz {
            redis_config.hz = hz;
        }
        if let Some(appendfsync) = value.appendfsync {
            redis_config.appendfsync = Some(appendfsync);
        }
        if let Some(maxclients) = value.maxclients {
            redis_config.maxclients = maxclients;
        }
        if value.save.len() > 0 {
            redis_config.save = Some(
                value
                    .save
                    .iter()
                    .map(|x| format!("{} {}", x.0, x.1))
                    .collect::<Vec<String>>()
                    .join(" "),
            );
        }
        if let Some(dir) = value.dir {
            redis_config.dir = dir.to_str().unwrap().to_string();
        }
        redis_config
    }
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            bind: "127.0.0.1".to_string(),
            port: 6379,
            password: None,
            databases: 16,
            dbfilename: Some("dump.rdb".to_string()),
            appendfilename: Some("appendonly.aof".to_string()),
            appendonly: false,
            hz: 10,
            appendfsync: None,
            maxclients: 0,
            save: None,
            dir: "./".to_string(),
        }
    }
}

fn parse_config_line(line: &str) -> Option<(String, String)> {
    let line = line.trim();
    if line.starts_with('#') {
        return None;
    }
    let parts: Vec<&str> = line.splitn(2, '=').map(|s| s.trim()).collect();
    if parts.len() == 2 {
        Some((parts[0].to_string(), parts[1].to_string()))
    } else {
        None
    }
}
