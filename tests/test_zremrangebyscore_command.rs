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
    fn test_zremrangebyscore_inclusive() {
        let mut con = setup();
        seed(&mut con, "zrs1");
        let n: i64 = redis::cmd("ZREMRANGEBYSCORE").arg("zrs1").arg(2).arg(4).query(&mut con).unwrap();
        assert_eq!(n, 3);
        let rest: Vec<String> = redis::cmd("ZRANGE").arg("zrs1").arg(0).arg(-1).query(&mut con).unwrap();
        assert_eq!(rest, vec!["a", "e"]);
    }

    #[test]
    fn test_zremrangebyscore_exclusive() {
        let mut con = setup();
        seed(&mut con, "zrs2");
        let n: i64 = redis::cmd("ZREMRANGEBYSCORE").arg("zrs2").arg("(1").arg("(5").query(&mut con).unwrap();
        assert_eq!(n, 3);
        let rest: Vec<String> = redis::cmd("ZRANGE").arg("zrs2").arg(0).arg(-1).query(&mut con).unwrap();
        assert_eq!(rest, vec!["a", "e"]);
    }

    #[test]
    fn test_zremrangebyscore_inf_deletes_key() {
        let mut con = setup();
        seed(&mut con, "zrs3");
        let n: i64 = redis::cmd("ZREMRANGEBYSCORE").arg("zrs3").arg("-inf").arg("+inf").query(&mut con).unwrap();
        assert_eq!(n, 5);
        let exists: i64 = redis::cmd("EXISTS").arg("zrs3").query(&mut con).unwrap();
        assert_eq!(exists, 0);
    }

    #[test]
    fn test_zremrangebyscore_no_match() {
        let mut con = setup();
        seed(&mut con, "zrs4");
        let n: i64 = redis::cmd("ZREMRANGEBYSCORE").arg("zrs4").arg(100).arg(200).query(&mut con).unwrap();
        assert_eq!(n, 0);
    }

    #[test]
    fn test_zremrangebyscore_nonexistent() {
        let mut con = setup();
        let _: () = con.del("zrs_none").unwrap();
        let n: i64 = redis::cmd("ZREMRANGEBYSCORE").arg("zrs_none").arg("-inf").arg("+inf").query(&mut con).unwrap();
        assert_eq!(n, 0);
    }

    #[test]
    fn test_zremrangebyscore_wrong_type() {
        let mut con = setup();
        let _: () = con.del("zrs_str").unwrap();
        let _: () = con.set("zrs_str", "x").unwrap();
        let r: RedisResult<i64> = redis::cmd("ZREMRANGEBYSCORE").arg("zrs_str").arg(0).arg(10).query(&mut con);
        assert!(r.is_err());
    }

    #[test]
    fn test_zremrangebyscore_invalid_float() {
        let mut con = setup();
        seed(&mut con, "zrs5");
        let r: RedisResult<i64> = redis::cmd("ZREMRANGEBYSCORE").arg("zrs5").arg("abc").arg(10).query(&mut con);
        assert!(r.is_err());
    }
}
