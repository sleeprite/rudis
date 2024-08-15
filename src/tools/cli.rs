use clap::Parser;
use std::error::Error;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version,name="Rudis" ,about="Rudis, A high-performance in memory database", long_about = None)]
pub struct Cli {
    /// 指定Rudis服务器绑定的IP地址，可以是IP地址或主机名，127.0.0.1表示只接受来自本机的连接
    #[arg(short, long)]
    pub bind: Option<String>,

    /// 指定Rudis服务器监听的TCP端口，客户端通过此端口与Rudis服务器进行通信
    #[arg(short, long)]
    pub port: Option<u16>,

    /// 启用密码保护，以增强Rudis服务器的安全性，留空不启用密码保护
    #[arg(long)]
    pub password: Option<String>,

    /// 定义Rudis实例中可用的数据库数量，Rudis默认支持16个独立的数据库
    #[arg(long)]
    pub databases: Option<usize>,

    /// 设置Rudis可以同时处理的最大客户端连接数
    #[arg(long)]
    pub maxclients: Option<usize>,

    /// 设置Rudis进行过期键检测的频率，单位为秒，M 表示每秒钟进行M次过期键的检测
    #[arg(long)]
    pub hz: Option<u64>,

    /// 数据持久化目录
    #[arg(long, value_hint = clap::ValueHint::DirPath)]
    pub dir: Option<PathBuf>,

    /// 定义RDB（Rudis Database）快照文件的名称，该文件用于数据的持久化存储
    #[arg(long)]
    pub dbfilename: Option<String>,

    /// 定义RDB快照的自动保存条件，格式为 M/N ，表示每M秒如果至少有N个键被改变，则会进行一次快照保存，eg: 60/1 10/1
    #[arg(long, value_parser = parse_rdb_save)]
    pub save: Vec<(u64, u64)>,

    /// 启用AOF（Append Only File）持久化，即所有的写命令都会被记录到一个文件中，true / false
    #[arg(long)]
    pub appendonly: Option<String>,

    /// 定义AOF持久化文件的名称
    #[arg(long)]
    pub appendfilename: Option<String>,

    #[arg(long)]
    pub appendfsync: Option<String>,

    ///  log level: OFF, ERROR, WARN, INFO, DEBUG, TRACE
    #[arg(long, default_value = "info")]
    pub log_level: log::Level,

    /// 指定配置
    #[arg(long, value_hint = clap::ValueHint::FilePath)]
    pub config: Option<PathBuf>,
}

fn parse_rdb_save(s: &str) -> Result<(u64, u64), Box<dyn Error + Send + Sync + 'static>> {
    let pos = s
        .find('/')
        .ok_or_else(|| format!("invalid M/N : no '/' found in '{s}'"))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

#[cfg(test)]
mod tests {
    use crate::db::db_config::RudisConfig;
    use crate::tools::cli;
    use clap::Parser;
    #[test]
    fn test_config_file() {
        let port = 42800;
        let config_file_path = "./release/linux/rudis-server.properties";
        let arg_string = format!(
            "rudis-server \
            -p {} \
            --config {}",
            port, config_file_path
        );
        let args: Vec<&str> = arg_string.split(' ').collect();
        let cli = cli::Cli::parse_from(args);
        let config: RudisConfig = cli.into();
        assert_eq!(config.port, port);
        assert_eq!(config.maxclients, 1000);
        assert_eq!(config.password, None);
        assert_eq!(
            config
                .save
                .unwrap()
                .iter()
                .map(|x| format!("{}/{}", x.0, x.1))
                .collect::<Vec<String>>()
                .join(" "),
            "60/1 20/2"
        );
    }
    #[test]
    fn test_cli() {
        let bind = "192.168.1.2";
        let port = 6379;
        let password = "123456";
        let databases = 1;
        let dbfilename = "123.rdb";
        let appendfilename = "321.aof";
        let appendonly = "false";
        let hz = 2;
        let appendfsync = "asd";
        let maxclients = 100;
        let dir = "/home/rudis";
        let save_1 = "60/1";
        let save_2 = "20/2";
        let save = "60/1 20/2";
        let arg_string = format!(
            "rudis-server \
            --bind {} \
            -p {} \
            --password {} \
            --databases {} \
            --dbfilename {} \
            --appendfilename {} \
            --hz {} \
            --appendfsync {} \
            --maxclients {} \
            --dir {} \
            --save {} \
            --save {} \
            --appendonly {}",
            bind,
            port,
            password,
            databases,
            dbfilename,
            appendfilename,
            hz,
            appendfsync,
            maxclients,
            dir,
            save_1,
            save_2,
            appendonly
        );

        let args: Vec<&str> = arg_string.split(' ').collect();
        let cli = cli::Cli::parse_from(args);
        let config: RudisConfig = cli.into();
        assert_eq!(config.bind, bind.to_string());
        assert_eq!(config.port, port);
        assert_eq!(config.password, Some(password.to_string()));
        assert_eq!(config.databases, databases);
        assert_eq!(config.dbfilename, Some(dbfilename.to_string()));
        assert_eq!(config.appendfilename, Some(appendfilename.to_string()));
        assert_eq!(config.appendonly.to_string(), appendonly);
        assert_eq!(config.appendfsync, Some(appendfsync.to_string()));
        assert_eq!(config.maxclients, maxclients);
        assert_eq!(config.dir, dir.to_string());
        assert!(config.save.is_some());
        assert_eq!(
            config
                .save
                .unwrap()
                .iter()
                .map(|x| format!("{}/{}", x.0, x.1))
                .collect::<Vec<String>>()
                .join(" "),
            save
        );
    }
}
