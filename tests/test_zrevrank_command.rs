#[cfg(test)]
mod tests {
    use redis::{Client, Commands, Connection, RedisResult};

    fn setup() -> Connection {
        let client = Client::open("redis://127.0.0.1:6379/").unwrap();
        client.get_connection().expect("Failed to get connection")
    }

    fn zrevrank(con: &mut Connection, key: &str, member: &str) -> RedisResult<Option<i64>> {
        redis::cmd("ZREVRANK").arg(key).arg(member).query(con)
    }

    #[test]
    fn test_zrevrank_basic() {
        let mut con = setup();
        let _: () = con.del("zrrk1").unwrap();
        let _: i64 = con.zadd_multiple("zrrk1", &[(1.0, "a"), (2.0, "b"), (3.0, "c")]).unwrap();

        // 升序是 [a, b, c]，降序是 [c, b, a]
        // 降序 rank: c=0, b=1, a=2
        assert_eq!(zrevrank(&mut con, "zrrk1", "c").unwrap(), Some(0));
        assert_eq!(zrevrank(&mut con, "zrrk1", "b").unwrap(), Some(1));
        assert_eq!(zrevrank(&mut con, "zrrk1", "a").unwrap(), Some(2));
    }

    #[test]
    fn test_zrevrank_same_score() {
        let mut con = setup();
        let _: () = con.del("zrrk2").unwrap();
        // 分数相同按 member 字典序升序排列 → 降序后 [c, b, a]
        let _: i64 = con.zadd_multiple("zrrk2", &[(0.0, "a"), (0.0, "b"), (0.0, "c")]).unwrap();

        assert_eq!(zrevrank(&mut con, "zrrk2", "c").unwrap(), Some(0));
        assert_eq!(zrevrank(&mut con, "zrrk2", "a").unwrap(), Some(2));
    }

    #[test]
    fn test_zrevrank_member_not_exists() {
        let mut con = setup();
        let _: () = con.del("zrrk3").unwrap();
        let _: i64 = con.zadd_multiple("zrrk3", &[(1.0, "a")]).unwrap();

        assert_eq!(zrevrank(&mut con, "zrrk3", "not-there").unwrap(), None);
    }

    #[test]
    fn test_zrevrank_key_not_exists() {
        let mut con = setup();
        let _: () = con.del("zrrk_none").unwrap();

        assert_eq!(zrevrank(&mut con, "zrrk_none", "anything").unwrap(), None);
    }

    #[test]
    fn test_zrevrank_wrong_type() {
        let mut con = setup();
        let _: () = con.del("zrrk_str").unwrap();
        let _: () = con.set("zrrk_str", "hello").unwrap();

        let result: RedisResult<Option<i64>> = zrevrank(&mut con, "zrrk_str", "x");
        assert!(result.is_err());
    }

    #[test]
    fn test_zrevrank_wrong_arg_count() {
        let mut con = setup();
        let _: () = con.del("zrrk4").unwrap();
        let _: i64 = con.zadd_multiple("zrrk4", &[(1.0, "a")]).unwrap();

        // 缺少 member 参数
        let result: RedisResult<Option<i64>> = redis::cmd("ZREVRANK").arg("zrrk4").query(&mut con);
        assert!(result.is_err());
    }
}
