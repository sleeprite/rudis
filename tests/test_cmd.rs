#[cfg(test)]
mod tests {

    use std::process::{Child, Command};
    use std::sync::Mutex;
    use std::{thread::sleep, time::Duration};

    use redis::{Client, Commands, Connection};

    const TEST_PORT: &str = "16379";
    static SERVER: Mutex<Option<Child>> = Mutex::new(None);

    /// 自动启动测试专用的 rudis-server 子进程（端口 16379），仅首次调用时启动
    fn ensure_server_started() {
        let mut guard = SERVER.lock().unwrap();
        if guard.is_some() {
            return;
        }

        // 优先通过 cargo 环境变量获取二进制路径，否则回退到默认路径
        let server_path = std::env::var("CARGO_BIN_EXE_rudis-server").unwrap_or_else(|_| {
                let mut p = std::env::current_dir().unwrap();
                p.push("target");
                p.push("debug");
                p.push("rudis-server.exe");
                p.to_string_lossy().to_string()
            });

        let mut child = Command::new(&server_path).arg("--port").arg(TEST_PORT).arg("--loglevel").arg("error")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .unwrap_or_else(|e| panic!("Failed to start test server at {}: {}", server_path, e));

        // 轮询等待服务就绪（最多 5 秒）
        let url = format!("redis://127.0.0.1:{}/", TEST_PORT);
        for i in 0..50 {
            sleep(Duration::from_millis(100));
            if let Ok(client) = Client::open(url.as_str()) {
                if client.get_connection().is_ok() {
                    *guard = Some(child);
                    return;
                }
            }
            if i == 49 {
                let _ = child.kill();
                panic!("Test server failed to start within 5 seconds");
            }
        }
    }

    fn setup() -> Connection {
        ensure_server_started();
        let url = format!("redis://127.0.0.1:{}/", TEST_PORT);
        let client = Client::open(url.as_str()).unwrap();
        match client.get_connection() {
            Ok(conn) => conn,
            Err(e) => {
                eprintln!("Failed to get connection: {}", e);
                panic!("Failed to get connection: {}", e);
            }
        }
    }

    #[test]
    fn test_set() {
        let mut con = setup();
        let _: () = con.set("test", "Helloword").unwrap();
        let get_set_result: String = con.get("test").unwrap();
        assert_eq!(get_set_result, "Helloword");
    }

    #[test]
    fn test_set_batch() {
        let mut con = setup();
        for i in 0..1000 {
            let _: () = con.set(i.to_string(), i.to_string()).unwrap();
        }
    }

    #[test]
    fn test_get_batch() {
        let mut con = setup();
        for _i in 0..1000 {
            let _: () = con.get("user").unwrap();
        }
    }

    #[test]
    fn test_del() {
        
        let mut con = setup();
        let _: () = con.set("del-test", "Helloword").unwrap();

        let get_set_result: String = con.get("del-test").unwrap();
        assert_eq!(get_set_result, "Helloword");

        let _: () = con.del("del-test").unwrap();
        let get_del_result: Option<String> = con.get("del-test").unwrap();
        assert_eq!(get_del_result, None);
    }

    #[test]
    fn test_append() {
        let mut con = setup();

        let _: () = con.set("append-test", "Hello").unwrap();
        let _: () = con.append("append-test", "word").unwrap();
        let get_result: String = con.get("append-test").unwrap();
        assert_eq!(get_result, "Helloword");
    }

    #[test]
    fn test_setrange() {
        let mut con = setup();

        // 测试基本的 setrange 功能
        let _: () = con.set("setrange-test", "Hello World").unwrap();
        let result: usize = con.setrange("setrange-test", 6, "Redis").unwrap();
        assert_eq!(result, 11);
        
        let get_result: String = con.get("setrange-test").unwrap();
        assert_eq!(get_result, "Hello Redis");

        // 测试扩展字符串长度的情况
        let _: () = con.set("setrange-extend", "Hello").unwrap();
        let result: usize = con.setrange("setrange-extend", 10, "World").unwrap();
        assert_eq!(result, 15);
        
        let get_result: String = con.get("setrange-extend").unwrap();
        // 注意：中间会有空字节，所以结果可能不是直观的 "Hello     World"
        assert_eq!(get_result.len(), 15);
    }

    #[test]
    fn test_exists() {
        let mut con = setup();

        let _: () = con.set("exists-test", "Helloworld").unwrap();
        let key_exists: bool = con.exists("exists-test").unwrap();
        assert_eq!(key_exists, true);
    }

