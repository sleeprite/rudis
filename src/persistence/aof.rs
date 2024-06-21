use std::collections::HashMap;
use std::fs::File;
use std::io::Seek;
use std::io::{SeekFrom, Write};
use std::sync::Mutex;
use std::{fs::OpenOptions, sync::Arc};

use indicatif::{ProgressBar, ProgressStyle};

use crate::db::db::Redis;
use crate::command_strategies::init_command_strategies;
use crate::db::db_config::RedisConfig;
use crate::session::session::Session;

pub struct AppendOnlyFile {
    pub redis_config: Arc<RedisConfig>,
    pub redis: Arc<Mutex<Redis>>,
    pub file: Option<std::fs::File>,
}

impl AppendOnlyFile {
    
    pub fn new(redis_config: Arc<RedisConfig>, redis: Arc<Mutex<Redis>>) -> AppendOnlyFile {
        let mut file = None;
        if redis_config.appendonly && redis_config.appendfilename.is_some() {
            if let Some(filename) = &redis_config.appendfilename {
                file = Some(OpenOptions::new().create(true).read(true).write(true).append(true).open(filename).expect("Failed to open AOF file"));
            }
        }
        AppendOnlyFile {
            redis_config,
            redis,
            file,
        }
    }

    /*
     * 写入 aof 日志
     *
     * @param command 命令
     */
    pub fn write(&mut self, command: &str) {
        if let Some(file) = self.file.as_mut() {
            if let Err(err) = writeln!(file, "{}", command) {
                eprintln!("Failed to append to AOF file: {}", err);
            }
        }
    }

    /*
     * 解析 appendfile 文件，执行命令，回填数据
     *
     * 调用时机：项目启动
     */
    pub fn load(&mut self) {
        if self.redis_config.appendonly {
            if let Some(appendfilename) = &self.redis_config.appendfilename {
                if let Ok(mut file) = File::open(appendfilename) {
                    use std::io::{BufRead, BufReader};
                    let line_count = BufReader::new(&file).lines().count() as u64;
                    let command_strategies = init_command_strategies();
                    let session_id = "0.0.0.0:0";
                    let sessions: Arc<Mutex<HashMap<String, Session>>> = Arc::new(Mutex::new(HashMap::new()));

                    {
                        let mut sessions_ref = sessions.lock().unwrap();
                        let mut session = Session::new();
                        session.set_selected_database(0);
                        session.set_authenticated(true);
                        sessions_ref.insert(session_id.to_string(), Session::new());
                    }

                    if let Ok(_) = file.seek(SeekFrom::Start(0)) {
                        let pb = ProgressBar::new(line_count);
                        pb.set_style(ProgressStyle::default_bar().template("[{bar:39.green/cyan}] percent: {percent}% lines: {pos}/{len}").progress_chars("=>-"));
                        let reader = BufReader::new(&mut file);
                        for line in reader.lines() {
                            if let Ok(operation) = line {
                                let fragments: Vec<&str> = operation.split("\\r\\n").collect();
                                let command = fragments[2];
                                if let Some(strategy) = command_strategies.get(command) {
                                    strategy.execute(None, &fragments, &self.redis, &self.redis_config, &sessions,&session_id.to_string());
                                }
                            }
                            pb.inc(1);
                        }
                        pb.finish();
                    }
                }
            }
        }
    }
}
