use std::{collections::HashMap, fs::{self, File}, io::Write, path::PathBuf};

use anyhow::Error;
use bincode::{config, decode_from_slice, encode_to_vec};

use crate::db::DatabaseSnapshot;

pub struct RdbFile {
    databases: HashMap<usize, DatabaseSnapshot>,
    path: PathBuf,
}

impl RdbFile {

    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            databases: HashMap::new(),
            path: path.into(),
        }
    }

    /// 设置指定数据库的快照
    pub fn set_database(&mut self, id: usize, snapshot: DatabaseSnapshot) {
        self.databases.insert(id, snapshot);
    }

    /// 获取指定数据库的快照
    pub fn get_database(&self, id: usize) -> DatabaseSnapshot {
        self.databases.get(&id).cloned().unwrap_or_else(|| DatabaseSnapshot::default())
    }

    /// 删除指定数据库的快照
    pub fn remove_database(&mut self, id: usize) -> Option<DatabaseSnapshot> {
        self.databases.remove(&id)
    }

    /// 保存到初始化时指定的路径
    pub fn save(&self) -> Result<(), Error> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = File::create(&self.path)?;
        let config = config::standard();
        let serialized = encode_to_vec(&self.databases, config)?;
        file.write_all(&serialized)?;
        Ok(())
    }

    /// 从初始化时指定的路径加载数据
    pub fn load(&mut self) -> Result<(), Error> {
        if self.path.exists() {
            let data = fs::read(&self.path)?;
            let config = config::standard();
            let (deserialized, _) = decode_from_slice(&data, config)?;
            self.databases = deserialized;
        }
        Ok(())
    }
}