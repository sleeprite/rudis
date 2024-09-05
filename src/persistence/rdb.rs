use std::fs::File;
use std::io::Seek;
use std::io::Write;
use std::{fs::OpenOptions, io::SeekFrom, sync::Arc};

use parking_lot::Mutex;

use ahash::AHashMap;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;

use crate::db::{
    db::{Db, TimedData, TimedDataValue},
    db_config::RudisConfig,
};

pub struct Rdb {
    pub rudis_config: Arc<RudisConfig>,
    pub db: Arc<Mutex<Db>>,
    pub rdb_file: Option<std::fs::File>,
}

impl Rdb {
    pub fn new(rudis_config: Arc<RudisConfig>, db: Arc<Mutex<Db>>) -> Rdb {
        let mut rdb_file = None;
        if let Some(filename) = &rudis_config.dbfilename {
            let base_path = &rudis_config.dir;
            let file_path = format!("{}{}", base_path, filename);
            rdb_file = Some(
                OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(file_path)
                    .expect("Failed to open RDB file"),
            );
        }

        Rdb {
            rudis_config,
            db,
            rdb_file,
        }
    }

    pub fn save(&mut self) {
        if let Some(file) = self.rdb_file.as_mut() {
            if let Err(err) = file.set_len(0) {
                eprintln!("Failed to truncate RDB file: {}", err);
                return;
            }
            if let Err(err) = file.seek(SeekFrom::Start(0)) {
                eprintln!("Failed to seek to start of RDB file: {}", err);
                return;
            }
            let db_ref = self.db.lock();
            let databases: &Vec<AHashMap<String, TimedData>> = db_ref.get_databases();
            for (db_index, database) in databases.iter().enumerate() {
                for (key, redis_data) in database.iter() {
                    let expire_at = redis_data.get_expire_at();
                    let protocol_line = match redis_data.get_value() {
                        TimedDataValue::List(list) => {
                            format!(
                                "{}\\r\\n{}\\r\\n{:?}\\r\\nList\\r\\n{}",
                                db_index, key, list, expire_at
                            )
                        }
                        TimedDataValue::Hash(hash) => {
                            format!(
                                "{}\\r\\n{}\\r\\n{:?}\\r\\nHash\\r\\n{}",
                                db_index, key, hash, expire_at
                            )
                        }
                        TimedDataValue::Zset(zset) => {
                            format!(
                                "{}\\r\\n{}\\r\\n{:?}\\r\\nZset\\r\\n{}",
                                db_index, key, zset, expire_at
                            )
                        }
                        TimedDataValue::String(value) => {
                            format!(
                                "{}\\r\\n{}\\r\\n{}\\r\\nString\\r\\n{}",
                                db_index, key, value, expire_at
                            )
                        }
                        TimedDataValue::Set(set) => {
                            format!(
                                "{}\\r\\n{}\\r\\n{:?}\\r\\nSet\\r\\n{}",
                                db_index, key, set, expire_at
                            )
                        }
                    };
                    if let Err(err) = writeln!(file, "{}", protocol_line) {
                        eprintln!("Failed to append to RDB file: {}", err);
                    }
                }
            }
        } else {
            eprintln!("RDB file is not available.");
        }
    }

    pub fn load(&mut self) {
        let mut db_ref = self.db.lock();
        if let Some(filename) = &self.rudis_config.dbfilename {
            let base_path = &self.rudis_config.dir;
            let file_path = format!("{}{}", base_path, filename);
            if let Ok(mut file) = File::open(file_path) {
                use std::io::{BufRead, BufReader};
                let line_count = BufReader::new(&file).lines().count() as u64;
                if file.seek(SeekFrom::Start(0)).is_ok() {
                    let pb = ProgressBar::new(line_count);
                    pb.set_style(
                        ProgressStyle::default_bar()
                            .template(
                                "[{bar:39.green/cyan}] percent: {percent}% lines: {pos}/{len}",
                            )
                            .progress_chars("=>-"),
                    );
                    let reader = BufReader::new(&mut file);
                    for line in reader.lines() {
                        if let Ok(operation) = line {
                            let parts: Vec<&str> = operation.split("\\r\\n").collect();
                            let data_type = parts[3].to_string();
                            let db_index = parts[0].parse::<usize>().unwrap();
                            let expire_at = parts[4].parse().unwrap();
                            let value_str = parts[2];
                            let value: Option<TimedDataValue> = match data_type.as_str() {
                                "List" => Some(TimedDataValue::List(
                                    serde_json::from_str(value_str).unwrap(),
                                )),
                                "Hash" => Some(TimedDataValue::Hash(
                                    serde_json::from_str(value_str).unwrap(),
                                )),
                                "Zset" => Some(TimedDataValue::Zset(
                                    serde_json::from_str(value_str).unwrap(),
                                )),
                                "String" => Some(TimedDataValue::String(value_str.to_string())),
                                "Set" => Some(TimedDataValue::Set(
                                    serde_json::from_str(value_str).unwrap(),
                                )),
                                _ => None,
                            };

                            if let Some(redis_value) = value {
                                // Handle the TimedDataValue here
                                let key = parts[1].to_string();
                                db_ref.set(db_index, key, redis_value, expire_at);
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
