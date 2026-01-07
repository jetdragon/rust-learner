//! # 练习 5: impl Trait
//!
//! 难度: 中等
//! 时间: 15 分钟
//! 前置知识: trait, 泛型
//! 学习目标:
//!   - 使用 impl Trait 作为参数和返回类型
//!   - 理解 impl Trait 的限制

use std::fmt::Display;

// TODO: 使用 impl Trait 简化参数类型
pub fn notify(item: &impl Display) {
    println!("Breaking news! {}", item);
}

// TODO: 使用 impl Trait 作为返回类型
pub fn create_displayable() -> impl Display {
    String::from("Hello, World!")
}

// TODO: 这个函数能使用 impl Trait 吗？为什么？
// pub fn get_conditional(condition: bool) -> impl Display {
//     if condition {
//         String::from("yes")
//     } else {
//         42
//     }
// }

fn main() {
    notify(&create_displayable());
}
