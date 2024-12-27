#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use redis::{Client, Commands, Connection};

    fn setup() -> Connection {
        let client = Client::open("redis://127.0.0.1:6379/").unwrap();
        client.get_connection().unwrap()
    }

    #[test]
    fn test_set() {
        let mut con = setup();
        let _: () = con.set("test", "Helloword").unwrap();
        let get_set_result: String = con.get("test").unwrap();
        assert_eq!(get_set_result, "Helloword");
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
}
