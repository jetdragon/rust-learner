//! # 练习 6 解答: 字符串切片
//!
//! **提示**：请先尝试自己实现，再参考此解答！
//!
//! # 实现思路
//!
//! 1. **字符串切片类型**：`&str` 是对字符串部分的不可变引用
//! 2. **切片语法**：`&s[start..end]` 创建从 start 到 end（不含）的切片
//! 3. **范围简写**：
//!    - `..end` 等价于 `0..end`
//!    - `start..` 等价于 `start..len`
//!    - `..` 等价于整个范围
//!
//! 4. **字符串字面量**：`"hello"` 的类型就是 `&str`
//!
//! # 关键概念
//!
//! - **切片不获取所有权**：只是借用字符串的一部分
//! - **UTF-8 安全**：切片必须在有效的 UTF-8 边界上
//! - **零拷贝**：切片操作不需要复制数据

/// 返回字符串的第一个单词
///
/// # 实现思路
/// 1. 使用 bytes().iter().enumerate() 遍历字节
/// 2. 找到第一个空格的位置
/// 3. 返回从开始到空格位置的切片
pub fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

/// 返回字符串的最后一个单词
///
/// # 实现思路
/// 1. 使用 rfind() 找到最后一个空格
/// 2. 返回从空格后到结尾的切片
pub fn last_word(s: &String) -> &str {
    if let Some(pos) = s.rfind(' ') {
        &s[pos + 1..]
    } else {
        &s[..]
    }
}

/// 返回指定范围内的字符切片
///
/// # 实现思路
/// 1. 使用切片语法 `&s[start..end]`
/// 2. 边界由调用者保证正确
pub fn slice_range(s: &String, start: usize, end: usize) -> &str {
    &s[start..end]
}

/// 检查字符串是否包含特定单词
///
/// # 实现思路
/// 使用 split_whitespace() 分割成单词，然后检查
pub fn has_word(s: &String, word: &str) -> bool {
    s.split_whitespace().any(|w| w == word)
}

/// 更灵活的版本：直接接受 &str 参数
pub fn first_word_str(s: &str) -> &str {
    match s.find(' ') {
        Some(pos) => &s[..pos],
        None => s,
    }
}

/// 获取字符串的前 n 个字符
pub fn first_n_chars(s: &String, n: usize) -> &str {
    let end = n.min(s.len());
    &s[..end]
}

/// 去除字符串首尾空格
pub fn trim_string(s: &String) -> &str {
    s.trim()
}

