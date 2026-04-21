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
            &[(0.0, "a"), (0.0, "b"), (0.0, "c"), (0.0, "d"), (0.0, "e")],
        ).unwrap();
    }

    #[test]
    fn test_zremrangebylex_inclusive() {
        let mut con = setup();
        seed(&mut con, "zrml1");
        let n: i64 = redis::cmd("ZREMRANGEBYLEX").arg("zrml1").arg("[b").arg("[d").query(&mut con).unwrap();
        assert_eq!(n, 3);
        let rest: Vec<String> = redis::cmd("ZRANGE").arg("zrml1").arg(0).arg(-1).query(&mut con).unwrap();
        assert_eq!(rest, vec!["a", "e"]);
    }

    #[test]
    fn test_zremrangebylex_exclusive() {
        let mut con = setup();
        seed(&mut con, "zrml2");
        let n: i64 = redis::cmd("ZREMRANGEBYLEX").arg("zrml2").arg("(b").arg("(d").query(&mut con).unwrap();
        assert_eq!(n, 1);
        let rest: Vec<String> = redis::cmd("ZRANGE").arg("zrml2").arg(0).arg(-1).query(&mut con).unwrap();
        assert_eq!(rest, vec!["a", "b", "d", "e"]);
    }

    #[test]
    fn test_zremrangebylex_all_deletes_key() {
        let mut con = setup();
        seed(&mut con, "zrml3");
        let n: i64 = redis::cmd("ZREMRANGEBYLEX").arg("zrml3").arg("-").arg("+").query(&mut con).unwrap();
        assert_eq!(n, 5);
        let exists: i64 = redis::cmd("EXISTS").arg("zrml3").query(&mut con).unwrap();
        assert_eq!(exists, 0);
    }

    #[test]
    fn test_zremrangebylex_no_match() {
        let mut con = setup();
        seed(&mut con, "zrml4");
        let n: i64 = redis::cmd("ZREMRANGEBYLEX").arg("zrml4").arg("[x").arg("[z").query(&mut con).unwrap();
        assert_eq!(n, 0);
    }

    #[test]
    fn test_zremrangebylex_nonexistent() {
        let mut con = setup();
        let _: () = con.del("zrml_none").unwrap();
        let n: i64 = redis::cmd("ZREMRANGEBYLEX").arg("zrml_none").arg("-").arg("+").query(&mut con).unwrap();
        assert_eq!(n, 0);
    }

    #[test]
    fn test_zremrangebylex_wrong_type() {
        let mut con = setup();
        let _: () = con.del("zrml_str").unwrap();
        let _: () = con.set("zrml_str", "x").unwrap();
        let r: RedisResult<i64> = redis::cmd("ZREMRANGEBYLEX").arg("zrml_str").arg("-").arg("+").query(&mut con);
        assert!(r.is_err());
    }

    #[test]
    fn test_zremrangebylex_invalid_token() {
        let mut con = setup();
        seed(&mut con, "zrml5");
        let r: RedisResult<i64> = redis::cmd("ZREMRANGEBYLEX").arg("zrml5").arg("b").arg("[d").query(&mut con);
        assert!(r.is_err());
    }
}
