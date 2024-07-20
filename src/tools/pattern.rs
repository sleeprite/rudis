use regex::Regex;

/// 将给定的 `pattern` 进行处理，替换其中的 `?` 为 `.`，`*` 为 `.*`，并使用正则表达式匹配方式，判断传入的 `key` 是否符合该模式。
///
/// # 参数
///
/// - `key`: 待匹配的字符串引用
/// - `pattern`: 包含 `?` 和 `*` 通配符的模式字符串引用
///
/// # 返回值
///
/// 返回一个布尔值，表示 `key` 是否匹配 `pattern`
///
pub fn match_key(key: &str, pattern: &str) -> bool {
    let pattern = pattern.replace('?', ".").replace('*', ".*");
    let regex = Regex::new(&format!("^{}$", pattern)).unwrap();
    regex.is_match(key)
}