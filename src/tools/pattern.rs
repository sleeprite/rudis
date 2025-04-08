use regex::Regex;

pub fn is_match(key: &str, pattern: &str) -> bool {
    fn convert_pattern(pattern: &str) -> String {
        let mut regex_pattern = String::new();
        let mut chars = pattern.chars().peekable();
        while let Some(p) = chars.next() {
            match p {
                '*' => regex_pattern.push_str(".*"), 
                '?' => regex_pattern.push('.'),    
                '[' => {
                    regex_pattern.push('[');
                    if let Some(next) = chars.peek() {
                        if *next == '^' {
                            regex_pattern.push('^');
                            chars.next(); // 跳过 '^'
                        }
                    }
                    while let Some(ch) = chars.next() {
                        if ch == ']' {
                            break;
                        }
                        regex_pattern.push(ch);
                    }
                    regex_pattern.push(']');
                }
                _ => regex_pattern.push(p)
            }
        }
        regex_pattern
    }
    let regex_pattern = convert_pattern(pattern);
    let regex = Regex::new(&regex_pattern).unwrap();
    regex.is_match(key)
}