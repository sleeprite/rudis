use std::{collections::HashMap, fs::{self, File}, io::Write, path::PathBuf, time::SystemTime};

use anyhow::Error;
use bincode::{config, decode_from_slice, encode_to_vec, Decode, Encode};

use crate::db::DatabaseSnapshot;

#[derive(Clone, Encode, Decode)]
pub struct RdbFile {
    databases: HashMap<usize, DatabaseSnapshot>,
    pub last_save_time: SystemTime,
    pub last_save_changes: u64,
    path: PathBuf,
}

impl RdbFile {

    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            databases: HashMap::new(),
            last_save_time: SystemTime::now(),
            last_save_changes: 0,
            path: path.into(),
        }
    }

    pub fn set_database(&mut self, id: usize, snapshot: DatabaseSnapshot) {
        self.databases.insert(id, snapshot);
    }

    pub fn get_database(&self, id: usize) -> DatabaseSnapshot {
        self.databases.get(&id).cloned().unwrap_or_else(|| DatabaseSnapshot::default())
    }

    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = File::create(&self.path)?;
        let config = config::standard();
        let serialized = encode_to_vec(&*self, config)?;
        self.last_save_time = SystemTime::now();
        file.write_all(&serialized)?;
        Ok(())
    }

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