    #[test]
    fn test_rename() {
        let mut con = setup();
        let _: () = con.set("rename-test", "Helloworld").unwrap();
        let _: () = con.rename("rename-test", "rename-new-test").unwrap();
        let key_exists: bool = con.exists("rename-new-test").unwrap();
        println!("是否存在：{}", key_exists);
        assert_eq!(key_exists, true);
    }

    #[test]
    fn test_keys() {
        
        let mut con = setup();

        let _: () = con.set("keys-1-test", "Helloworld").unwrap();
        let _: () = con.set("keys-2-test", "Helloworld").unwrap();
        let _: () = con.set("keys-3-test", "Helloworld").unwrap();

        let result: Vec<String> = con.keys("keys*").unwrap();
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_llen() {
        
        let mut con = setup();

        let _: () = con.del("llen-test").unwrap();
        let _: () = con.rpush("llen-test", "Helloworld").unwrap();
        let _: () = con.rpush("llen-test", "Helloworld").unwrap();
        let _: () = con.rpush("llen-test", "Helloworld").unwrap();

        let count: usize = con.llen("llen-test").unwrap();
        assert_eq!(count, 3);
    }

    #[test]
    fn test_rpush() {
        
        let mut con = setup();

        let _: () = con.del("rpush-test").unwrap();
        let _: () = con.rpush("rpush-test", "Helloworld1").unwrap();
        let _: () = con.rpush("rpush-test", "Helloworld2").unwrap();
        let _: () = con.rpush("rpush-test", "Helloworld3").unwrap();

        let value: String = con.lindex("rpush-test",0).unwrap();

        assert_eq!(value, "Helloworld1");
    }
    
    #[test]
    fn test_lpush() {
        
        let mut con = setup();

        let _: () = con.del("lpush-test").unwrap();
        let _: () = con.lpush("lpush-test", "Helloworld1").unwrap();
        let _: () = con.lpush("lpush-test", "Helloworld2").unwrap();
        let _: () = con.lpush("lpush-test", "Helloworld3").unwrap();

        let value: String = con.lindex("lpush-test",0).unwrap();
        
        assert_eq!(value, "Helloworld3");
    }

    #[test]
    fn test_sadd() {

        let mut con = setup();

        let _: () = con.del("sadd-test").unwrap();
        let _: () = con.sadd("sadd-test", "admin1").unwrap(); 
        let _: () = con.sadd("sadd-test", "admin2").unwrap(); 
        let _: () = con.sadd("sadd-test", "admin3").unwrap(); 
    
        let count: usize = con.scard("sadd-test").unwrap();
        assert_eq!(count, 3);

        let members: Vec<String> =  con.smembers("sadd-test").unwrap();
        assert_eq!(members.len(), 3);
    }

    #[test]
    fn test_expire () {

        let mut con = setup();
        let _: () = con.set("test-expire", "Helloword").unwrap();
        let _: () = con.expire("test-expire", 3).unwrap();
        
        sleep(Duration::from_secs(2));

        let value1: Option<String> = con.get("test-expire").unwrap();
        assert_eq!(value1, Some("Helloword".to_string()));

        sleep(Duration::from_secs(2));

        let value2: Option<String> = con.get("test-expire").unwrap();
        assert_eq!(value2, None);
    }

    #[test]
    fn test_hmset() {

        let mut con = setup();

        let data: [(String, String); 3] = [
            ("name".to_string(), "Alice".to_string()),
            ("age".to_string(), "30".to_string()),
            ("email".to_string(), "alice@example.com".to_string()),
        ];

        let _: () = con.del("test-hmset").unwrap();
        let _: () = con.hset_multiple("test-hmset", &data).unwrap();

        let name: String = con.hget("test-hmset", "name").unwrap();
        assert_eq!(name, "Alice");
    
        let _: () = con.hdel("test-hmset", "email").unwrap();

        let email: Option<String> = con.hget("test-hmset", "email").unwrap();
        assert_eq!(email, None);

        let _:() = con.hset("test-hmset", "sex", "boy").unwrap();

        let sex: String = con.hget("test-hmset", "sex").unwrap();
        assert_eq!(sex, "boy");

        let exists: usize = con.hexists("test-hmset", "city").unwrap();
        assert_eq!(exists, 0);
    }

    #[test]
    fn test_ltrim() {
        let mut con = setup();

        // 准备测试数据
        let _: () = con.del("ltrim-test").unwrap();
        let _: () = con.rpush("ltrim-test", "hello").unwrap();
        let _: () = con.rpush("ltrim-test", "hello").unwrap();
        let _: () = con.rpush("ltrim-test", "foo").unwrap();
        let _: () = con.rpush("ltrim-test", "bar").unwrap();

        // 执行 LTRIM 命令
        let result: String = con.ltrim("ltrim-test", 1, -1).unwrap();
        assert_eq!(result, "OK");

        // 验证结果
        let range: Vec<String> = con.lrange("ltrim-test", 0, -1).unwrap();
        assert_eq!(range.len(), 3);
        assert_eq!(range[0], "hello");
        assert_eq!(range[1], "foo");
        assert_eq!(range[2], "bar");
    }

    #[test]
    fn test_ltrim_out_of_range() {
        let mut con = setup();

        // 准备测试数据
        let _: () = con.del("ltrim-out-of-range-test").unwrap();
        let _: () = con.rpush("ltrim-out-of-range-test", "hello").unwrap();
        let _: () = con.rpush("ltrim-out-of-range-test", "world").unwrap();

        // 执行 LTRIM 命令，使用超出范围的索引
        let result: String = con.ltrim("ltrim-out-of-range-test", 5, 10).unwrap();
        assert_eq!(result, "OK");

        // 验证结果 - 列表应该为空
        let range: Vec<String> = con.lrange("ltrim-out-of-range-test", 0, -1).unwrap();
        assert_eq!(range.len(), 0);
    }

    #[test]
    fn test_ltrim_with_negative_indices() {
        let mut con = setup();

        // 准备测试数据
        let _: () = con.del("ltrim-negative-indices-test").unwrap();
        let _: () = con.rpush("ltrim-negative-indices-test", "one").unwrap();
        let _: () = con.rpush("ltrim-negative-indices-test", "two").unwrap();
        let _: () = con.rpush("ltrim-negative-indices-test", "three").unwrap();
        let _: () = con.rpush("ltrim-negative-indices-test", "four").unwrap();
        let _: () = con.rpush("ltrim-negative-indices-test", "five").unwrap();

        // 执行 LTRIM 命令，使用负数索引
        let result: String = con.ltrim("ltrim-negative-indices-test", -3, -1).unwrap();
        assert_eq!(result, "OK");

        // 验证结果 - 应该保留最后3个元素
        let range: Vec<String> = con.lrange("ltrim-negative-indices-test", 0, -1).unwrap();
        assert_eq!(range.len(), 3);
        assert_eq!(range[0], "three");
        assert_eq!(range[1], "four");
        assert_eq!(range[2], "five");
    }

    #[test]
    fn test_ltrim_entire_list() {
        let mut con = setup();

        // 准备测试数据
        let _: () = con.del("ltrim-entire-list-test").unwrap();
        let _: () = con.rpush("ltrim-entire-list-test", "a").unwrap();
        let _: () = con.rpush("ltrim-entire-list-test", "b").unwrap();
        let _: () = con.rpush("ltrim-entire-list-test", "c").unwrap();

        // 执行 LTRIM 命令，保留整个列表
        let result: String = con.ltrim("ltrim-entire-list-test", 0, -1).unwrap();
        assert_eq!(result, "OK");

        // 验证结果 - 列表应该保持不变
        let range: Vec<String> = con.lrange("ltrim-entire-list-test", 0, -1).unwrap();
        assert_eq!(range.len(), 3);
        assert_eq!(range[0], "a");
        assert_eq!(range[1], "b");
        assert_eq!(range[2], "c");
    }

    #[test]
    fn test_ltrim_empty_list() {
        let mut con = setup();

        // 准备空列表
        let _: () = con.del("ltrim-empty-list-test").unwrap();

        // 执行 LTRIM 命令在空列表上
        let result: String = con.ltrim("ltrim-empty-list-test", 0, -1).unwrap();
        assert_eq!(result, "OK");

        // 验证结果 - 列表应该仍然为空
        let range: Vec<String> = con.lrange("ltrim-empty-list-test", 0, -1).unwrap();
        assert_eq!(range.len(), 0);
    }

    #[test]
    fn test_ltrim_start_greater_than_end() {
        let mut con = setup();

        // 准备测试数据
        let _: () = con.del("ltrim-start-greater-test").unwrap();
        let _: () = con.rpush("ltrim-start-greater-test", "x").unwrap();
        let _: () = con.rpush("ltrim-start-greater-test", "y").unwrap();
        let _: () = con.rpush("ltrim-start-greater-test", "z").unwrap();

        // 执行 LTRIM 命令，start 大于 end
        let result: String = con.ltrim("ltrim-start-greater-test", 2, 1).unwrap();
        assert_eq!(result, "OK");

        // 验证结果 - 列表应该为空
        let range: Vec<String> = con.lrange("ltrim-start-greater-test", 0, -1).unwrap();
        assert_eq!(range.len(), 0);
    }

    #[test]
    fn test_ltrim_large_negative_start() {
        let mut con = setup();

        // 准备测试数据
        let _: () = con.del("ltrim-large-negative-start-test").unwrap();
        let _: () = con.rpush("ltrim-large-negative-start-test", "a").unwrap();
        let _: () = con.rpush("ltrim-large-negative-start-test", "b").unwrap();
        let _: () = con.rpush("ltrim-large-negative-start-test", "c").unwrap();

        // 执行 LTRIM 命令，使用很大的负数作为起始索引
        let result: String = con.ltrim("ltrim-large-negative-start-test", -10, -1).unwrap();
        assert_eq!(result, "OK");

        // 验证结果 - 应该保留整个列表（因为负数索引超出了范围，会被调整为0）
        let range: Vec<String> = con.lrange("ltrim-large-negative-start-test", 0, -1).unwrap();
        assert_eq!(range.len(), 3);
        assert_eq!(range[0], "a");
        assert_eq!(range[1], "b");
        assert_eq!(range[2], "c");
    }

    #[test]
    fn test_ltrim_single_element() {
        let mut con = setup();

        // 准备测试数据
        let _: () = con.del("ltrim-single-element-test").unwrap();
        let _: () = con.rpush("ltrim-single-element-test", "only").unwrap();

        // 执行 LTRIM 命令，只保留一个元素
        let result: String = con.ltrim("ltrim-single-element-test", 0, 0).unwrap();
        assert_eq!(result, "OK");

        // 验证结果
        let range: Vec<String> = con.lrange("ltrim-single-element-test", 0, -1).unwrap();
        assert_eq!(range.len(), 1);
        assert_eq!(range[0], "only");
    }

    #[test]
    fn test_ltrim_stop_exceeds_length() {
        let mut con = setup();

        // 准备测试数据
        let _: () = con.del("ltrim-stop-exceeds-test").unwrap();
        let _: () = con.rpush("ltrim-stop-exceeds-test", "first").unwrap();
        let _: () = con.rpush("ltrim-stop-exceeds-test", "second").unwrap();
        let _: () = con.rpush("ltrim-stop-exceeds-test", "third").unwrap();

        // 执行 LTRIM 命令，停止索引超过列表长度
        let result: String = con.ltrim("ltrim-stop-exceeds-test", 1, 100).unwrap();
        assert_eq!(result, "OK");

        // 验证结果 - 应该保留从索引1开始到列表末尾的所有元素
        let range: Vec<String> = con.lrange("ltrim-stop-exceeds-test", 0, -1).unwrap();
        assert_eq!(range.len(), 2);
        assert_eq!(range[0], "second");
        assert_eq!(range[1], "third");
    }

    #[test]
    fn test_ttl_command() {
        let mut con = setup();

        // 测试键不存在的情况
        let ttl_result: i32 = con.ttl("non-existent-key").unwrap();
        assert_eq!(ttl_result, -2);

        // 测试键存在但没有设置过期时间的情况
        let _: () = con.set("no-expire-key", "value").unwrap();
        let ttl_result: i32 = con.ttl("no-expire-key").unwrap();
        assert_eq!(ttl_result, -1);

        // 测试键存在且设置了过期时间的情况
        let _: () = con.set("expire-key", "value").unwrap();
        let _: () = con.expire("expire-key", 10).unwrap();
        let ttl_result: i32 = con.ttl("expire-key").unwrap();
        // TTL 应该在 9 到 10 秒之间
        assert!(ttl_result >= 9 && ttl_result <= 10);
    }

    #[test]
    fn test_pttl_command() {
        let mut con = setup();

        // 测试键不存在的情况
        let pttl_result: i32 = con.pttl("non-existent-key").unwrap();
        assert_eq!(pttl_result, -2);

        // 测试键存在但没有设置过期时间的情况
        let _: () = con.set("no-expire-key", "value").unwrap();
        let pttl_result: i32 = con.pttl("no-expire-key").unwrap();
        assert_eq!(pttl_result, -1);

        // 测试键存在且设置了过期时间的情况
        let _: () = con.set("expire-key", "value").unwrap();
        let _: () = con.pexpire("expire-key", 10000).unwrap(); // 10秒 = 10000毫秒
        let pttl_result: i32 = con.pttl("expire-key").unwrap();
        // PTTL 应该在 9000 到 10000 毫秒之间
        assert!(pttl_result >= 9000 && pttl_result <= 10000);
    }
}
