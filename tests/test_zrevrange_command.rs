#[cfg(test)]
mod tests {
    use redis::{Client, Commands, Connection, RedisResult};

    fn setup() -> Connection {
        let client = Client::open("redis://127.0.0.1:6379/").unwrap();
        client.get_connection().expect("Failed to get connection")
    }

    fn zrevrange(con: &mut Connection, key: &str, start: i64, stop: i64) -> RedisResult<Vec<String>> {
        redis::cmd("ZREVRANGE").arg(key).arg(start).arg(stop).query(con)
    }

    fn zrevrange_with_scores(con: &mut Connection, key: &str, start: i64, stop: i64) -> RedisResult<Vec<String>> {
        redis::cmd("ZREVRANGE").arg(key).arg(start).arg(stop).arg("WITHSCORES").query(con)
    }

    #[test]
    fn test_zrevrange_basic() {
        let mut con = setup();
        let _: () = con.del("zrev1").unwrap();
        let _: i64 = con.zadd_multiple("zrev1", &[(1.0, "a"), (2.0, "b"), (3.0, "c")]).unwrap();

        // 全量倒序：[c, b, a]
        let result = zrevrange(&mut con, "zrev1", 0, -1).unwrap();
        assert_eq!(result, vec!["c", "b", "a"]);
    }

    #[test]
    fn test_zrevrange_partial() {
        let mut con = setup();
        let _: () = con.del("zrev2").unwrap();
        let _: i64 = con.zadd_multiple("zrev2", &[(1.0, "a"), (2.0, "b"), (3.0, "c"), (4.0, "d")]).unwrap();

        // 倒序前两个：[d, c]
        let result = zrevrange(&mut con, "zrev2", 0, 1).unwrap();
        assert_eq!(result, vec!["d", "c"]);
    }

    #[test]
    fn test_zrevrange_negative_index() {
        let mut con = setup();
        let _: () = con.del("zrev3").unwrap();
        let _: i64 = con.zadd_multiple("zrev3", &[(1.0, "a"), (2.0, "b"), (3.0, "c")]).unwrap();

        // -2..-1 在降序 [c, b, a] 中为 [b, a]
        let result = zrevrange(&mut con, "zrev3", -2, -1).unwrap();
        assert_eq!(result, vec!["b", "a"]);
    }

    #[test]
    fn test_zrevrange_withscores() {
        let mut con = setup();
        let _: () = con.del("zrev4").unwrap();
        let _: i64 = con.zadd_multiple("zrev4", &[(1.5, "a"), (2.5, "b")]).unwrap();

        let result = zrevrange_with_scores(&mut con, "zrev4", 0, -1).unwrap();
        assert_eq!(result, vec!["b", "2.5", "a", "1.5"]);
    }

    #[test]
    fn test_zrevrange_same_score_member_order() {
        let mut con = setup();
        let _: () = con.del("zrev5").unwrap();
        // 相同分数按成员字典序降序
        let _: i64 = con.zadd_multiple("zrev5", &[(0.0, "a"), (0.0, "b"), (0.0, "c")]).unwrap();

        let result = zrevrange(&mut con, "zrev5", 0, -1).unwrap();
        assert_eq!(result, vec!["c", "b", "a"]);
    }

    #[test]
    fn test_zrevrange_nonexistent_key() {
        let mut con = setup();
        let _: () = con.del("zrev_none").unwrap();

        let result = zrevrange(&mut con, "zrev_none", 0, -1).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_zrevrange_start_greater_than_stop() {
        let mut con = setup();
        let _: () = con.del("zrev6").unwrap();
        let _: i64 = con.zadd_multiple("zrev6", &[(1.0, "a"), (2.0, "b")]).unwrap();

        let result = zrevrange(&mut con, "zrev6", 5, 1).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_zrevrange_out_of_range() {
        let mut con = setup();
        let _: () = con.del("zrev7").unwrap();
        let _: i64 = con.zadd_multiple("zrev7", &[(1.0, "a"), (2.0, "b")]).unwrap();

        // stop 超界应自动截断
        let result = zrevrange(&mut con, "zrev7", 0, 100).unwrap();
        assert_eq!(result, vec!["b", "a"]);
    }

    #[test]
    fn test_zrevrange_wrong_type() {
        let mut con = setup();
        let _: () = con.del("zrev_str").unwrap();
        let _: () = con.set("zrev_str", "hello").unwrap();

        let result: RedisResult<Vec<String>> = zrevrange(&mut con, "zrev_str", 0, -1);
        assert!(result.is_err());
    }

    #[test]
    fn test_zrevrange_syntax_error() {
        let mut con = setup();
        let _: () = con.del("zrev8").unwrap();
        let _: i64 = con.zadd_multiple("zrev8", &[(1.0, "a")]).unwrap();

        // 不合法的第 5 个参数
        let result: RedisResult<Vec<String>> = redis::cmd("ZREVRANGE")
            .arg("zrev8").arg(0).arg(-1).arg("FOO").query(&mut con);
        assert!(result.is_err());
    }
}
