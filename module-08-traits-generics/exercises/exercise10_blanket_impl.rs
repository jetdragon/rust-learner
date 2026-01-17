//! # 练习 10: Blanket 实现
//!
//! 难度: 中等
//! 时间: 15 分钟
//! 前置知识: trait, 泛型
//! 学习目标:
//!   - 理解 blanket 实现的概念
//!   - 使用条件为满足 trait bound 的类型实现 trait

use std::fmt::Display;

// TODO: 定义一个 trait，为所有实现 Display 的类型提供 blanket 实现
pub trait Printable {
    fn print(&self) {
        println!("{}", self);
    }

    fn print_twice(&self) {
        println!("{}", self);
        println!("{}", self);
    }
}

// TODO: 为所有实现 Display 的类型实现 Printable
impl<T: Display> Printable for T {}

fn main() {
    let message = "Hello, World!";
    message.print();
    message.print_twice();

    let number = 42;
    number.print();
}
