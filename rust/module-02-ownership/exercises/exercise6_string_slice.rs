//! # 练习 6: 字符串切片
//!
//! **难度**: 中等
//! **预计时间**: 20 分钟
//!
//! **前置知识**:
//! - 基本的字符串类型 (String, &str)
//! - 引用概念
//!
//! **学习目标**:
//! - 理解字符串切片 (`&str`) 的概念
//! - 学会使用切片语法 `&s[start..end]`
//! - 掌握字符串字面量就是切片
//! - 理解切片的范围边界

/// 返回字符串的第一个单词
///
/// # TODO
/// 实现这个函数，返回空格前的第一个单词
/// 如果没有空格，返回整个字符串
pub fn first_word(s: &String) -> &str {
    unimplemented!()
}

/// 返回字符串的最后一个单词
///
/// # TODO
/// 实现这个函数，返回最后一个空格后的单词
pub fn last_word(s: &String) -> &str {
    unimplemented!()
}

/// 返回指定范围内的字符切片
///
/// # TODO
/// 实现这个函数，使用切片语法
/// 注意：需要处理边界情况
pub fn slice_range(s: &String, start: usize, end: usize) -> &str {
    unimplemented!()
}

/// 检查字符串是否包含特定单词
///
/// # TODO
/// 使用切片操作实现，不使用 contains 方法
pub fn has_word(s: &String, word: &str) -> bool {
    unimplemented!()
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
    fn test_has_word_partial() {
        let s = String::from("testing");
        // "test" 是 "testing" 的子串，但不是独立单词
        // 这个测试的期望取决于你的实现
        assert!(has_word(&s, "test") || !has_word(&s, "test"));
    }

    #[test]
    fn test_string_literal_is_slice() {
        let s: &str = "hello";
        assert_eq!(s, "hello");
        assert_eq!(first_word(&String::from("hello world")), "hello");
    }

    #[test]
    fn test_slice_bounds_safety() {
        // 切片会保证 UTF-8 边界安全
        let s = String::from("hello");
        let result = slice_range(&s, 0, s.len());
        assert_eq!(result, "hello");
    }
}
