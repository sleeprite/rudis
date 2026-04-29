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

    fn zremrangebyrank(con: &mut Connection, key: &str, start: i64, stop: i64) -> RedisResult<i64> {
        redis::cmd("ZREMRANGEBYRANK").arg(key).arg(start).arg(stop).query(con)
    }

    #[test]
    fn test_zremrangebyrank_basic() {
        let mut con = setup();
        seed(&mut con, "zrr1");
        // 删除前 2 个（rank 0, 1）
        let n = zremrangebyrank(&mut con, "zrr1", 0, 1).unwrap();
        assert_eq!(n, 2);
        let rest: Vec<String> = redis::cmd("ZRANGE").arg("zrr1").arg(0).arg(-1).query(&mut con).unwrap();
        assert_eq!(rest, vec!["c", "d", "e"]);
    }

    #[test]
    fn test_zremrangebyrank_negative_index() {
        let mut con = setup();
        seed(&mut con, "zrr2");
        // 删除最后两个（rank -2..-1）
        let n = zremrangebyrank(&mut con, "zrr2", -2, -1).unwrap();
        assert_eq!(n, 2);
        let rest: Vec<String> = redis::cmd("ZRANGE").arg("zrr2").arg(0).arg(-1).query(&mut con).unwrap();
        assert_eq!(rest, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_zremrangebyrank_all_deletes_key() {
        let mut con = setup();
        seed(&mut con, "zrr3");
        let n = zremrangebyrank(&mut con, "zrr3", 0, -1).unwrap();
        assert_eq!(n, 5);
        // 集合被清空后，键应被删除
        let exists: i64 = redis::cmd("EXISTS").arg("zrr3").query(&mut con).unwrap();
        assert_eq!(exists, 0);
    }

    #[test]
    fn test_zremrangebyrank_empty_range() {
        let mut con = setup();
        seed(&mut con, "zrr4");
        let n = zremrangebyrank(&mut con, "zrr4", 10, 20).unwrap();
        assert_eq!(n, 0);
    }

    #[test]
    fn test_zremrangebyrank_nonexistent() {
        let mut con = setup();
        let _: () = con.del("zrr_none").unwrap();
        let n = zremrangebyrank(&mut con, "zrr_none", 0, -1).unwrap();
        assert_eq!(n, 0);
    }

    #[test]
    fn test_zremrangebyrank_wrong_type() {
        let mut con = setup();
        let _: () = con.del("zrr_str").unwrap();
        let _: () = con.set("zrr_str", "x").unwrap();
        let r: RedisResult<i64> = zremrangebyrank(&mut con, "zrr_str", 0, -1);
        assert!(r.is_err());
    }
}
