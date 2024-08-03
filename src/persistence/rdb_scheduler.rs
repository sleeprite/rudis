use std::thread;
use std::sync::{Arc, Mutex};
use super::{rdb::Rdb, rdb_count::RdbCount};
use tokio::time::Duration;

pub struct RdbScheduler {
    pub rdb: Arc<Mutex<Rdb>>,
}

impl RdbScheduler {

    pub fn new(rdb: Arc<Mutex<Rdb>>) -> RdbScheduler {
        RdbScheduler {
            rdb,
        }
    }

    pub fn execute(&mut self, save: Vec<(u64,u64)>, arc_rdb_count: Arc<Mutex<RdbCount>>) {
        for (interval, count) in save {
            let rdb_count_clone = arc_rdb_count.clone();
            let rdb = Arc::clone(&self.rdb);
            tokio::spawn(async move {
                let duration = Duration::from_secs(interval);
                loop {
                    thread::sleep(duration);
                    let mut rdb_guard = rdb.lock().unwrap();
                    let mut rdb_count = rdb_count_clone.lock().unwrap();
                    if rdb_count.modify_statistics >= count {
                        rdb_count.init();
                        rdb_guard.save();
                    }
                }
            });
        }
    }
}