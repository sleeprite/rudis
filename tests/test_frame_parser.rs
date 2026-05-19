#[cfg(test)]
mod tests {
    use rudis_server::frame::Frame;
    use anyhow::Error;

    struct TestCase {
        name: &'static str,
        resp_bytes: &'static [u8],
        expected_args: Vec<&'static str>,
    }

    fn run_tests(cases: &[TestCase]) -> (usize, usize, Vec<String>) {
        let mut passed = 0;
        let mut failed = Vec::new();

        for case in cases {
            match Frame::parse_from_bytes(case.resp_bytes) {
                Ok(frame) => {
                    let args = frame.get_args();
                    if args.len() != case.expected_args.len() {
                        failed.push(format!(
                            "FAIL [{}]: arg count mismatch, expected {} got {}: {:?}",
                            case.name,
                            case.expected_args.len(),
                            args.len(),
                            args
                        ));
                        continue;
                    }

                    let mut mismatch = false;
                    for (i, (actual, expected)) in args.iter().zip(case.expected_args.iter()).enumerate() {
                        if actual != *expected {
                            failed.push(format!(
                                "FAIL [{}]: arg[{}] expected '{}' got '{}'",
                                case.name, i, expected, actual
                            ));
                            mismatch = true;
                            break;
                        }
                    }
                    if !mismatch {
                        passed += 1;
                    }
                }
                Err(e) => {
                    failed.push(format!("FAIL [{}]: parse error: {}", case.name, e));
                }
            }
        }

        (passed, passed + failed.len(), failed)
    }

