#[cfg(test)]
mod tests {
    use redis::{Client, Commands, Connection};

    fn setup() -> Connection {
        let client = Client::open("redis://127.0.0.1:6379/").unwrap();
        client.get_connection().unwrap()
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
}
