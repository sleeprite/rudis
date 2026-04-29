#[cfg(test)]
mod tests {
    use redis::{Client, Commands, Connection, RedisResult};

    fn setup() -> Connection {
        let client = Client::open("redis://127.0.0.1:6379/").unwrap();
        client.get_connection().expect("Failed to get connection")
    }

    fn seed(con: &mut Connection, key: &str) {
        let _: () = con.del(key).unwrap();
        let _: i64 = con.zadd_multiple(
            key,
            &[(1.0, "a"), (2.0, "b"), (3.0, "c"), (4.0, "d"), (5.0, "e")],
        ).unwrap();
    }

    #[test]
    fn test_zrangebyscore_inclusive() {
        let mut con = setup();
        seed(&mut con, "zrs1");
        let r: Vec<String> = redis::cmd("ZRANGEBYSCORE").arg("zrs1").arg(2).arg(4).query(&mut con).unwrap();
        assert_eq!(r, vec!["b", "c", "d"]);
    }

    #[test]
    fn test_zrangebyscore_exclusive() {
        let mut con = setup();
        seed(&mut con, "zrs2");
        // (2 4) 表示 score > 2 且 score < 4
        let r: Vec<String> = redis::cmd("ZRANGEBYSCORE").arg("zrs2").arg("(2").arg("(4").query(&mut con).unwrap();
        assert_eq!(r, vec!["c"]);
    }

    #[test]
    fn test_zrangebyscore_inf() {
        let mut con = setup();
        seed(&mut con, "zrs3");
        let r: Vec<String> = redis::cmd("ZRANGEBYSCORE").arg("zrs3").arg("-inf").arg("+inf").query(&mut con).unwrap();
        assert_eq!(r, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn test_zrangebyscore_withscores() {
        let mut con = setup();
        seed(&mut con, "zrs4");
        let r: Vec<String> = redis::cmd("ZRANGEBYSCORE")
            .arg("zrs4").arg(2).arg(3).arg("WITHSCORES").query(&mut con).unwrap();
        assert_eq!(r, vec!["b", "2", "c", "3"]);
    }

    #[test]
    fn test_zrangebyscore_limit() {
        let mut con = setup();
        seed(&mut con, "zrs5");
        let r: Vec<String> = redis::cmd("ZRANGEBYSCORE")
            .arg("zrs5").arg("-inf").arg("+inf").arg("LIMIT").arg(1).arg(2).query(&mut con).unwrap();
        assert_eq!(r, vec!["b", "c"]);
    }

    #[test]
    fn test_zrangebyscore_limit_negative_count() {
        let mut con = setup();
        seed(&mut con, "zrs6");
        // count = -1 表示全部剩余
        let r: Vec<String> = redis::cmd("ZRANGEBYSCORE")
            .arg("zrs6").arg("-inf").arg("+inf").arg("LIMIT").arg(2).arg(-1).query(&mut con).unwrap();
        assert_eq!(r, vec!["c", "d", "e"]);
    }

    #[test]
    fn test_zrangebyscore_empty_range() {
        let mut con = setup();
        seed(&mut con, "zrs7");
        // min > max
        let r: Vec<String> = redis::cmd("ZRANGEBYSCORE").arg("zrs7").arg(10).arg(20).query(&mut con).unwrap();
        assert!(r.is_empty());
    }

    #[test]
    fn test_zrangebyscore_nonexistent_key() {
        let mut con = setup();
        let _: () = con.del("zrs_none").unwrap();
        let r: Vec<String> = redis::cmd("ZRANGEBYSCORE").arg("zrs_none").arg("-inf").arg("+inf").query(&mut con).unwrap();
        assert!(r.is_empty());
    }

    #[test]
    fn test_zrangebyscore_wrong_type() {
        let mut con = setup();
        let _: () = con.del("zrs_str").unwrap();
        let _: () = con.set("zrs_str", "hello").unwrap();
        let r: RedisResult<Vec<String>> = redis::cmd("ZRANGEBYSCORE").arg("zrs_str").arg(0).arg(10).query(&mut con);
        assert!(r.is_err());
    }

    #[test]
    fn test_zrangebyscore_invalid_float() {
        let mut con = setup();
        seed(&mut con, "zrs8");
        let r: RedisResult<Vec<String>> = redis::cmd("ZRANGEBYSCORE").arg("zrs8").arg("abc").arg("+inf").query(&mut con);
        assert!(r.is_err());
    }
}
