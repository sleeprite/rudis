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
            &[(1.0, "a"), (2.0, "b"), (3.0, "c")],
        ).unwrap();
    }

    #[test]
    fn test_zpopmax_no_count() {
        let mut con = setup();
        seed(&mut con, "zpx1");
        let r: Vec<String> = redis::cmd("ZPOPMAX").arg("zpx1").query(&mut con).unwrap();
        assert_eq!(r, vec!["c", "3"]);
        let remaining: i64 = redis::cmd("ZCARD").arg("zpx1").query(&mut con).unwrap();
        assert_eq!(remaining, 2);
    }

    #[test]
    fn test_zpopmax_with_count() {
        let mut con = setup();
        seed(&mut con, "zpx2");
        let r: Vec<String> = redis::cmd("ZPOPMAX").arg("zpx2").arg(2).query(&mut con).unwrap();
        // 返回按分数降序：先 c 再 b
        assert_eq!(r, vec!["c", "3", "b", "2"]);
        let rest: Vec<String> = redis::cmd("ZRANGE").arg("zpx2").arg(0).arg(-1).query(&mut con).unwrap();
        assert_eq!(rest, vec!["a"]);
    }

    #[test]
    fn test_zpopmax_count_larger_than_size() {
        let mut con = setup();
        seed(&mut con, "zpx3");
        let r: Vec<String> = redis::cmd("ZPOPMAX").arg("zpx3").arg(100).query(&mut con).unwrap();
        assert_eq!(r, vec!["c", "3", "b", "2", "a", "1"]);
        let exists: i64 = redis::cmd("EXISTS").arg("zpx3").query(&mut con).unwrap();
        assert_eq!(exists, 0);
    }

    #[test]
    fn test_zpopmax_count_zero() {
        let mut con = setup();
        seed(&mut con, "zpx4");
        let r: Vec<String> = redis::cmd("ZPOPMAX").arg("zpx4").arg(0).query(&mut con).unwrap();
        assert!(r.is_empty());
        let remaining: i64 = redis::cmd("ZCARD").arg("zpx4").query(&mut con).unwrap();
        assert_eq!(remaining, 3);
    }

    #[test]
    fn test_zpopmax_nonexistent() {
        let mut con = setup();
        let _: () = con.del("zpx_none").unwrap();
        let r: Vec<String> = redis::cmd("ZPOPMAX").arg("zpx_none").query(&mut con).unwrap();
        assert!(r.is_empty());
    }

    #[test]
    fn test_zpopmax_wrong_type() {
        let mut con = setup();
        let _: () = con.del("zpx_str").unwrap();
        let _: () = con.set("zpx_str", "x").unwrap();
        let r: RedisResult<Vec<String>> = redis::cmd("ZPOPMAX").arg("zpx_str").query(&mut con);
        assert!(r.is_err());
    }

    #[test]
    fn test_zpopmax_negative_count() {
        let mut con = setup();
        seed(&mut con, "zpx5");
        let r: RedisResult<Vec<String>> = redis::cmd("ZPOPMAX").arg("zpx5").arg(-1).query(&mut con);
        assert!(r.is_err());
    }

    #[test]
    fn test_zpopmax_same_score_member_order() {
        let mut con = setup();
        let _: () = con.del("zpx6").unwrap();
        let _: i64 = con.zadd_multiple("zpx6", &[(5.0, "c"), (5.0, "a"), (5.0, "b")]).unwrap();
        // 同分数按 member 字典序升序 → 升序末尾是 "c" → ZPOPMAX 先弹 "c"
        let r: Vec<String> = redis::cmd("ZPOPMAX").arg("zpx6").query(&mut con).unwrap();
        assert_eq!(r, vec!["c", "5"]);
    }
}
