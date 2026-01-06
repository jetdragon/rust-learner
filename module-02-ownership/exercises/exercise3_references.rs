//! 练习 3: 引用与借用
//!
//! # 目标
//! 学习使用引用（&）来借用数据而不获取所有权
//!
//! # 任务
//! 实现函数使用引用计算字符串长度
//!
//! # 难度
//! 中等
//!
//! # 预计时间
//! 15 分钟
//!
//! # 前置知识
//! - 引用（&）的概念
//! - 借用（borrowing）
//! - .len() 方法

/// 计算字符串的长度（不获取所有权）
///
/// # 参数
/// * `s` - 字符串的引用
///
/// # 返回值
/// 字符串的长度
///
/// # 示例
/// ```rust
/// use module_02_ownership::exercises::calculate_length;
///
/// let s = String::from("hello");
/// let len = calculate_length(&s);
/// assert_eq!(len, 5);
/// assert_eq!(s, "hello"); // s 仍然有效
/// ```
pub fn calculate_length(s: &String) -> usize {
    // TODO: 实现此函数
    // 提示：使用 .len() 方法
    unimplemented!()
}

/// 打印字符串及其长度
///
/// # 参数
/// * `s` - 字符串的引用
pub fn print_length(s: &String) {
    let len = calculate_length(s);
    println!("字符串 '{}' 的长度是 {}", s, len);
}

/// 获取字符串的第一个字符（可选挑战）
///
/// # 参数
/// * `s` - 字符串的引用
///
/// # 返回值
/// 第一个字符的引用，如果字符串为空返回 None
pub fn first_char(s: &String) -> Option<&str> {
    // TODO (可选): 实现获取第一个字符
    // 提示：使用 .chars().next()
    unimplemented!()
}

fn main() {
    let s = String::from("hello world");

    // 使用引用计算长度
    let len = calculate_length(&s);
    println!("长度: {}", len);

    // s 仍然有效，可以继续使用
    println!("字符串: {}", s);

    // 打印长度
    print_length(&s);

    // 再次证明 s 仍然有效
    println!("字符串仍然可用: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_length() {
        let s = String::from("hello");
        assert_eq!(calculate_length(&s), 5);
    }

    #[test]
    fn test_calculate_length_empty() {
        let s = String::from("");
        assert_eq!(calculate_length(&s), 0);
    }

    #[test]
    fn test_calculate_length_unicode() {
        // Rust 的 String 是 UTF-8 编码
        let s = String::from("你好");
        // 中文字符占用 3 个字节，但 len() 返回字节数
        assert_eq!(calculate_length(&s), 6);
    }

    #[test]
    fn test_string_still_valid() {
        let s = String::from("test");
        calculate_length(&s);
        // s 仍然有效，因为我们只借用了它
        assert_eq!(s, "test");
    }

    #[test]
    fn test_first_char_some() {
        let s = String::from("hello");
        let first = first_char(&s);
        assert_eq!(first, Some("h"));
    }

    #[test]
    fn test_first_char_none() {
        let s = String::from("");
        let first = first_char(&s);
        assert_eq!(first, None);
    }
}
