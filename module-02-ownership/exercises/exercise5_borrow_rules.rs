//! # 练习 5: 借用规则挑战
//!
//! **难度**: 困难
//! **预计时间**: 20 分钟
//!
//! **前置知识**:
//! - 可变和不可变引用
//! - 作用域概念
//!
//! **学习目标**:
//! - 深入理解 Rust 的借用规则
//! - 学会解决常见的借用冲突
//! - 掌握引用的作用域管理
//!
//! **借用规则**:
//! 1. 在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用
//! 2. 引用必须始终有效

/// 尝试同时使用多个引用
///
/// # TODO
/// 修复代码中的借用错误
pub fn multiple_refs_demo() -> String {
    let mut s = String::from("hello");

    // 创建不可变引用
    let r1 = &s;
    let r2 = &s;

    // TODO: 在这里尝试创建可变引用，需要处理作用域问题
    // let r3 = &mut s;  // 这会编译失败！为什么？

    format!("r1={}, r2={}", r1, r2)
}

/// 修复后的版本 - 正确使用多个引用
///
/// # TODO
/// 实现这个函数，展示如何正确使用多个引用
pub fn correct_multiple_refs() -> String {
    let mut s = String::from("hello");

    // 不可变引用作用域
    {
        let r1 = &s;
        let r2 = &s;
        println!("不可变引用: {} 和 {}", r1, r2);
    } // r1 和 r2 在这里离开作用域

    // TODO: 现在可以创建可变引用了
    let r3 = &mut s;
    unimplemented!()
}

/// 计算字符串长度并修改它
///
/// # TODO
/// 这个函数需要：
/// 1. 先获取字符串长度（使用不可变引用）
/// 2. 然后修改字符串（使用可变引用）
pub fn len_then_modify(s: &mut String, suffix: &str) -> usize {
    // TODO: 实现这个函数
    unimplemented!()
}

fn main() {
    println!("=== 借用规则演示 ===\n");

    // 演示 1: 多个不可变引用
    let result1 = multiple_refs_demo();
    println!("演示 1: {}", result1);

    // 演示 2: 正确的引用使用
    let result2 = correct_multiple_refs();
    println!("演示 2: {}", result2);

    // 演示 3: 长度计算后修改
    let mut text = String::from("rust");
    let len = len_then_modify(&mut text, " is awesome");
    println!("演示 3: 长度={}, 结果={}", len, text);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiple_refs_demo() {
        let result = multiple_refs_demo();
        assert!(result.contains("hello"));
    }

    #[test]
    fn test_correct_multiple_refs() {
        let result = correct_multiple_refs();
        assert!(result.contains("hello") || result.contains("HELLO"));
    }

    #[test]
    fn test_len_then_modify() {
        let mut s = String::from("test");
        let len = len_then_modify(&mut s, "ing");
        assert_eq!(s, "testing");
        assert_eq!(len, 4); // 原始长度
    }

    #[test]
    fn test_len_then_modify_empty() {
        let mut s = String::from("");
        let len = len_then_modify(&mut s, "hello");
        assert_eq!(s, "hello");
        assert_eq!(len, 0);
    }

    #[test]
    fn test_borrow_checker_prevents_data_race() {
        // 这个测试展示借用检查器如何防止数据竞争
        let mut s = String::from("safe");

        // 多个不可变引用 - OK
        let r1 = &s;
        let r2 = &s;
        assert_eq!(r1, r2);

        // 在使用完 r1 和 r2 后才能创建可变引用
        let r3 = &mut s;
        r3.push_str("ty");

        assert_eq!(s, "safety");
    }
}
