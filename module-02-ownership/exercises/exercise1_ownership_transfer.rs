//! 练习 1: 所有权转移
//!
//! # 目标
//! 理解 Rust 中所有权的转移机制
//!
//! # 任务
//! 修复代码中的所有权转移错误
//!
//! # 难度
//! 简单
//!
//! # 预计时间
//! 10 分钟
//!
//! # 前置知识
//! - 所有权的概念
//! - 移动语义
//! - String 类型的所有权

fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    // TODO: 修复下面的错误
    // println!("{}, world!", s1);

    // 提示：String 赋值时发生移动，s1 不再有效
    // 解决方案：
    // 1. 在赋值前使用 s1
    // 2. 使用 .clone() 克隆 s1
    // 3. 只使用 s2

    println!("{}, world!", s2);

    // 练习：取消注释并修复代码
    // println!("{}, world!", s1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ownership_move() {
        let s1 = String::from("hello");
        let s2 = s1;
        // s1 在这里已经无效，所有权转移给了 s2
        assert_eq!(s2, "hello");
    }

    #[test]
    fn test_clone_before_move() {
        let s1 = String::from("hello");
        let s2 = s1.clone();
        // clone 创建了副本，s1 仍然有效
        assert_eq!(s1, "hello");
        assert_eq!(s2, "hello");
    }

    #[test]
    fn test_clone_types() {
        // TODO: 完成测试 - 验证不同类型的拷贝行为
        let x = 5;
        let y = x;
        // i32 实现了 Copy trait，x 仍然有效
        assert_eq!(x, 5);
        assert_eq!(y, 5);
    }
}
