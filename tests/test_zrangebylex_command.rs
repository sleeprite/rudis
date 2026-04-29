#[cfg(test)]
mod tests {
    use redis::{Client, Commands, Connection, RedisResult};

    fn setup() -> Connection {
        let client = Client::open("redis://127.0.0.1:6379/").unwrap();
        client.get_connection().expect("Failed to get connection")
    }

    fn seed(con: &mut Connection, key: &str) {
        let _: () = con.del(key).unwrap();
        // 所有成员同分数（Redis 对 ZRANGEBYLEX 的前提）
        let _: i64 = con.zadd_multiple(
            key,
            &[(0.0, "a"), (0.0, "b"), (0.0, "c"), (0.0, "d"), (0.0, "e"), (0.0, "f")],
        ).unwrap();
    }

    #[test]
    fn test_zrangebylex_all() {
        let mut con = setup();
        seed(&mut con, "zrbl1");
        let r: Vec<String> = redis::cmd("ZRANGEBYLEX").arg("zrbl1").arg("-").arg("+").query(&mut con).unwrap();
        assert_eq!(r, vec!["a", "b", "c", "d", "e", "f"]);
    }

    #[test]
    fn test_zrangebylex_inclusive() {
        let mut con = setup();
        seed(&mut con, "zrbl2");
        let r: Vec<String> = redis::cmd("ZRANGEBYLEX").arg("zrbl2").arg("[b").arg("[d").query(&mut con).unwrap();
        assert_eq!(r, vec!["b", "c", "d"]);
    }

    #[test]
    fn test_zrangebylex_exclusive() {
        let mut con = setup();
        seed(&mut con, "zrbl3");
        let r: Vec<String> = redis::cmd("ZRANGEBYLEX").arg("zrbl3").arg("(b").arg("(d").query(&mut con).unwrap();
        assert_eq!(r, vec!["c"]);
    }

    #[test]
    fn test_zrangebylex_mixed() {
        let mut con = setup();
        seed(&mut con, "zrbl4");
        let r: Vec<String> = redis::cmd("ZRANGEBYLEX").arg("zrbl4").arg("[b").arg("(e").query(&mut con).unwrap();
        assert_eq!(r, vec!["b", "c", "d"]);
    }

    #[test]
    fn test_zrangebylex_limit() {
        let mut con = setup();
        seed(&mut con, "zrbl5");
        let r: Vec<String> = redis::cmd("ZRANGEBYLEX")
            .arg("zrbl5").arg("-").arg("+").arg("LIMIT").arg(1).arg(3).query(&mut con).unwrap();
        assert_eq!(r, vec!["b", "c", "d"]);
    }

    #[test]
    fn test_zrangebylex_withscores_rejected() {
        let mut con = setup();
        seed(&mut con, "zrbl6");
        // ZRANGEBYLEX 不支持 WITHSCORES
        let r: RedisResult<Vec<String>> = redis::cmd("ZRANGEBYLEX")
            .arg("zrbl6").arg("-").arg("+").arg("WITHSCORES").query(&mut con);
        assert!(r.is_err());
    }

    #[test]
    fn test_zrangebylex_invalid_range_token() {
        let mut con = setup();
        seed(&mut con, "zrbl7");
        let r: RedisResult<Vec<String>> = redis::cmd("ZRANGEBYLEX").arg("zrbl7").arg("b").arg("[d").query(&mut con);
        assert!(r.is_err());
    }

    #[test]
    fn test_zrangebylex_nonexistent() {
        let mut con = setup();
        let _: () = con.del("zrbl_none").unwrap();
        let r: Vec<String> = redis::cmd("ZRANGEBYLEX").arg("zrbl_none").arg("-").arg("+").query(&mut con).unwrap();
        assert!(r.is_empty());
    }

    #[test]
    fn test_zrangebylex_wrong_type() {
        let mut con = setup();
        let _: () = con.del("zrbl_str").unwrap();
        let _: () = con.set("zrbl_str", "x").unwrap();
        let r: RedisResult<Vec<String>> = redis::cmd("ZRANGEBYLEX").arg("zrbl_str").arg("-").arg("+").query(&mut con);
        assert!(r.is_err());
    }

    #[test]
    fn test_zrangebylex_invalid_range() {
        let mut con = setup();
        seed(&mut con, "zrbl8");
        // min > max 返回空
        let r: Vec<String> = redis::cmd("ZRANGEBYLEX").arg("zrbl8").arg("[d").arg("[a").query(&mut con).unwrap();
        assert!(r.is_empty());
    }
}
