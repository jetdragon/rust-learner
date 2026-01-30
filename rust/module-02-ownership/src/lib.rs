//! # 所有权系统 (Ownership System)
//!
//! 本模块演示 Rust 的所有权系统，包括：
//! - 所有权规则和移动语义
//! - 引用和借用
//! - 切片类型
//!
//! ## 示例
//!
//! ```
//! use module_02_ownership::{calculate_length, first_word};
//!
//! let s = String::from("hello world");
//! let len = calculate_length(&s);
//! println!("'{}' 的长度是 {}", s, len);
//!
//! let word = first_word(&s);
//! println!("第一个单词: {}", word);
//! ```

/// 计算字符串长度（不获取所有权）
///
/// # 参数
///
/// * `s: &String` - 字符串的引用
///
/// # 返回
///
/// * `usize` - 字符串长度
///
/// # 示例
///
/// ```
/// use module_02_ownership::calculate_length;
///
/// let s = String::from("hello");
/// let len = calculate_length(&s);
/// assert_eq!(len, 5);
/// assert_eq!(s, "hello");  // s 仍然有效，因为只借用了它
/// ```
pub fn calculate_length(s: &String) -> usize {
    s.len()
}

/// 获取字符串的第一个单词
///
/// 返回字符串切片，不拷贝数据
///
/// # 参数
///
/// * `s: &String` - 字符串引用
///
/// # 返回
///
/// * `&str` - 第一个单词的切片
///
/// # 示例
///
/// ```
/// use module_02_ownership::first_word;
///
/// let s = String::from("hello world");
/// let word = first_word(&s);
/// assert_eq!(word, "hello");
/// ```
pub fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

/// 获取字符串的第一个单词（改进版，接受 &str）
///
/// # 参数
///
/// * `s: &str` - 字符串切片
///
/// # 返回
///
/// * `&str` - 第一个单词的切片
pub fn first_word_improved(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

/// 演示所有权转移
///
/// # 参数
///
/// * `s: String` - 获取 String 的所有权
pub fn take_ownership(s: String) {
    println!("{}", s);
} // s 超出作用域，被丢弃

/// 演示拷贝语义
///
/// # 参数
///
/// * `x: i32` - i32 实现了 Copy，会拷贝而非移动
pub fn makes_copy(x: i32) {
    println!("{}", x);
} // x 超出作用域，但因为是 Copy，无特殊处理

/// 返回 String 的所有权
///
/// # 返回
///
/// * `String` - 创建的字符串
pub fn gives_ownership() -> String {
    String::from("yours")
}

/// 接收 String 并返回修改后的 String
///
/// # 参数
///
/// * `mut s: String` - 可变 String
///
/// # 返回
///
/// * `String` - 修改后的 String
pub fn takes_and_gives_back(mut s: String) -> String {
    s.push_str(", world");
    s
}

/// 使用可变引用修改字符串
///
/// # 参数
///
/// * `s: &mut String` - 可变字符串引用
pub fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/// 演示悬垂引用问题
///
/// 注意：这是教学示例，实际使用中应该返回 String 而非 &String。
/// 如果尝试返回指向局部变量的引用，编译器会报错。
///
/// ```compile_fail
/// // 以下代码无法编译：
/// fn dangle() -> &String {
///     let s = String::from("hello");
///     &s  // 错误：返回指向局部变量的引用
/// }
/// ```
#[allow(dead_code)]
pub fn dangle_example() -> String {
    // 正确做法：返回 String
    let s = String::from("hello");
    s // 转移所有权而非返回引用
}

/// 正确的做法：直接返回 String
pub fn no_dangle() -> String {
    let s = String::from("hello");
    s // s 的所有权转移给调用者
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_length() {
        let s = String::from("hello");
        let len = calculate_length(&s);
        assert_eq!(len, 5);
        assert_eq!(s, "hello"); // 验证 s 仍然有效
    }

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
    fn test_first_word_improved() {
        let s = "hello world";
        assert_eq!(first_word_improved(s), "hello");
    }

    #[test]
    fn test_no_dangle() {
        let s = no_dangle();
        assert_eq!(s, "hello");
    }
}
