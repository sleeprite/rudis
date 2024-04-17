use std::time::{SystemTime, UNIX_EPOCH};

/*
 * 获取当前时间戳
 */
pub fn current_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}