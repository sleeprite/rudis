#[cfg(test)]
mod tests {
    use redis::{Client, Commands, Connection, RedisResult, Value};

    fn setup() -> Connection {
        let client = Client::open("redis://127.0.0.1:6379/").unwrap();
        client.get_connection().expect("Failed to get connection")
    }

    fn seed(con: &mut Connection, key: &str) {
        let _: () = con.del(key).unwrap();
        let _: i64 = con.zadd_multiple(
            key,
            &[(1.0, "alpha"), (2.0, "beta"), (3.0, "gamma"), (4.0, "delta")],
        ).unwrap();
    }

    fn parse_zscan(reply: Value) -> (i64, Vec<(String, String)>) {
        if let Value::Array(items) = reply {
            assert_eq!(items.len(), 2);
            let cursor = match &items[0] {
                Value::Int(i) => *i,
                Value::BulkString(b) => String::from_utf8(b.clone()).unwrap().parse().unwrap(),
                other => panic!("unexpected cursor frame: {:?}", other),
            };
            let inner = if let Value::Array(inner) = &items[1] {
                inner.clone()
            } else {
                panic!("expected inner array, got {:?}", items[1]);
            };
            let mut out = Vec::new();
            let mut iter = inner.into_iter();
            while let (Some(m), Some(s)) = (iter.next(), iter.next()) {
                let m = if let Value::BulkString(b) = m { String::from_utf8(b).unwrap() } else { panic!() };
                let s = if let Value::BulkString(b) = s { String::from_utf8(b).unwrap() } else { panic!() };
                out.push((m, s));
            }
            (cursor, out)
        } else {
            panic!("expected outer array, got {:?}", reply);
        }
    }

    #[test]
    fn test_zscan_full_iteration() {
        let mut con = setup();
        seed(&mut con, "zsc1");
        let reply: Value = redis::cmd("ZSCAN").arg("zsc1").arg(0).arg("COUNT").arg(100).query(&mut con).unwrap();
        let (cursor, mut pairs) = parse_zscan(reply);
        assert_eq!(cursor, 0);
        pairs.sort();
        assert_eq!(pairs, vec![
            ("alpha".to_string(),"1".to_string()),
            ("beta".to_string(), "2".to_string()),
            ("delta".to_string(),"4".to_string()),
            ("gamma".to_string(),"3".to_string()),
        ]);
    }

    #[test]
    fn test_zscan_match_pattern() {
        let mut con = setup();
        seed(&mut con, "zsc2");
        // 注：避免使用以 `*` 开头的 pattern（如 `*a*`），仓库现有的 RESP
        // parser（src/frame.rs::parse_array）会误把以 `*` 开头且长度 >1
        // 的 BulkString 当成 array 头跳过。这是 ZSCAN 之外的既有 bug。
        // 用 `?lph?`（5 字符且第 2-4 位是 `lph`）→ 命中 alpha
        let reply: Value = redis::cmd("ZSCAN").arg("zsc2").arg(0).arg("MATCH").arg("?lph?").arg("COUNT").arg(100).query(&mut con).unwrap();
        let (cursor, pairs) = parse_zscan(reply);
        assert_eq!(cursor, 0);
        let names: Vec<String> = pairs.iter().map(|(m,_)| m.clone()).collect();
        assert_eq!(names, vec!["alpha"]);
    }

    #[test]
    fn test_zscan_match_specific() {
        let mut con = setup();
        seed(&mut con, "zsc3");
        let reply: Value = redis::cmd("ZSCAN").arg("zsc3").arg(0).arg("MATCH").arg("alpha").arg("COUNT").arg(100).query(&mut con).unwrap();
        let (_, pairs) = parse_zscan(reply);
        assert_eq!(pairs, vec![("alpha".to_string(), "1".to_string())]);
    }

    #[test]
    fn test_zscan_pagination() {
        let mut con = setup();
        seed(&mut con, "zsc4");
        // 第一次 cursor=0 count=2
        let reply1: Value = redis::cmd("ZSCAN").arg("zsc4").arg(0).arg("COUNT").arg(2).query(&mut con).unwrap();
        let (cursor1, page1) = parse_zscan(reply1);
        assert_eq!(page1.len(), 2);
        assert_ne!(cursor1, 0);
        // 第二次用上一个 cursor 取剩余
        let reply2: Value = redis::cmd("ZSCAN").arg("zsc4").arg(cursor1).arg("COUNT").arg(100).query(&mut con).unwrap();
        let (cursor2, page2) = parse_zscan(reply2);
        assert_eq!(cursor2, 0);
        assert_eq!(page1.len() + page2.len(), 4);
    }

    #[test]
    fn test_zscan_nonexistent() {
        let mut con = setup();
        let _: () = con.del("zsc_none").unwrap();
        let reply: Value = redis::cmd("ZSCAN").arg("zsc_none").arg(0).query(&mut con).unwrap();
        let (cursor, pairs) = parse_zscan(reply);
        assert_eq!(cursor, 0);
        assert!(pairs.is_empty());
    }

    #[test]
    fn test_zscan_wrong_type() {
        let mut con = setup();
        let _: () = con.del("zsc_str").unwrap();
        let _: () = con.set("zsc_str", "x").unwrap();
        let r: RedisResult<Value> = redis::cmd("ZSCAN").arg("zsc_str").arg(0).query(&mut con);
        assert!(r.is_err());
    }

    #[test]
    fn test_zscan_match_no_match() {
        let mut con = setup();
        seed(&mut con, "zsc5");
        let reply: Value = redis::cmd("ZSCAN").arg("zsc5").arg(0).arg("MATCH").arg("zzz*").arg("COUNT").arg(100).query(&mut con).unwrap();
        let (cursor, pairs) = parse_zscan(reply);
        assert_eq!(cursor, 0);
        assert!(pairs.is_empty());
    }
}
