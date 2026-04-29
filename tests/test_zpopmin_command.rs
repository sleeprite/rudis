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
    fn test_zpopmin_no_count() {
        let mut con = setup();
        seed(&mut con, "zpm1");
        let r: Vec<String> = redis::cmd("ZPOPMIN").arg("zpm1").query(&mut con).unwrap();
        assert_eq!(r, vec!["a", "1"]);
        let remaining: i64 = redis::cmd("ZCARD").arg("zpm1").query(&mut con).unwrap();
        assert_eq!(remaining, 2);
    }

    #[test]
    fn test_zpopmin_with_count() {
        let mut con = setup();
        seed(&mut con, "zpm2");
        let r: Vec<String> = redis::cmd("ZPOPMIN").arg("zpm2").arg(2).query(&mut con).unwrap();
        assert_eq!(r, vec!["a", "1", "b", "2"]);
        let rest: Vec<String> = redis::cmd("ZRANGE").arg("zpm2").arg(0).arg(-1).query(&mut con).unwrap();
        assert_eq!(rest, vec!["c"]);
    }

    #[test]
    fn test_zpopmin_count_larger_than_size() {
        let mut con = setup();
        seed(&mut con, "zpm3");
        let r: Vec<String> = redis::cmd("ZPOPMIN").arg("zpm3").arg(100).query(&mut con).unwrap();
        assert_eq!(r, vec!["a", "1", "b", "2", "c", "3"]);
        // 全部弹出后键应被删除
        let exists: i64 = redis::cmd("EXISTS").arg("zpm3").query(&mut con).unwrap();
        assert_eq!(exists, 0);
    }

    #[test]
    fn test_zpopmin_count_zero() {
        let mut con = setup();
        seed(&mut con, "zpm4");
        let r: Vec<String> = redis::cmd("ZPOPMIN").arg("zpm4").arg(0).query(&mut con).unwrap();
        assert!(r.is_empty());
        // 原集合不动
        let remaining: i64 = redis::cmd("ZCARD").arg("zpm4").query(&mut con).unwrap();
        assert_eq!(remaining, 3);
    }

    #[test]
    fn test_zpopmin_nonexistent() {
        let mut con = setup();
        let _: () = con.del("zpm_none").unwrap();
        let r: Vec<String> = redis::cmd("ZPOPMIN").arg("zpm_none").query(&mut con).unwrap();
        assert!(r.is_empty());
    }

    #[test]
    fn test_zpopmin_wrong_type() {
        let mut con = setup();
        let _: () = con.del("zpm_str").unwrap();
        let _: () = con.set("zpm_str", "x").unwrap();
        let r: RedisResult<Vec<String>> = redis::cmd("ZPOPMIN").arg("zpm_str").query(&mut con);
        assert!(r.is_err());
    }

    #[test]
    fn test_zpopmin_negative_count() {
        let mut con = setup();
        seed(&mut con, "zpm5");
        let r: RedisResult<Vec<String>> = redis::cmd("ZPOPMIN").arg("zpm5").arg(-1).query(&mut con);
        assert!(r.is_err());
    }

    #[test]
    fn test_zpopmin_same_score_member_order() {
        let mut con = setup();
        let _: () = con.del("zpm6").unwrap();
        let _: i64 = con.zadd_multiple("zpm6", &[(5.0, "c"), (5.0, "a"), (5.0, "b")]).unwrap();
        // 同分数按 member 字典序升序，ZPOPMIN 先弹 "a"
        let r: Vec<String> = redis::cmd("ZPOPMIN").arg("zpm6").query(&mut con).unwrap();
        assert_eq!(r, vec!["a", "5"]);
    }
}
