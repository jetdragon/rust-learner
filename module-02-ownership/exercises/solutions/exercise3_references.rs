//! # 练习 3 解答: 引用与借用
//!
//! **提示**：请先尝试自己实现，再参考此解答！
//!
//! # 实现思路
//!
//! 1. **引用语法**：`&T` 创建类型 T 的引用
//! 2. **借用**：创建引用的行为称为"借用"
//! 3. **不可变引用**：默认的引用是不可变的，不能修改数据
//! 4. **解引用**：使用 `*` 操作符解引用（获取引用指向的值）
//!
//! # 关键概念
//!
//! - **借用 vs 拥有**：借用不获取所有权，借用结束后原值仍然有效
//! - **引用有效性**：引用必须始终指向有效的值
//! - **不可变借用**：`&T` 创建共享引用，可以同时存在多个

/// 计算字符串的长度
///
/// # 实现思路
/// 使用 `&String` 借用字符串，不获取所有权
pub fn calculate_length(s: &String) -> usize {
    s.len()
} // s 离开作用域，但因为它不拥有数据，所以不会 drop

/// 检查字符串是否为空
pub fn is_empty(s: &String) -> bool {
    s.is_empty()
}

/// 获取字符串的第一个字符（如果存在）
///
/// # 实现思路
/// 使用 chars() 和 next() 获取第一个字符
/// 注意：这需要处理 Unicode
pub fn first_char(s: &String) -> Option<char> {
    s.chars().next()
}

/// 比较两个字符串是否相等
pub fn string_equals(s1: &String, s2: &String) -> bool {
    s1 == s2
}

/// 计算两个字符串的总长度
pub fn combined_length(s1: &String, s2: &String) -> usize {
    s1.len() + s2.len()
}

/// 创建字符串的大写版本（不修改原字符串）
pub fn to_uppercase_copy(s: &String) -> String {
    s.to_uppercase()
}

fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("world");

    println!("=== 引用与借用演示 ===\n");

    // 演示 1: 基本引用
    println!("演示 1: 基本引用");
    let len = calculate_length(&s1);
    println!("字符串 '{}', 长度: {}", s1, len);
    println!("s1 仍然有效: '{}'", s1);

    // 演示 2: 多个引用
    println!("\n演示 2: 多个引用");
    let len1 = calculate_length(&s1);
    let len2 = calculate_length(&s1);
    println!("第一次计算: {}, 第二次计算: {}", len1, len2);

    // 演示 3: 多个参数引用
    println!("\n演示 3: 多个参数引用");
    let total = combined_length(&s1, &s2);
    println!("'{}' + '{}' 的总长度: {}", s1, s2, total);

    // 演示 4: 链式借用
    println!("\n演示 4: 链式借用");
    if !is_empty(&s1) {
        let first = first_char(&s1);
        println!("第一个字符: {:?}", first);
    }

    // 演示 5: 不修改原字符串
    println!("\n演示 5: 创建新字符串");
    let original = String::from("rust");
    let upper = to_uppercase_copy(&original);
    println!("原始: '{}', 大写: '{}'", original, upper);

    // 演示 6: 引用作用域
    println!("\n演示 6: 引用作用域");
    {
        let s = String::from("scoped");
        let r = &s;
        println!("借用: {}", r);
    } // r 在这里离开作用域，s 被 drop
    // println!("{}", r);  // 错误！r 不再有效
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_length() {
        let s = String::from("hello");
        assert_eq!(calculate_length(&s), 5);
        assert_eq!(s, "hello"); // s 仍然有效
    }

    #[test]
    fn test_calculate_length_empty() {
        let s = String::from("");
        assert_eq!(calculate_length(&s), 0);
    }

    #[test]
    fn test_is_empty() {
        assert!(is_empty(&String::from("")));
        assert!(!is_empty(&String::from("test")));
    }

    #[test]
    fn test_first_char() {
        assert_eq!(first_char(&String::from("hello")), Some('h'));
        assert_eq!(first_char(&String::from("")), None);
        assert_eq!(first_char(&String::from("你好")), Some('你'));
    }

    #[test]
    fn test_string_equals() {
        let s1 = String::from("test");
        let s2 = String::from("test");
        let s3 = String::from("other");
        assert!(string_equals(&s1, &s2));
        assert!(!string_equals(&s1, &s3));
    }

    #[test]
    fn test_combined_length() {
        let s1 = String::from("hello");
        let s2 = String::from("world");
        assert_eq!(combined_length(&s1, &s2), 10);
    }

    #[test]
    fn test_to_uppercase_copy() {
        let original = String::from("rust");
        let upper = to_uppercase_copy(&original);
        assert_eq!(upper, "RUST");
        assert_eq!(original, "rust"); // 原字符串未修改
    }

    #[test]
    fn test_multiple_borrows() {
        let s = String::from("test");
        let r1 = &s;
        let r2 = &s;
        let r3 = &s;
        // 可以同时存在多个不可变引用
        assert_eq!(r1, r2);
        assert_eq!(r2, r3);
    }

    #[test]
    fn test_borrow_after_use() {
        let s = String::from("rust");
        let len = calculate_length(&s);
        let is_emp = is_empty(&s);
        let first = first_char(&s);
        // s 可以被多次借用
        assert_eq!(len, 4);
        assert!(!is_emp);
        assert_eq!(first, Some('r'));
    }

    #[test]
    fn test_reference_scope() {
        let s = String::from("scope");
        {
            let r = &s;
            assert_eq!(r, "scope");
        } // r 离开作用域
        assert_eq!(s, "scope"); // s 仍然有效
    }
}
