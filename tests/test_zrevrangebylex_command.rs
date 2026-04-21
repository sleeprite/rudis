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
            &[(0.0, "a"), (0.0, "b"), (0.0, "c"), (0.0, "d"), (0.0, "e"), (0.0, "f")],
        ).unwrap();
    }

    #[test]
    fn test_zrevrangebylex_all() {
        let mut con = setup();
        seed(&mut con, "zrvl1");
        // 参数顺序：max 在前，min 在后
        let r: Vec<String> = redis::cmd("ZREVRANGEBYLEX").arg("zrvl1").arg("+").arg("-").query(&mut con).unwrap();
        assert_eq!(r, vec!["f", "e", "d", "c", "b", "a"]);
    }

    #[test]
    fn test_zrevrangebylex_inclusive() {
        let mut con = setup();
        seed(&mut con, "zrvl2");
        let r: Vec<String> = redis::cmd("ZREVRANGEBYLEX").arg("zrvl2").arg("[d").arg("[b").query(&mut con).unwrap();
        assert_eq!(r, vec!["d", "c", "b"]);
    }

    #[test]
    fn test_zrevrangebylex_exclusive() {
        let mut con = setup();
        seed(&mut con, "zrvl3");
        let r: Vec<String> = redis::cmd("ZREVRANGEBYLEX").arg("zrvl3").arg("(d").arg("(b").query(&mut con).unwrap();
        assert_eq!(r, vec!["c"]);
    }

    #[test]
    fn test_zrevrangebylex_limit() {
        let mut con = setup();
        seed(&mut con, "zrvl4");
        let r: Vec<String> = redis::cmd("ZREVRANGEBYLEX")
            .arg("zrvl4").arg("+").arg("-").arg("LIMIT").arg(1).arg(2).query(&mut con).unwrap();
        assert_eq!(r, vec!["e", "d"]);
    }

    #[test]
    fn test_zrevrangebylex_nonexistent() {
        let mut con = setup();
        let _: () = con.del("zrvl_none").unwrap();
        let r: Vec<String> = redis::cmd("ZREVRANGEBYLEX").arg("zrvl_none").arg("+").arg("-").query(&mut con).unwrap();
        assert!(r.is_empty());
    }

    #[test]
    fn test_zrevrangebylex_wrong_type() {
        let mut con = setup();
        let _: () = con.del("zrvl_str").unwrap();
        let _: () = con.set("zrvl_str", "x").unwrap();
        let r: RedisResult<Vec<String>> = redis::cmd("ZREVRANGEBYLEX").arg("zrvl_str").arg("+").arg("-").query(&mut con);
        assert!(r.is_err());
    }

    #[test]
    fn test_zrevrangebylex_invalid_range() {
        let mut con = setup();
        seed(&mut con, "zrvl5");
        // max < min（字典序反了）返回空
        let r: Vec<String> = redis::cmd("ZREVRANGEBYLEX").arg("zrvl5").arg("[a").arg("[d").query(&mut con).unwrap();
        assert!(r.is_empty());
    }
}