fn main() {
    let s = String::from("hello world rust");

    println!("=== 字符串切片演示 ===\n");
    println!("原字符串: {}", s);

    // 第一个单词
    let first = first_word(&s);
    println!("第一个单词: {}", first);

    // 最后一个单词
    let last = last_word(&s);
    println!("最后一个单词: {}", last);

    // 范围切片
    let range = slice_range(&s, 6, 11);
    println!("范围 [6..11]: {}", range);

    // 检查单词
    let has_rust = has_word(&s, "rust");
    println!("包含 'rust': {}", has_rust);

    // 演示字符串字面量也是切片
    let literal: &str = "hello";
    println!("\n字符串字面量类型: &str");
    println!("字面值: {}", literal);

    // 演示切片与所有权的优势
    println!("\n=== 切片的优势 ===");
    let sentence = String::from("The quick brown fox");
    let word = first_word(&sentence);
    println!("句子: {}", sentence);
    println!("第一个单词: {}", word);
    println!("注意：仍然可以使用 sentence，因为只是借用！");

    // 演示切片语法简写
    println!("\n=== 切片语法简写 ===");
    let s = String::from("hello");
    println!("完整: {}", &s[0..5]);
    println!("从0: {}", &s[..5]);
    println!("到结尾: {}", &s[2..]);
    println!("全部: {}", &s[..]);

    // 演示 &str 参数的灵活性
    println!("\n=== &str 参数的灵活性 ===");
    let string = String::from("hello world");
    let literal = "hello world";
    println!("String: {}", first_word_str(&string));
    println!("&str: {}", first_word_str(literal));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_word() {
        let s = String::from("hello world");
        assert_eq!(first_word(&s), "hello");
    }

    #[test]
    fn test_first_word_no_space() {
        let s = String::from("hello");
        assert_eq!(first_word(&s), "hello");
    }

    #[test]
    fn test_first_word_multiple_spaces() {
        let s = String::from("one two three");
        assert_eq!(first_word(&s), "one");
    }

    #[test]
    fn test_last_word() {
        let s = String::from("hello world");
        assert_eq!(last_word(&s), "world");
    }

    #[test]
    fn test_last_word_three_words() {
        let s = String::from("one two three");
        assert_eq!(last_word(&s), "three");
    }

    #[test]
    fn test_last_word_no_space() {
        let s = String::from("single");
        assert_eq!(last_word(&s), "single");
    }

    #[test]
    fn test_slice_range() {
        let s = String::from("hello world");
        assert_eq!(slice_range(&s, 0, 5), "hello");
        assert_eq!(slice_range(&s, 6, 11), "world");
    }

    #[test]
    fn test_slice_range_full() {
        let s = String::from("hello");
        assert_eq!(slice_range(&s, 0, 5), "hello");
    }

    #[test]
    fn test_has_word() {
        let s = String::from("hello world rust");
        assert!(has_word(&s, "world"));
        assert!(has_word(&s, "hello"));
        assert!(has_word(&s, "rust"));
        assert!(!has_word(&s, "python"));
    }

    #[test]
    fn test_has_word_case_sensitive() {
        let s = String::from("Hello World");
        assert!(has_word(&s, "Hello"));
        assert!(!has_word(&s, "hello"));
    }

    #[test]
    fn test_string_literal_is_slice() {
        let s: &str = "hello";
        assert_eq!(s, "hello");
        assert_eq!(first_word(&String::from("hello world")), "hello");
    }

    #[test]
    fn test_slice_bounds_safety() {
        let s = String::from("hello");
        let result = slice_range(&s, 0, s.len());
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_first_word_str_with_string() {
        let s = String::from("test example");
        assert_eq!(first_word_str(&s), "test");
    }

    #[test]
    fn test_first_word_str_with_literal() {
        assert_eq!(first_word_str("hello world"), "hello");
    }

    #[test]
    fn test_first_n_chars() {
        let s = String::from("hello");
        assert_eq!(first_n_chars(&s, 2), "he");
        assert_eq!(first_n_chars(&s, 10), "hello"); // 不超过长度
    }

    #[test]
    fn test_trim_string() {
        let s = String::from("  hello  ");
        assert_eq!(trim_string(&s), "hello");
    }

    #[test]
    fn test_slice_does_not_take_ownership() {
        let s = String::from("hello world");
        let first = first_word(&s);
        assert_eq!(first, "hello");
        assert_eq!(s, "hello world"); // s 仍然有效
    }

    #[test]
    fn test_multiple_slices() {
        let s = String::from("hello world test");
        let first = first_word(&s);
        let last = last_word(&s);
        // 可以同时存在多个切片（都是不可变引用）
        assert_eq!(first, "hello");
        assert_eq!(last, "test");
    }

    #[test]
    fn test_empty_string() {
        let s = String::from("");
        assert_eq!(first_word(&s), "");
        assert_eq!(last_word(&s), "");
        assert_eq!(first_n_chars(&s, 5), "");
    }

    #[test]
    fn test_range_syntax() {
        let s = String::from("rust");
        assert_eq!(&s[0..4], "rust");
        assert_eq!(&s[..4], "rust");
        assert_eq!(&s[0..], "rust");
        assert_eq!(&s[..], "rust");
        assert_eq!(&s[2..4], "st");
        assert_eq!(&s[2..], "st");
    }

    #[test]
    fn test_slice_lifetime() {
        // 演示切片的生命周期与借用
        let s = String::from("hello world");

        {
            let first = first_word(&s);
            assert_eq!(first, "hello");
            // first 在这里仍然有效
        } // first 离开作用域

        // s 仍然有效
        assert_eq!(s.len(), 11);
    }
}
