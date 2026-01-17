//! # 练习 3: Trait Bound
//!
//! 难度: 中等
//! 时间: 15 分钟
//! 前置知识: 泛型基础
//! 学习目标:
//!   - 使用 trait bound 约束泛型
//!   - 理解 where 子句

use std::fmt::Display;

// TODO: 使用 trait bound 使 T 必须实现 Display
pub fn notify<T: Display>(item: &T) {
    println!("Breaking news! {}", item);
}

// TODO: 为以下函数添加 trait bound
// T 必须实现 PartialOrd + Display
pub fn compare_and_print<T>(a: &T, b: &T) -> String
where
    T: PartialOrd + Display,
{
    if a > b {
        format!("{} is greater than {}", a, b)
    } else {
        format!("{} is not greater than {}", a, b)
    }
}

fn main() {
    notify(&"Hello");
    println!("{}", compare_and_print(&5, &3));
}
