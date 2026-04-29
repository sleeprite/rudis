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
    fn test_zinterstore_default_sum() {
        let mut con = setup();
        seed_two(&mut con, "zis1_a", "zis1_b");
        let _: () = con.del("zis1_dest").unwrap();
        let n: i64 = redis::cmd("ZINTERSTORE").arg("zis1_dest").arg(2).arg("zis1_a").arg("zis1_b").query(&mut con).unwrap();
        assert_eq!(n, 2);
        let mut got = pairs_with_scores(&mut con, "zis1_dest");
        got.sort();
        assert_eq!(got, vec![
            ("b".to_string(), "12".to_string()),
            ("c".to_string(), "23".to_string()),
        ]);
    }

    #[test]
    fn test_zinterstore_aggregate_min() {
        let mut con = setup();
        seed_two(&mut con, "zis2_a", "zis2_b");
        let _: () = con.del("zis2_dest").unwrap();
        let _: i64 = redis::cmd("ZINTERSTORE").arg("zis2_dest").arg(2).arg("zis2_a").arg("zis2_b")
            .arg("AGGREGATE").arg("MIN").query(&mut con).unwrap();
        let mut got = pairs_with_scores(&mut con, "zis2_dest");
        got.sort();
        assert_eq!(got, vec![
            ("b".to_string(), "2".to_string()),
            ("c".to_string(), "3".to_string()),
        ]);
    }

    #[test]
    fn test_zinterstore_aggregate_max() {
        let mut con = setup();
        seed_two(&mut con, "zis3_a", "zis3_b");
        let _: () = con.del("zis3_dest").unwrap();
        let _: i64 = redis::cmd("ZINTERSTORE").arg("zis3_dest").arg(2).arg("zis3_a").arg("zis3_b")
            .arg("AGGREGATE").arg("MAX").query(&mut con).unwrap();
        let mut got = pairs_with_scores(&mut con, "zis3_dest");
        got.sort();
        assert_eq!(got, vec![
            ("b".to_string(), "10".to_string()),
            ("c".to_string(), "20".to_string()),
        ]);
    }

    #[test]
    fn test_zinterstore_weights() {
        let mut con = setup();
        seed_two(&mut con, "zis4_a", "zis4_b");
        let _: () = con.del("zis4_dest").unwrap();
        let _: i64 = redis::cmd("ZINTERSTORE").arg("zis4_dest").arg(2).arg("zis4_a").arg("zis4_b")
            .arg("WEIGHTS").arg(2).arg("0.5").query(&mut con).unwrap();
        let mut got = pairs_with_scores(&mut con, "zis4_dest");
        got.sort();
        // b: 2*2 + 10*0.5 = 9; c: 3*2 + 20*0.5 = 16
        assert_eq!(got, vec![
            ("b".to_string(), "9".to_string()),
            ("c".to_string(), "16".to_string()),
        ]);
    }

    #[test]
    fn test_zinterstore_with_set_input() {
        let mut con = setup();
        let _: () = con.del("zis5_zs").unwrap();
        let _: () = con.del("zis5_set").unwrap();
        let _: () = con.del("zis5_dest").unwrap();
        let _: i64 = con.zadd_multiple("zis5_zs", &[(5.0, "a"), (5.0, "b"), (5.0, "c")]).unwrap();
        let _: i64 = con.sadd("zis5_set", &["b", "c", "d"][..]).unwrap();
        let n: i64 = redis::cmd("ZINTERSTORE").arg("zis5_dest").arg(2).arg("zis5_zs").arg("zis5_set").query(&mut con).unwrap();
        assert_eq!(n, 2);
        let mut got = pairs_with_scores(&mut con, "zis5_dest");
        got.sort();
        assert_eq!(got, vec![
            ("b".to_string(), "6".to_string()),
            ("c".to_string(), "6".to_string()),
        ]);
    }

    #[test]
    fn test_zinterstore_missing_key_empty() {
        let mut con = setup();
        let _: () = con.del("zis6_a").unwrap();
        let _: () = con.del("zis6_missing").unwrap();
        let _: () = con.del("zis6_dest").unwrap();
        let _: i64 = con.zadd_multiple("zis6_a", &[(1.0, "a")]).unwrap();
        let _: i64 = con.zadd_multiple("zis6_dest", &[(99.0, "leftover")]).unwrap();
        let n: i64 = redis::cmd("ZINTERSTORE").arg("zis6_dest").arg(2).arg("zis6_a").arg("zis6_missing").query(&mut con).unwrap();
        assert_eq!(n, 0);
        let exists: i64 = redis::cmd("EXISTS").arg("zis6_dest").query(&mut con).unwrap();
        assert_eq!(exists, 0);
    }

    #[test]
    fn test_zinterstore_overwrites_dest() {
        let mut con = setup();
        seed_two(&mut con, "zis7_a", "zis7_b");
        let _: () = con.del("zis7_dest").unwrap();
        let _: i64 = con.zadd_multiple("zis7_dest", &[(99.0, "x")]).unwrap();
        let _: i64 = redis::cmd("ZINTERSTORE").arg("zis7_dest").arg(2).arg("zis7_a").arg("zis7_b").query(&mut con).unwrap();
        let exists_x: bool = redis::cmd("ZSCORE").arg("zis7_dest").arg("x").query::<Option<String>>(&mut con).unwrap().is_some();
        assert!(!exists_x);
    }

    #[test]
    fn test_zinterstore_no_overlap() {
        let mut con = setup();
        let _: () = con.del("zis8_a").unwrap();
        let _: () = con.del("zis8_b").unwrap();
        let _: () = con.del("zis8_dest").unwrap();
        let _: i64 = con.zadd_multiple("zis8_a", &[(1.0, "a"), (2.0, "b")]).unwrap();
        let _: i64 = con.zadd_multiple("zis8_b", &[(1.0, "c"), (2.0, "d")]).unwrap();
        let _: i64 = con.zadd_multiple("zis8_dest", &[(99.0, "leftover")]).unwrap();
        let n: i64 = redis::cmd("ZINTERSTORE").arg("zis8_dest").arg(2).arg("zis8_a").arg("zis8_b").query(&mut con).unwrap();
        assert_eq!(n, 0);
        let exists: i64 = redis::cmd("EXISTS").arg("zis8_dest").query(&mut con).unwrap();
        assert_eq!(exists, 0);
    }

    #[test]
    fn test_zinterstore_wrong_type() {
        let mut con = setup();
        let _: () = con.del("zis9_a").unwrap();
        let _: i64 = con.zadd_multiple("zis9_a", &[(1.0, "a")]).unwrap();
        let _: () = con.del("zis9_str").unwrap();
        let _: () = con.set("zis9_str", "x").unwrap();
        let _: () = con.del("zis9_dest").unwrap();
        let r: RedisResult<i64> = redis::cmd("ZINTERSTORE").arg("zis9_dest").arg(2).arg("zis9_a").arg("zis9_str").query(&mut con);
        assert!(r.is_err());
    }

    #[test]
    fn test_zinterstore_invalid_args() {
        let mut con = setup();
        let _: () = con.del("zis10_dest").unwrap();
        let r1: RedisResult<i64> = redis::cmd("ZINTERSTORE").arg("zis10_dest").arg(0).query(&mut con);
        assert!(r1.is_err());
    }
}
