use std::{collections::HashMap, fs::{self, File}, io::Write, path::PathBuf, time::SystemTime};

use anyhow::Error;
use bincode::{config, decode_from_slice, encode_to_vec, Decode, Encode};

use crate::db::DatabaseSnapshot;

/// Rudis 数据库快照文件 (RDB) 的表示
///
/// 包含多个数据库的快照、持久化元数据和文件路径信息。
/// 使用二进制格式 (bincode) 进行序列化和反序列化。
#[derive(Clone, Encode, Decode)]
pub struct RdbFile {
    pub databases: HashMap<usize, DatabaseSnapshot>,
    pub last_save_time: SystemTime,
    pub last_save_changes: u64,
    path: PathBuf,
}

impl RdbFile {

    /// 创建新的空 RDB 文件对象
    ///
    /// # 参数
    /// - `path`: RDB 文件存储路径
    ///
    /// # 返回
    /// 初始化后的 RdbFile 实例（含当前时间戳和空数据库）
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            databases: HashMap::new(),
            last_save_time: SystemTime::now(),
            last_save_changes: 0,
            path: path.into(),
        }
    }

    /// 从数据库快照列表创建 RDB 文件对象
    ///
    /// # 参数
    /// - `snapshots`: 数据库快照向量（按索引顺序）
    ///
    /// # 返回
    /// 包含所有快照的 RdbFile 实例（使用临时路径）
    pub fn from_snapshots(snapshots: Vec<DatabaseSnapshot>) -> Self {
        let mut db_map = HashMap::new();
        for (idx, snapshot) in snapshots.into_iter().enumerate() {
            db_map.insert(idx, snapshot);
        }
        
        Self {
            databases: db_map,
            path: PathBuf::from("virtual-dump.rdb"),
            last_save_time: SystemTime::now(),
            last_save_changes: 0,
        }
    }

    /// 从字节数据反序列化 RDB 文件
    ///
    /// # 参数
    /// - `bytes`: RDB 文件原始字节数据
    ///
    /// # 返回
    /// - `Ok(RdbFile)`: 解析成功的 RDB 文件对象
    /// - `Err(Error)`: 反序列化失败时返回错误
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let config = config::standard();
        let (rdb_file, _) = decode_from_slice(bytes, config)?;
        Ok(rdb_file)
    }

    /// 将当前对象序列化为字节向量
    ///
    /// # 返回
    /// - `Ok(Vec<u8>)`: 序列化后的字节数据
    /// - `Err(Error)`: 序列化失败时返回错误
    pub fn serialize(&self) -> Result<Vec<u8>, Error> {
        let config = config::standard();
        Ok(encode_to_vec(self, config)?)
    }

    /// 获取指定数据库的快照
    ///
    /// # 参数
    /// - `id`: 数据库索引
    ///
    /// # 返回
    /// - 存在: 返回数据库快照的克隆
    /// - 不存在: 返回默认空快照
    pub fn get_database(&self, id: usize) -> DatabaseSnapshot {
        self.databases.get(&id).cloned().unwrap_or_else(|| DatabaseSnapshot::default())
    }

    /// 设置/更新数据库快照
    ///
    /// # 参数
    /// - `id`: 目标数据库索引
    /// - `snapshot`: 新的数据库快照
    pub fn set_database(&mut self, id: usize, snapshot: DatabaseSnapshot) {
        self.databases.insert(id, snapshot);
    }

    /// 将当前状态保存到文件
    ///
    /// 执行步骤:
    /// 1. 创建父目录（如果需要）
    /// 2. 序列化数据
    /// 3. 写入文件
    /// 4. 更新保存时间和变更计数
    ///
    /// # 返回
    /// - `Ok(())`: 保存成功
    /// - `Err(Error)`: 文件操作或序列化失败
    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = File::create(&self.path)?;
        let config = config::standard();
        let serialized = encode_to_vec(&*self, config)?;
        file.write_all(&serialized)?;
        Ok(())
    }

    /// 从磁盘文件加载数据
    ///
    /// 执行步骤:
    /// 1. 检查文件是否存在
    /// 2. 读取字节数据
    /// 3. 反序列化并覆盖当前状态
    ///
    /// # 返回
    /// - `Ok(())`: 加载成功（文件不存在视为空操作）
    /// - `Err(Error)`: 文件读取或反序列化失败
    pub fn load(&mut self) -> Result<(), Error> {
        if self.path.exists() {
            let data = fs::read(&self.path)?;
            let config = config::standard();
            let (deserialized, _) = decode_from_slice::<RdbFile, _>(&data, config)?;
            self.last_save_changes = deserialized.last_save_changes;
            self.last_save_time = deserialized.last_save_time;
            self.databases = deserialized.databases;
        }
        Ok(())
    }
}