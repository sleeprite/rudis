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
    fn test_zrevrangebyscore_basic() {
        let mut con = setup();
        seed(&mut con, "zrrs1");
        // 参数顺序：max 在前，min 在后
        let r: Vec<String> = redis::cmd("ZREVRANGEBYSCORE").arg("zrrs1").arg(4).arg(2).query(&mut con).unwrap();
        assert_eq!(r, vec!["d", "c", "b"]);
    }

    #[test]
    fn test_zrevrangebyscore_exclusive() {
        let mut con = setup();
        seed(&mut con, "zrrs2");
        let r: Vec<String> = redis::cmd("ZREVRANGEBYSCORE").arg("zrrs2").arg("(4").arg("(2").query(&mut con).unwrap();
        assert_eq!(r, vec!["c"]);
    }

    #[test]
    fn test_zrevrangebyscore_inf() {
        let mut con = setup();
        seed(&mut con, "zrrs3");
        let r: Vec<String> = redis::cmd("ZREVRANGEBYSCORE").arg("zrrs3").arg("+inf").arg("-inf").query(&mut con).unwrap();
        assert_eq!(r, vec!["e", "d", "c", "b", "a"]);
    }

    #[test]
    fn test_zrevrangebyscore_withscores() {
        let mut con = setup();
        seed(&mut con, "zrrs4");
        let r: Vec<String> = redis::cmd("ZREVRANGEBYSCORE")
            .arg("zrrs4").arg(3).arg(2).arg("WITHSCORES").query(&mut con).unwrap();
        assert_eq!(r, vec!["c", "3", "b", "2"]);
    }

    #[test]
    fn test_zrevrangebyscore_limit() {
        let mut con = setup();
        seed(&mut con, "zrrs5");
        let r: Vec<String> = redis::cmd("ZREVRANGEBYSCORE")
            .arg("zrrs5").arg("+inf").arg("-inf").arg("LIMIT").arg(1).arg(2).query(&mut con).unwrap();
        assert_eq!(r, vec!["d", "c"]);
    }

    #[test]
    fn test_zrevrangebyscore_empty() {
        let mut con = setup();
        seed(&mut con, "zrrs6");
        // max < min 返回空
        let r: Vec<String> = redis::cmd("ZREVRANGEBYSCORE").arg("zrrs6").arg(0).arg(10).query(&mut con).unwrap();
        assert!(r.is_empty());
    }

    #[test]
    fn test_zrevrangebyscore_nonexistent() {
        let mut con = setup();
        let _: () = con.del("zrrs_none").unwrap();
        let r: Vec<String> = redis::cmd("ZREVRANGEBYSCORE").arg("zrrs_none").arg("+inf").arg("-inf").query(&mut con).unwrap();
        assert!(r.is_empty());
    }

    #[test]
    fn test_zrevrangebyscore_wrong_type() {
        let mut con = setup();
        let _: () = con.del("zrrs_str").unwrap();
        let _: () = con.set("zrrs_str", "x").unwrap();
        let r: RedisResult<Vec<String>> = redis::cmd("ZREVRANGEBYSCORE").arg("zrrs_str").arg(10).arg(0).query(&mut con);
        assert!(r.is_err());
    }
}
