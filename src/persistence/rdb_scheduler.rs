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

    pub fn execute(&mut self, input: &str, arc_rdb_count: Arc<Mutex<RdbCount>>) {
        let parts: Vec<&str> = input.split_whitespace().collect();
        for i in (0..parts.len()).step_by(2) {
            let rdb_count_clone = arc_rdb_count.clone();
            if let (Some(interval), Some(count)) = (parts.get(i), parts.get(i + 1)) {
                if let (Ok(interval), Ok(count)) = (interval.parse::<u64>(), count.parse::<u64>()) {
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
    }
}