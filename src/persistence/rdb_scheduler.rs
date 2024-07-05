use std::{ sync::{Arc, Mutex}, time::Duration};
use std::thread;
use super::{rdb::RDB, rdb_count::RdbCount};

pub struct RdbScheduler {
    pub rdb: Arc<Mutex<RDB>>,
}

impl RdbScheduler {

    pub fn new(rdb: Arc<Mutex<RDB>>) -> RdbScheduler {
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
                    thread::spawn(move || {
                        let duration = Duration::from_secs(interval);
                        loop {
                            thread::sleep(duration);
                            let mut rdb_guard = rdb.lock().unwrap();
                            let mut rdb_count = rdb_count_clone.lock().unwrap();
                            if rdb_count.modify_statistics >= count {
                                rdb_count.clear();
                                rdb_guard.save();
                            }
                        }
                    });
                }
            }
        }
    }
}