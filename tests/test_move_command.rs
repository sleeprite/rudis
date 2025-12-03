#[cfg(test)]
mod tests {
    use redis::{Client, Commands, Connection, RedisResult};

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

    // 使用Redis的通用命令执行方法来执行MOVE命令
    fn move_key(con: &mut Connection, key: &str, db_index: i32) -> RedisResult<i32> {
        redis::cmd("MOVE").arg(key).arg(db_index).query(con)
    }

    // 使用Redis的通用命令执行方法来执行SELECT命令
    fn select_db(con: &mut Connection, db_index: i32) -> RedisResult<()> {
        redis::cmd("SELECT").arg(db_index).query(con)
    }

    #[test]
    fn test_move_key_success() {
        let mut con = setup();
        
        // 先删除可能存在的键
        let _: () = con.del("move-test-key").unwrap();
        
        // 在数据库0中设置一个键
        let _: () = select_db(&mut con, 0).unwrap();
        let _: () = con.set("move-test-key", "test-value").unwrap();
        
        // 确认键在数据库0中存在
        let exists_in_db0: bool = con.exists("move-test-key").unwrap();
        assert_eq!(exists_in_db0, true);
        
        // 将键移动到数据库1
        let moved: i32 = move_key(&mut con, "move-test-key", 1).unwrap();
        assert_eq!(moved, 1);
        
        // 确认键在数据库0中已不存在
        let exists_in_db0_after: bool = con.exists("move-test-key").unwrap();
        assert_eq!(exists_in_db0_after, false);
        
        // 切换到数据库1并确认键存在
        let _: () = select_db(&mut con, 1).unwrap();
        let exists_in_db1: bool = con.exists("move-test-key").unwrap();
        assert_eq!(exists_in_db1, true);
        
        let value: String = con.get("move-test-key").unwrap();
        assert_eq!(value, "test-value");
        
        // 清理：删除测试键
        let _: () = con.del("move-test-key").unwrap();
    }

    #[test]
    fn test_move_nonexistent_key() {
        let mut con = setup();
        
        // 先删除可能存在的键
        let _: () = con.del("nonexistent-move-key").unwrap();
        
        // 尝试移动一个不存在的键
        let moved: i32 = move_key(&mut con, "nonexistent-move-key", 1).unwrap();
        assert_eq!(moved, 0);
    }

    #[test]
    fn test_move_to_same_database() {
        let mut con = setup();
        
        // 先删除可能存在的键
        let _: () = con.del("same-db-move-key").unwrap();
        
        // 在数据库0中设置一个键
        let _: () = select_db(&mut con, 0).unwrap();
        let _: () = con.set("same-db-move-key", "test-value").unwrap();
        
        // 尝试将键移动到同一个数据库
        let moved: i32 = move_key(&mut con, "same-db-move-key", 0).unwrap();
        assert_eq!(moved, 0); // 应该返回0，因为目标数据库中已经存在同名键
        
        // 确认键仍在原数据库中
        let exists: bool = con.exists("same-db-move-key").unwrap();
        assert_eq!(exists, true);
        
        let value: String = con.get("same-db-move-key").unwrap();
        assert_eq!(value, "test-value");
        
        // 清理
        let _: () = con.del("same-db-move-key").unwrap();
    }

    #[test]
    fn test_move_key_already_exists_in_target() {
        let mut con = setup();
        
        // 先删除可能存在的键
        let _: () = con.del("source-key").unwrap();
        let _: () = con.del("target-key").unwrap();
        
        // 在数据库0中设置源键
        let _: () = select_db(&mut con, 0).unwrap();
        let _: () = con.set("source-key", "source-value").unwrap();
        
        // 在数据库1中设置同名的目标键
        let _: () = select_db(&mut con, 1).unwrap();
        let _: () = con.set("source-key", "target-value").unwrap();
        
        // 切换回数据库0并尝试移动键
        let _: () = select_db(&mut con, 0).unwrap();
        let moved: i32 = move_key(&mut con, "source-key", 1).unwrap();
        assert_eq!(moved, 0); // 应该返回0，因为目标数据库中已存在同名键
        
        // 确认源键仍在数据库0中
        let exists_in_db0: bool = con.exists("source-key").unwrap();
        assert_eq!(exists_in_db0, true);
        
        let value: String = con.get("source-key").unwrap();
        assert_eq!(value, "source-value");
        
        // 切换到数据库1并确认目标键未被覆盖
        let _: () = select_db(&mut con, 1).unwrap();
        let value: String = con.get("source-key").unwrap();
        assert_eq!(value, "target-value");
        
        // 清理
        let _: () = select_db(&mut con, 0).unwrap();
        let _: () = con.del("source-key").unwrap();
        let _: () = select_db(&mut con, 1).unwrap();
        let _: () = con.del("source-key").unwrap();
    }

    #[test]
    fn test_move_invalid_database_index() {
        let mut con = setup();
        
        // 先删除可能存在的键
        let _: () = con.del("invalid-db-key").unwrap();
        
        // 在数据库0中设置一个键
        let _: () = select_db(&mut con, 0).unwrap();
        let _: () = con.set("invalid-db-key", "test-value").unwrap();
        
        // 尝试移动到无效的数据库索引（假设rudis默认只配置了16个数据库）
        // 这应该会返回一个错误，因为数据库索引超出范围
        let result = move_key(&mut con, "invalid-db-key", 100);
        assert!(result.is_err());
        
        // 确认键仍在原数据库中
        let exists: bool = con.exists("invalid-db-key").unwrap();
        assert_eq!(exists, true);
        
        // 清理
        let _: () = con.del("invalid-db-key").unwrap();
    }
}