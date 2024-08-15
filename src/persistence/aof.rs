use std::collections::HashMap;
use std::fs::File;
use std::io::Seek;
use std::io::{SeekFrom, Write};
use parking_lot::Mutex;
use std::{fs::OpenOptions, sync::Arc};

use indicatif::{ProgressBar, ProgressStyle};

use crate::db::db::Db;
use crate::command_strategies::init_command_strategies;
use crate::db::db_config::RudisConfig;
use crate::session::session::Session;

pub struct Aof {
    pub rudis_config: Arc<RudisConfig>,
    pub db: Arc<Mutex<Db>>,
    pub aof_file: Option<std::fs::File>,
}

impl Aof {
    
    pub fn new(rudis_config: Arc<RudisConfig>, db: Arc<Mutex<Db>>) -> Aof {
        let mut aof_file = None;
        if rudis_config.appendonly && rudis_config.appendfilename.is_some() {
            if let Some(filename) = &rudis_config.appendfilename {
                let base_path = &rudis_config.dir;
                let file_path = format!("{}{}", base_path, filename);
                aof_file = Some(OpenOptions::new().create(true).read(true).append(true).open(file_path).expect("Failed to open AOF file"));
            }
        }
        Aof {
            rudis_config,
            db,
            aof_file,
        }
    }

    /*
     * 写入 aof 日志【增量】
     *
     * @param command 命令
     */
    pub fn save(&mut self, command: &str) {
        if let Some(file) = self.aof_file.as_mut() {
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
        if self.rudis_config.appendonly {
            if let Some(filename) = &self.rudis_config.appendfilename {
                let base_path = &self.rudis_config.dir;
                let file_path = format!("{}{}", base_path, filename);
                if let Ok(mut file) = File::open(file_path) {
                    use std::io::{BufRead, BufReader};
                    let line_count = BufReader::new(&file).lines().count() as u64;
                    let command_strategies = init_command_strategies();
                    let sessions: Arc<Mutex<HashMap<String, Session>>> = Arc::new(Mutex::new(HashMap::new()));
                    let session_id = "0.0.0.0:0";

                    {
                        let mut sessions_ref = sessions.lock();
                        let mut session = Session::new();
                        session.set_selected_database(0);
                        session.set_authenticated(true);
                        sessions_ref.insert(session_id.to_string(), session);
                    }

                    if file.seek(SeekFrom::Start(0)).is_ok() {
                        let pb = ProgressBar::new(line_count);
                        pb.set_style(ProgressStyle::default_bar().template("[{bar:39.green/cyan}] percent: {percent}% lines: {pos}/{len}").progress_chars("=>-"));
                        let reader = BufReader::new(&mut file);
                        for line in reader.lines() {
                            if let Ok(operation) = line {
                                let fragments: Vec<&str> = operation.split("\\r\\n").collect();
                                let command = fragments[2];
                                if let Some(strategy) = command_strategies.get(command.to_uppercase().as_str()) {
                                    strategy.execute(None, &fragments, &self.db, &self.rudis_config, &sessions,session_id);
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
