#[cfg(test)]
mod tests {
    use redis::{Client, Commands, Connection};

    fn setup() -> Connection {
        let client = Client::open("redis://127.0.0.1:6379/").unwrap();
        client.get_connection().unwrap()
    }

    #[test]
    fn test_set_get() {

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
        let _: () = con.set("del_test", "Helloword").unwrap();

        // 获取 key 为 "test" 的值
        let get_set_result: String = con.get("del_test").unwrap();

        // 断言获取到的值与预期值相等
        assert_eq!(get_set_result, "Helloword");

        // 删除 key 为 "test" 的值
        let _: () = con.del("del_test").unwrap();

        // 获取 key 为 "test" 的值
        let get_del_result: Option<String> = con.get("del_test").unwrap();

        // 断言获取到的值与预期值相等
        assert_eq!(get_del_result, None);
    }

    #[test]
    fn test_append() {

        let mut con = setup();

        // 设置 key 为 "test"，值为 "Hello, Redis!"
        let _: () = con.set("append-test", "Hello").unwrap();

        let _: () = con.append("append-test", "Append").unwrap();

        let get_result: String = con.get("append-test").unwrap();

        assert_eq!(get_result, "HelloAppend");
    }


}
