use std::{collections::HashMap, fs::File, io::Write, time::SystemTime};

use anyhow::Error;
use bincode::{config, encode_to_vec};

use crate::db::Structure;

pub struct RdbFile {
    pub expire_records: HashMap<String, SystemTime>,
    pub records: HashMap<String, Structure>,
}

impl RdbFile {

    pub fn save(&self, path: &str) -> Result<(), Error> {
        let mut file = File::create(path)?;
        let config = config::standard();
        let serialized = encode_to_vec(&(self.records.clone(), self.expire_records.clone()), config)?;
        file.write_all(&serialized)?;
        Ok(())
    }
}