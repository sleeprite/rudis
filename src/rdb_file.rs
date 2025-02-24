use std::{collections::HashMap, fs::File, io::Write, path::Path, time::SystemTime};

use anyhow::Error;
use bincode::{config, encode_to_vec};

use crate::db::Structure;

pub struct RdbFile {
    pub expire_records: HashMap<String, SystemTime>,
    pub records: HashMap<String, Structure>,
}

impl RdbFile {

    /**
     * 保存 dump 内容
     * 
     * @param path dump 文件路径
     */
    pub fn save(&self, path: &str) -> Result<(), Error> {

        let path = Path::new(path);
        if let Some(parent) =  Path::new(path).parent() {
            std::fs::create_dir_all(parent)?; // 创建父级目录
        }

        let mut file = File::create(path)?;
        let config = config::standard();
        let serialized = encode_to_vec(&(self.records.clone(), self.expire_records.clone()), config)?;
        file.write_all(&serialized)?;
        Ok(())
    }

    /**
     * 加载 dump 内容
     */
    pub fn load(self) -> Self {
        self
    }
}