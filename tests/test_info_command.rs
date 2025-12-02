#[cfg(test)]
mod tests {
    use redis::{Client, Connection, cmd};

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

    #[test]
    fn test_info_command() {
        let mut con = setup();
        
        // Test basic INFO command using cmd macro
        let info_result: String = cmd("INFO").query(&mut con).unwrap();
        assert!(!info_result.is_empty());
        assert!(info_result.contains("# Server"));
        assert!(info_result.contains("# Clients"));
        assert!(info_result.contains("# Memory"));
        
        // Test INFO with specific section
        let server_info: String = cmd("INFO").arg("server").query(&mut con).unwrap();
        assert!(server_info.contains("# Server"));
        // Note: Even when requesting a specific section, Redis may still return other sections
        
        // Test INFO with all sections
        let all_info: String = cmd("INFO").arg("all").query(&mut con).unwrap();
        assert!(all_info.contains("# Server"));
        assert!(all_info.contains("# Clients"));
        assert!(all_info.contains("# Memory"));
        assert!(all_info.contains("# Persistence"));
        assert!(all_info.contains("# Stats"));
        assert!(all_info.contains("# Replication"));
        assert!(all_info.contains("# CPU"));
        assert!(all_info.contains("# Commandstats"));
        assert!(all_info.contains("# Keyspace"));
    }
}