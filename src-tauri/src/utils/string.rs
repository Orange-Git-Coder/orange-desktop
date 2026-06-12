/// 截断字符串
pub fn truncate(s: &str, max_len: usize) -> &str {
    if s.len() <= max_len {
        s
    } else {
        &s[..max_len]
    }
}

/// 去除字符串两端空白
pub fn trim(s: &str) -> &str {
    s.trim()
}

/// 检查字符串是否为空
pub fn is_empty(s: &str) -> bool {
    s.trim().is_empty()
}

/// 转换为蛇形命名 (snake_case)
pub fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    result
}