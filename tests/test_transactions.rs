#[cfg(test)]
mod tests {
    use redis::{Client, Commands, Connection, RedisResult};

    /// 设置测试环境并返回 Redis 连接
    fn setup() -> Connection {
        let client = Client::open("redis://127.0.0.1:6379/").unwrap();
        match client.get_connection() {
            Ok(conn) => conn,
            Err(e) => {
                eprintln!("Failed to get connection: {}", e);
                panic!("Failed to get connection: {}", e);
            }
        }
    }

    /// 测试事务基本功能
    #[test]
    fn test_basic_transaction() {
        // 连接到服务器（假设服务器已经在运行）
        let mut con = setup();
        
        // 清理测试数据
        let _: () = con.del("key").unwrap();
        
        // 发送 MULTI 命令
        let result: RedisResult<()> = redis::cmd("MULTI").query(&mut con);
        assert!(result.is_ok());
        
        // 发送 SET 命令
        let result: RedisResult<()> = redis::cmd("SET").arg("key").arg("value").query(&mut con);
        assert!(result.is_ok());
        
        // 发送 GET 命令
        let result: RedisResult<()> = redis::cmd("GET").arg("key").query(&mut con);
        assert!(result.is_ok());
        
        // 发送 EXEC 命令
        let result: RedisResult<Vec<String>> = redis::cmd("EXEC").query(&mut con);
        assert!(result.is_ok());
        
        let results = result.unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0], "OK"); // SET 命令的结果
        assert_eq!(results[1], "value"); // GET 命令的结果
    }

    /// 测试 DISCARD 命令
    #[test]
    fn test_discard_transaction() {
        // 连接到服务器（假设服务器已经在运行）
        let mut con = setup();
        
        // 清理测试数据
        let _: () = con.del("discard_key").unwrap();
        
        // 发送 MULTI 命令
        let result: RedisResult<()> = redis::cmd("MULTI").query(&mut con);
        assert!(result.is_ok());
        
        // 发送 SET 命令
        let result: RedisResult<()> = redis::cmd("SET").arg("discard_key").arg("value").query(&mut con);
        assert!(result.is_ok());
        
        // 发送 DISCARD 命令
        let result: RedisResult<String> = redis::cmd("DISCARD").query(&mut con);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "OK");
        
        // 验证键未被设置
        let result: RedisResult<Option<String>> = con.get("discard_key");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    /// 测试在非事务模式下使用 EXEC 和 DISCARD
    #[test]
    fn test_exec_discard_without_multi() {
        // 连接到服务器（假设服务器已经在运行）
        let mut con = setup();
        
        // 发送 EXEC 命令（没有先发送 MULTI）
        let result: RedisResult<()> = redis::cmd("EXEC").query(&mut con);
        assert!(result.is_err());
        let err_msg = format!("{:?}", result.unwrap_err());
        assert!(err_msg.contains("EXEC without MULTI") || err_msg.contains("ERR"));
        
        // 发送 DISCARD 命令（没有先发送 MULTI）
        let result: RedisResult<()> = redis::cmd("DISCARD").query(&mut con);
        assert!(result.is_err());
        let err_msg = format!("{:?}", result.unwrap_err());
        assert!(err_msg.contains("DISCARD without MULTI") || err_msg.contains("ERR"));
    }
}