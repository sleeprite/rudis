#[cfg(test)]
mod tests {
    use redis::{Client, Commands, Connection, RedisResult};

    fn setup() -> Connection {
        let client = Client::open("redis://127.0.0.1:6379/").unwrap();
        client.get_connection().expect("Failed to get connection")
    }

    fn seed_two(con: &mut Connection, ka: &str, kb: &str) {
        let _: () = con.del(ka).unwrap();
        let _: () = con.del(kb).unwrap();
        let _: i64 = con.zadd_multiple(ka, &[(1.0, "a"), (2.0, "b"), (3.0, "c")]).unwrap();
        let _: i64 = con.zadd_multiple(kb, &[(10.0, "b"), (20.0, "c"), (30.0, "d")]).unwrap();
    }

    fn pairs_with_scores(con: &mut Connection, key: &str) -> Vec<(String, String)> {
        let raw: Vec<String> = redis::cmd("ZRANGE").arg(key).arg(0).arg(-1).arg("WITHSCORES").query(con).unwrap();
        let mut iter = raw.into_iter();
        let mut out = Vec::new();
        while let (Some(m), Some(s)) = (iter.next(), iter.next()) {
            out.push((m, s));
        }
        out
    }

    #[test]
    fn test_zunionstore_default_sum() {
        let mut con = setup();
        seed_two(&mut con, "zus1_a", "zus1_b");
        let _: () = con.del("zus1_dest").unwrap();
        let n: i64 = redis::cmd("ZUNIONSTORE").arg("zus1_dest").arg(2).arg("zus1_a").arg("zus1_b").query(&mut con).unwrap();
        assert_eq!(n, 4);
        let mut got = pairs_with_scores(&mut con, "zus1_dest");
        got.sort();
        assert_eq!(got, vec![
            ("a".to_string(), "1".to_string()),
            ("b".to_string(), "12".to_string()),
            ("c".to_string(), "23".to_string()),
            ("d".to_string(), "30".to_string()),
        ]);
    }

    #[test]
    fn test_zunionstore_aggregate_min() {
        let mut con = setup();
        seed_two(&mut con, "zus2_a", "zus2_b");
        let _: () = con.del("zus2_dest").unwrap();
        let n: i64 = redis::cmd("ZUNIONSTORE").arg("zus2_dest").arg(2).arg("zus2_a").arg("zus2_b")
            .arg("AGGREGATE").arg("MIN").query(&mut con).unwrap();
        assert_eq!(n, 4);
        let mut got = pairs_with_scores(&mut con, "zus2_dest");
        got.sort();
        assert_eq!(got, vec![
            ("a".to_string(), "1".to_string()),
            ("b".to_string(), "2".to_string()),
            ("c".to_string(), "3".to_string()),
            ("d".to_string(), "30".to_string()),
        ]);
    }

    #[test]
    fn test_zunionstore_aggregate_max() {
        let mut con = setup();
        seed_two(&mut con, "zus3_a", "zus3_b");
        let _: () = con.del("zus3_dest").unwrap();
        let _: i64 = redis::cmd("ZUNIONSTORE").arg("zus3_dest").arg(2).arg("zus3_a").arg("zus3_b")
            .arg("AGGREGATE").arg("MAX").query(&mut con).unwrap();
        let mut got = pairs_with_scores(&mut con, "zus3_dest");
        got.sort();
        assert_eq!(got, vec![
            ("a".to_string(), "1".to_string()),
            ("b".to_string(), "10".to_string()),
            ("c".to_string(), "20".to_string()),
            ("d".to_string(), "30".to_string()),
        ]);
    }

    #[test]
    fn test_zunionstore_weights() {
        let mut con = setup();
        seed_two(&mut con, "zus4_a", "zus4_b");
        let _: () = con.del("zus4_dest").unwrap();
        // weight: a*2, b*0.5
        let _: i64 = redis::cmd("ZUNIONSTORE").arg("zus4_dest").arg(2).arg("zus4_a").arg("zus4_b")
            .arg("WEIGHTS").arg(2).arg("0.5").query(&mut con).unwrap();
        let mut got = pairs_with_scores(&mut con, "zus4_dest");
        got.sort();
        // a: 1*2 = 2; b: 2*2 + 10*0.5 = 9; c: 3*2 + 20*0.5 = 16; d: 30*0.5 = 15
        assert_eq!(got, vec![
            ("a".to_string(), "2".to_string()),
            ("b".to_string(), "9".to_string()),
            ("c".to_string(), "16".to_string()),
            ("d".to_string(), "15".to_string()),
        ]);
    }

