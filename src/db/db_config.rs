use std::fs;

/*
 * Rudis 配置对象
 * 
 * @param bind 地址
 * @param port 端口
 * @param password 密码
 * @param maxclients 客户端上限
 * @param databases 初始化数据库
 * @param appendfilename 命令持久化文件
 * @param appendonly 是否开启持久化
 */
pub struct RudisConfig {
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
    pub save: Option<Vec<(u64, u64)>>,
    pub dir: String,
}

impl From<crate::tools::cli::Cli> for RudisConfig {
    fn from(value: crate::tools::cli::Cli) -> Self {
        let mut rudis_config = RudisConfig::default();
        if let Some(config) = value.config {
            if let Ok(content) = fs::read_to_string(config) {
                for line in content.lines() {
                    if let Some((key, value)) = parse_config_line(line) {
                        match key {
                            "dir" => rudis_config.dir = value.to_string(),
                            "bind" => rudis_config.bind = value.to_string(),
                            "password" => rudis_config.password = Some(value.to_string()),
                            "dbfilename" => rudis_config.dbfilename = Some(value.to_string()),
                            "port" => rudis_config.port = value.parse().unwrap_or(rudis_config.port),
                            "databases" => rudis_config.databases = value.parse().unwrap_or(rudis_config.databases),
                            "maxclients" => rudis_config.maxclients = value.parse().unwrap_or(rudis_config.maxclients),
                            "appendonly" => rudis_config.appendonly = value.parse().unwrap_or(rudis_config.appendonly),
                            "appendfilename" => rudis_config.appendfilename = Some(value.to_string()),
                            "appendfsync" => rudis_config.appendfsync = Some(value.to_string()),
                            "hz" => rudis_config.hz = value.parse().unwrap_or(rudis_config.hz),
                            "save" => rudis_config.save = parse_save(value),
                            _ => {}
                        }
                    }
                }
            }
        }

        if let Some(bind) = value.bind {
            rudis_config.bind = bind;
        }
        if let Some(port) = value.port {
            rudis_config.port = port;
        }
        if let Some(password) = value.password {
            rudis_config.password = Some(password);
        }
        if let Some(databases) = value.databases {
            rudis_config.databases = databases;
        }
        if let Some(dbfilename) = value.dbfilename {
            rudis_config.dbfilename = Some(dbfilename);
        }
        if let Some(appendfilename) = value.appendfilename {
            rudis_config.appendfilename = Some(appendfilename);
        }
        if let Some(appendonly) = value.appendonly {
            rudis_config.appendonly = appendonly == "true";
        }
        if let Some(hz) = value.hz {
            rudis_config.hz = hz;
        }
        if let Some(appendfsync) = value.appendfsync {
            rudis_config.appendfsync = Some(appendfsync);
        }
        if let Some(maxclients) = value.maxclients {
            rudis_config.maxclients = maxclients;
        }
        if !value.save.is_empty() {
            rudis_config.save = Some(value.save);
        }
        if let Some(dir) = value.dir {
            rudis_config.dir = dir.to_str().unwrap().to_string();
        }
        rudis_config
    }
}

impl Default for RudisConfig {
    fn default() -> Self {
        Self {
            bind: "0.0.0.0".to_string(),
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

fn parse_config_line(line: &str) -> Option<(&str, &str)> {
    let line = line.trim();
    if line.starts_with('#') {
        return None;
    }
    let parts: Vec<&str> = line.splitn(2, '=').map(|s| s.trim()).collect();
    if parts.len() == 2 {
        Some((parts[0], parts[1]))
    } else {
        None
    }
}

fn parse_save(value: &str) -> Option<Vec<(u64, u64)>> {
    let mut vec = Vec::new();
    let parts = value.split_whitespace();
    for part in parts {
        if let Some(pos) = part.find('/') {
            if let (Ok(i), Ok(c)) = (part[..pos].parse(), part[pos + 1..].parse()) {
                vec.push((i, c));
            }
        } else {
            return None;
        }
    }
    if !vec.is_empty() {
        return Some(vec);
    }
    None
}
