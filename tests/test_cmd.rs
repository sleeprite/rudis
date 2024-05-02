#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use redis::{Client, Commands, Connection};
    use tokio::time::Sleep;

    fn setup() -> Connection {
        let client = Client::open("redis://127.0.0.1:6379/").unwrap();
        client.get_connection().unwrap()
    }

    #[test]
    fn test_set() {

        let mut con = setup();

        // 设置 key 为 "test"，值为 "Hello, Redis!"
        let _: () = con.set("test", "Helloword").unwrap();

        // 获取 key 为 "test" 的值
        let get_set_result: String = con.get("test").unwrap();

        // 断言获取到的值与预期值相等
        assert_eq!(get_set_result, "Helloword");
    }

    #[test]
    fn test_del() {
        
        let mut con = setup();

        // 设置 key 为 "test"，值为 "Hello, Redis!"
        let _: () = con.set("del-test", "Helloword").unwrap();

        // 获取 key 为 "test" 的值
        let get_set_result: String = con.get("del-test").unwrap();

        // 断言获取到的值与预期值相等
        assert_eq!(get_set_result, "Helloword");

        // 删除 key 为 "test" 的值
        let _: () = con.del("del-test").unwrap();

        // 获取 key 为 "test" 的值
        let get_del_result: Option<String> = con.get("del-test").unwrap();

        // 断言获取到的值与预期值相等
        assert_eq!(get_del_result, None);
    }

    #[test]
    fn test_append() {

        let mut con = setup();

        // 设置 key 为 "test"，值为 "Hello, Redis!"
        let _: () = con.set("append-test", "Hello").unwrap();

        let _: () = con.append("append-test", "word").unwrap();

        let get_result: String = con.get("append-test").unwrap();

        assert_eq!(get_result, "Helloword");
    }

    #[test]
    fn test_exists() {

        let mut con = setup();

        // 设置 key 为 "test"，值为 "Hello, Redis!"
        let _: () = con.set("exists-test", "Helloworld").unwrap();

        // 检查 key "test" 是否存在
        let key_exists: bool = con.exists("exists-test").unwrap();

        // 验证返回结果是否为 true，即键存在于 Redis 中
        assert_eq!(key_exists, true);
    }

    #[test]
    fn test_rename() {

        let mut con = setup();

        // 设置 key 为 "test"，值为 "Hello, Redis!"
        let _: () = con.set("rename-test", "Helloworld").unwrap();

        let _: () = con.rename("rename-test", "rename-new-test").unwrap();

        // 检查 key "test" 是否存在
        let key_exists: bool = con.exists("rename-new-test").unwrap();

        // 验证返回结果是否为 true，即键存在于 Redis 中
        assert_eq!(key_exists, true);
    }

    #[test]
    fn test_keys() {
        
        let mut con = setup();

        // 设置 key 为 "test"，值为 "Hello, Redis!"
        let _: () = con.set("keys-1-test", "Helloworld").unwrap();
        let _: () = con.set("keys-2-test", "Helloworld").unwrap();
        let _: () = con.set("keys-3-test", "Helloworld").unwrap();

        let result: Vec<String> = con.keys("keys*").unwrap();

        // 验证返回结果是否为 true，即键存在于 Redis 中
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
}