    #[test]
    fn test_zunionstore_with_set_input() {
        let mut con = setup();
        let _: () = con.del("zus5_zs").unwrap();
        let _: () = con.del("zus5_set").unwrap();
        let _: () = con.del("zus5_dest").unwrap();
        let _: i64 = con.zadd_multiple("zus5_zs", &[(5.0, "a"), (5.0, "b")]).unwrap();
        let _: i64 = con.sadd("zus5_set", &["b", "c"][..]).unwrap();
        let n: i64 = redis::cmd("ZUNIONSTORE").arg("zus5_dest").arg(2).arg("zus5_zs").arg("zus5_set").query(&mut con).unwrap();
        assert_eq!(n, 3);
        let mut got = pairs_with_scores(&mut con, "zus5_dest");
        got.sort();
        assert_eq!(got, vec![
            ("a".to_string(), "5".to_string()),
            ("b".to_string(), "6".to_string()),  // 5 + 1
            ("c".to_string(), "1".to_string()),
        ]);
    }

    #[test]
    fn test_zunionstore_overwrites_dest() {
        let mut con = setup();
        seed_two(&mut con, "zus6_a", "zus6_b");
        let _: () = con.del("zus6_dest").unwrap();
        let _: i64 = con.zadd_multiple("zus6_dest", &[(99.0, "x"), (99.0, "y")]).unwrap();
        let _: i64 = redis::cmd("ZUNIONSTORE").arg("zus6_dest").arg(1).arg("zus6_a").query(&mut con).unwrap();
        let card: i64 = redis::cmd("ZCARD").arg("zus6_dest").query(&mut con).unwrap();
        assert_eq!(card, 3);  // 仅 zus6_a 内容
        let exists_x: bool = redis::cmd("ZSCORE").arg("zus6_dest").arg("x").query::<Option<String>>(&mut con).unwrap().is_some();
        assert!(!exists_x);
    }

    #[test]
    fn test_zunionstore_empty_result_no_dest() {
        let mut con = setup();
        let _: () = con.del("zus7_e1").unwrap();
        let _: () = con.del("zus7_e2").unwrap();
        let _: () = con.del("zus7_dest").unwrap();
        let _: i64 = con.zadd_multiple("zus7_dest", &[(99.0, "leftover")]).unwrap();
        let n: i64 = redis::cmd("ZUNIONSTORE").arg("zus7_dest").arg(2).arg("zus7_e1").arg("zus7_e2").query(&mut con).unwrap();
        assert_eq!(n, 0);
        let exists: i64 = redis::cmd("EXISTS").arg("zus7_dest").query(&mut con).unwrap();
        assert_eq!(exists, 0);
    }

    #[test]
    fn test_zunionstore_wrong_type() {
        let mut con = setup();
        let _: () = con.del("zus8_a").unwrap();
        let _: i64 = con.zadd_multiple("zus8_a", &[(1.0, "a")]).unwrap();
        let _: () = con.del("zus8_str").unwrap();
        let _: () = con.set("zus8_str", "x").unwrap();
        let _: () = con.del("zus8_dest").unwrap();
        let r: RedisResult<i64> = redis::cmd("ZUNIONSTORE").arg("zus8_dest").arg(2).arg("zus8_a").arg("zus8_str").query(&mut con);
        assert!(r.is_err());
    }

    #[test]
    fn test_zunionstore_invalid_args() {
        let mut con = setup();
        let _: () = con.del("zus9_dest").unwrap();
        // numkeys = 0
        let r1: RedisResult<i64> = redis::cmd("ZUNIONSTORE").arg("zus9_dest").arg(0).query(&mut con);
        assert!(r1.is_err());
        // WEIGHTS 长度不对
        let _: () = con.del("zus9_a").unwrap();
        let _: i64 = con.zadd_multiple("zus9_a", &[(1.0, "a")]).unwrap();
        let r2: RedisResult<i64> = redis::cmd("ZUNIONSTORE").arg("zus9_dest").arg(1).arg("zus9_a")
            .arg("WEIGHTS").arg(1).arg(2).query(&mut con);
        assert!(r2.is_err());
        // AGGREGATE 不识别
        let r3: RedisResult<i64> = redis::cmd("ZUNIONSTORE").arg("zus9_dest").arg(1).arg("zus9_a")
            .arg("AGGREGATE").arg("FOO").query(&mut con);
        assert!(r3.is_err());
    }
}
