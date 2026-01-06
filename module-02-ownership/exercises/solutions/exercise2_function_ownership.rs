//! # 练习 2 解答: 函数与所有权
//!
//! **提示**：请先尝试自己实现，再参考此解答！
//!
//! # 实现思路
//!
//! 1. **传递给函数**：将值传递给函数时会移动所有权
//! 2. **返回所有权**：函数可以返回值，将所有权交还给调用者
//! 3. **元组技巧**：使用元组可以返回多个值
//! 4. **引用的优势**：使用引用可以避免获取所有权
//!
//! # 关键模式
//!
//! - Transfer → Return：转移后返回，恢复所有权
//! - Tuple Return：返回多个值
//! - Borrow：借用，不获取所有权

/// 获取字符串长度（转移所有权）
///
/// 这个函数获取 s 的所有权，s 在调用后不再有效
pub fn takes_ownership(s: String) -> usize {
    let length = s.len();
    length // s 在这里离开作用域并被 drop
}

/// 计算长度并返回字符串（恢复所有权）
///
/// # 实现思路
/// 接收所有权，处理后返回所有权，让调用者可以继续使用
pub fn gives_back(s: String) -> (String, usize) {
    let length = s.len();
    (s, length) // 返回字符串和长度
}

/// 使用引用计算长度（不获取所有权）
///
/// # 实现思路
/// 使用 `&String` 借用字符串，不获取所有权
/// 这是更常用的方式！
pub fn calculate_length(s: &String) -> usize {
    s.len()
} // s 离开作用域，但因为它没有所有权，所以不会 drop

/// 返回字符串的第一个单词和剩余部分
///
/// # 实现思路
/// 使用元组返回两个值，保留剩余部分的所有权
pub fn first_word_rest(s: String) -> (String, String) {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            let first = s[..i].to_string();
            let rest = s[i + 1..].to_string();
            return (first, rest);
        }
    }

    (s, String::new())
}

fn main() {
    let s = String::from("hello world");

    // 演示 1: 转移所有权
    println!("=== 演示 1: 转移所有权 ===");
    let len = takes_ownership(s.clone()); // 使用 clone 保留 s
    println!("长度: {}", len);

    // 演示 2: 返回所有权
    println!("\n=== 演示 2: 返回所有权 ===");
    let s2 = String::from("rust");
    let (s2, len) = gives_back(s2);
    println!("字符串: {}, 长度: {}", s2, len);

    // 演示 3: 使用引用（推荐！）
    println!("\n=== 演示 3: 使用引用 ===");
    let s3 = String::from("ownership");
    let len = calculate_length(&s3);
    println!("字符串: {}, 长度: {}", s3, len);
    // s3 仍然有效！

    // 演示 4: 多返回值
    println!("\n=== 演示 4: 多返回值 ===");
    let sentence = String::from("hello world rust");
    let (first, rest) = first_word_rest(sentence);
    println!("第一个单词: '{}', 剩余: '{}'", first, rest);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_takes_ownership() {
        let s = String::from("test");
        let len = takes_ownership(s.clone()); // clone 避免移动
        assert_eq!(len, 4);
        assert_eq!(s, "test"); // 因为 clone，s 仍然有效
    }

    #[test]
    fn test_gives_back() {
        let s = String::from("hello");
        let (s, len) = gives_back(s);
        assert_eq!(s, "hello");
        assert_eq!(len, 5);
    }

    #[test]
    fn test_calculate_length_borrow() {
        let s = String::from("borrow");
        let len = calculate_length(&s);
        assert_eq!(len, 6);
        assert_eq!(s, "borrow"); // s 仍然有效！
    }

    #[test]
    fn test_calculate_length_multiple_uses() {
        let s = String::from("rust");
        assert_eq!(calculate_length(&s), 4);
        assert_eq!(calculate_length(&s), 4);
        assert_eq!(s, "rust"); // 可以多次使用
    }

    #[test]
    fn test_first_word_rest() {
        let s = String::from("hello world");
        let (first, rest) = first_word_rest(s);
        assert_eq!(first, "hello");
        assert_eq!(rest, "world");
    }

    #[test]
    fn test_first_word_rest_no_space() {
        let s = String::from("single");
        let (first, rest) = first_word_rest(s);
        assert_eq!(first, "single");
        assert_eq!(rest, "");
    }

    #[test]
    fn test_multiple_borrows() {
        let s = String::from("test");
        let len1 = calculate_length(&s);
        let len2 = calculate_length(&s);
        let len3 = calculate_length(&s);
        assert_eq!(len1, len2);
        assert_eq!(len2, len3);
        assert_eq!(s, "test");
    }
}
