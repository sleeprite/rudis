use std::{collections::HashMap, fs::{self, File}, io::Write, path::Path, time::SystemTime};

use anyhow::Error;
use bincode::{config, decode_from_slice, encode_to_vec};

use crate::db::Structure;

pub struct RdbFile {
    pub expire_records: HashMap<String, SystemTime>,
    pub records: HashMap<String, Structure>,
}

impl RdbFile {

    /**
     * 创建 RDB 文件【空】
     */
    pub fn new() -> Self {
        RdbFile {
            expire_records: HashMap::new(),
            records: HashMap::new()
        }
    }

    /**
     * 保存 dump 内容
     * 
     * @param path dump 文件路径
     */
    pub fn save(&self, path: &str) -> Result<(), Error> {
        
        let path = Path::new(path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?; // 创建父级目录
        }
    
        let mut file = File::create(path)?;
        let config = config::standard();
        let serialized = encode_to_vec(&(&self.records, &self.expire_records), config)?;
        file.write_all(&serialized)?;
        Ok(())
    }

    /**
     * 加载 dump 内容
     * 
     * @param path dump 文件路径
     * @return Result<RdbFile, Error>
     */
    pub fn load(mut self, path: String) -> Result<Self, Error> {
        let path = Path::new(&path);
        let data = fs::read(path)?;
    
        let config = config::standard();
        let (deserialized, _len) = decode_from_slice(&data, config)?;
        let (records, expire_records) = deserialized;

        self.expire_records = expire_records;
        self.records = records;
        Ok(self)
    }
}