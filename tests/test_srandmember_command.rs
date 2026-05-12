use std::collections::HashSet;

#[tokio::test]
async fn test_srandmember_command() {
    let client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
    let mut con = client.get_connection().unwrap();

    // clean up any keys that may be present
    let _: () = redis::cmd("DEL").arg("test_srandmember_key").query(&mut con).unwrap_or(());

    // add elements to collections
    let _: () = redis::cmd("SADD")
        .arg("test_srandmember_key")
        .arg("one")
        .arg("two")
        .arg("three")
        .arg("four")
        .arg("five")
        .query(&mut con)
        .unwrap();

    // Test SRANDMEMBER - does not specify a count, returns a single random element
    let result: Option<String> = redis::cmd("SRANDMEMBER")
        .arg("test_srandmember_key")
        .query(&mut con)
        .unwrap();
    assert!(result.is_some());
    assert!(vec!["one", "two", "three", "four", "five"].contains(&result.unwrap().as_str()));

    // Verify that the collection has not been modified (the number of elements remains the same)
    let count: i32 = redis::cmd("SCARD")
        .arg("test_srandmember_key")
        .query(&mut con)
        .unwrap();
    assert_eq!(count, 5);
}

#[tokio::test]
async fn test_srandmember_with_positive_count() {
    let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
    let mut con = client.get_connection().unwrap();

    // clean up any keys that may be present
    let _: () = redis::cmd("DEL").arg("test_srandmember_positive").query(&mut con).unwrap_or(());

    // 添加元素到集合
    let _: () = redis::cmd("SADD")
        .arg("test_srandmember_positive")
        .arg("one")
        .arg("two")
        .arg("three")
        .arg("four")
        .arg("five")
        .query(&mut con)
        .unwrap();

    // Test SRANDMEMBER - Specifies a positive count, returning a unique random element
    let result: Vec<String> = redis::cmd("SRANDMEMBER")
        .arg("test_srandmember_positive")
        .arg(3)
        .query(&mut con)
        .unwrap();
    assert_eq!(result.len(), 3);
    
    // Verify that all elements are in the original collection
    let valid_items = vec!["one", "two", "three", "four", "five"];
    for item in &result {
        assert!(valid_items.contains(&item.as_str()));
    }
    
    // verify that the element is not duplicated
    let mut seen = HashSet::new();
    for item in &result {
        assert!(seen.insert(item), "Found duplicate element: {}", item);
    }

    // verify that the collection has not been modified
    let count: i32 = redis::cmd("SCARD")
        .arg("test_srandmember_positive")
        .query(&mut con)
        .unwrap();
    assert_eq!(count, 5);
}

#[tokio::test]
async fn test_srandmember_with_negative_count() {
    let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
    let mut con = client.get_connection().unwrap();

    // clean up any keys that may be present
    let _: () = redis::cmd("DEL").arg("test_srandmember_negative").query(&mut con).unwrap_or(());

    // add elements to collections
    let _: () = redis::cmd("SADD")
        .arg("test_srandmember_negative")
        .arg("one")
        .arg("two")
        .arg("three")
        .query(&mut con)
        .unwrap();

    // Test SRANDMEMBER - Specifies a negative count, returning random elements that may be repeated
    let result: Vec<String> = redis::cmd("SRANDMEMBER")
        .arg("test_srandmember_negative")
        .arg(-5)
        .query(&mut con)
        .unwrap();
    assert_eq!(result.len(), 5);
    
    // Verify that all elements are in the original collection
    let valid_items = vec!["one", "two", "three"];
    for item in &result {
        assert!(valid_items.contains(&item.as_str()));
    }

    // verify that the collection has not been modified
    let count: i32 = redis::cmd("SCARD")
        .arg("test_srandmember_negative")
        .query(&mut con)
        .unwrap();
    assert_eq!(count, 3);
}

#[tokio::test]
async fn test_srandmember_empty_set() {
    let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
    let mut con = client.get_connection().unwrap();

    // clean up any keys that may be present
    let _: () = redis::cmd("DEL").arg("test_srandmember_empty").query(&mut con).unwrap_or(());

    // Test SRANDMEMBER - empty collection, return nil
    let result: Option<String> = redis::cmd("SRANDMEMBER")
        .arg("test_srandmember_empty")
        .query(&mut con)
        .unwrap();
    assert_eq!(result, None);
}

#[tokio::test]
async fn test_srandmember_nonexistent_key() {
    let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
    let mut con = client.get_connection().unwrap();

    // clean up any keys that may be present
    let _: () = redis::cmd("DEL").arg("test_srandmember_nonexistent").query(&mut con).unwrap_or(());

    // Test SRANDMEMBER - non-existent key, returns nil
    let result: Option<String> = redis::cmd("SRANDMEMBER")
        .arg("test_srandmember_nonexistent")
        .query(&mut con)
        .unwrap();
    assert_eq!(result, None);
}