    #[test]
    fn test_frame_parser() -> Result<(), Error> {
        let cases = vec![
            // === 基础命令 ===
            TestCase {
                name: "SET key value",
                resp_bytes: b"*3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$5\r\nvalue\r\n",
                expected_args: vec!["SET", "key", "value"],
            },
            TestCase {
                name: "GET key",
                resp_bytes: b"*2\r\n$3\r\nGET\r\n$3\r\nkey\r\n",
                expected_args: vec!["GET", "key"],
            },
            TestCase {
                name: "DEL key",
                resp_bytes: b"*2\r\n$3\r\nDEL\r\n$3\r\nkey\r\n",
                expected_args: vec!["DEL", "key"],
            },

            // === KEYS 通配符 (之前 * 侥幸通过，*root 会丢失) ===
            TestCase {
                name: "KEYS *",
                resp_bytes: b"*2\r\n$4\r\nKEYS\r\n$1\r\n*\r\n",
                expected_args: vec!["KEYS", "*"],
            },
            TestCase {
                name: "KEYS *root",
                resp_bytes: b"*2\r\n$4\r\nKEYS\r\n$5\r\n*root\r\n",
                expected_args: vec!["KEYS", "*root"],
            },

            // === 值以 * 开头 (核心回归) ===
            TestCase {
                name: "SET key *value",
                resp_bytes: b"*3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$6\r\n*value\r\n",
                expected_args: vec!["SET", "key", "*value"],
            },
            TestCase {
                name: "SET key *2",
                resp_bytes: b"*3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$2\r\n*2\r\n",
                expected_args: vec!["SET", "key", "*2"],
            },
            TestCase {
                name: "SET key **double",
                resp_bytes: b"*3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$8\r\n**double\r\n",
                expected_args: vec!["SET", "key", "**double"],
            },

            // === 值以 $ 开头 ===
            TestCase {
                name: "SET key $dollar",
                resp_bytes: b"*3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$7\r\n$dollar\r\n",
                expected_args: vec!["SET", "key", "$dollar"],
            },
            TestCase {
                name: "SET key $5",
                resp_bytes: b"*3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$2\r\n$5\r\n",
                expected_args: vec!["SET", "key", "$5"],
            },

            // === 空值 / 特殊字符 ===
            TestCase {
                name: "SET key (empty)",
                resp_bytes: b"*3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$0\r\n\r\n",
                expected_args: vec!["SET", "key", ""],
            },
            TestCase {
                name: "SET key with spaces in value",
                resp_bytes: b"*3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$11\r\nhello world\r\n",
                expected_args: vec!["SET", "key", "hello world"],
            },

            // === 多参数命令 ===
            TestCase {
                name: "MSET k1 v1 k2 v2",
                resp_bytes: b"*5\r\n$4\r\nMSET\r\n$2\r\nk1\r\n$2\r\nv1\r\n$2\r\nk2\r\n$2\r\nv2\r\n",
                expected_args: vec!["MSET", "k1", "v1", "k2", "v2"],
            },
            TestCase {
                name: "MGET k1 k2 k3",
                resp_bytes: b"*4\r\n$4\r\nMGET\r\n$2\r\nk1\r\n$2\r\nk2\r\n$2\r\nk3\r\n",
                expected_args: vec!["MGET", "k1", "k2", "k3"],
            },
            TestCase {
                name: "HSET hash f1 v1 f2 v2",
                resp_bytes: b"*6\r\n$4\r\nHSET\r\n$4\r\nhash\r\n$2\r\nf1\r\n$2\r\nv1\r\n$2\r\nf2\r\n$2\r\nv2\r\n",
                expected_args: vec!["HSET", "hash", "f1", "v1", "f2", "v2"],
            },

            // === CLIENT 命令 (类似 sticky commands 场景) ===
            TestCase {
                name: "CLIENT SETINFO LIB-NAME rust",
                resp_bytes: b"*4\r\n$6\r\nCLIENT\r\n$7\r\nSETINFO\r\n$8\r\nLIB-NAME\r\n$4\r\nrust\r\n",
                expected_args: vec!["CLIENT", "SETINFO", "LIB-NAME", "rust"],
            },

            // === 数字值 ===
            TestCase {
                name: "INCRBY key 100",
                resp_bytes: b"*3\r\n$6\r\nINCRBY\r\n$3\r\nkey\r\n$3\r\n100\r\n",
                expected_args: vec!["INCRBY", "key", "100"],
            },

            // === 中文值 ===
            TestCase {
                name: "SET key 你好世界",
                resp_bytes: b"*3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$12\r\n\xE4\xBD\xA0\xE5\xA5\xBD\xE4\xB8\x96\xE7\x95\x8C\r\n",
                expected_args: vec!["SET", "key", "你好世界"],
            },

            // === 粘包场景 ===
            TestCase {
                name: "EXPIRE key 60",
                resp_bytes: b"*3\r\n$6\r\nEXPIRE\r\n$3\r\nkey\r\n$2\r\n60\r\n",
                expected_args: vec!["EXPIRE", "key", "60"],
            },
        ];

        let (passed, total, failures) = run_tests(&cases);

        println!("\n========== Frame Parser Test Results ==========");
        println!("  Total:  {}", total);
        println!("  Passed: {}", passed);
        println!("  Failed: {}", total - passed);
        println!("  Rate:   {:.1}%", passed as f64 / total as f64 * 100.0);

        if !failures.is_empty() {
            println!("\nFailures:");
            for f in &failures {
                println!("  {}", f);
            }
        }
        println!("================================================\n");

        // 不允许有任何失败
        assert!(failures.is_empty(), "{} test(s) failed", failures.len());
        Ok(())
    }

    #[test]
    fn test_parse_multiple_frames_with_special_values() -> Result<(), Error> {
        // 验证粘包场景中也能正确解析带 * 的值
        // SET k1 *root   +   SET k2 normal
        let bytes = b"*3\r\n$3\r\nSET\r\n$2\r\nk1\r\n$5\r\n*root\r\n*3\r\n$3\r\nSET\r\n$2\r\nk2\r\n$6\r\nnormal\r\n";

        let frames = Frame::parse_multiple_frames(bytes)?;
        assert_eq!(frames.len(), 2);

        let args1 = frames[0].get_args();
        assert_eq!(args1, vec!["SET", "k1", "*root"]);

        let args2 = frames[1].get_args();
        assert_eq!(args2, vec!["SET", "k2", "normal"]);

        Ok(())
    }
}
