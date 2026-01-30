//! # 练习 9: Newtype 模式
//!
//! 难度: 中等
//! 时间: 15 分钟
//! 前置知识: tuple struct, trait
//! 学习目标:
//!   - 使用 newtype 模式实现外部 trait
//!   - 理解孤儿规则的限制

use std::fmt;

// 假设这是外部库中的类型，我们无法为其实现 Display
pub struct Wrapper(Vec<String>);

// TODO: 为 Wrapper 实现 Display trait
// 格式: [item1, item2, item3]
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// TODO: 为 Wrapper 实现 Debug trait
impl fmt::Debug for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Wrapper({:?})", self.0)
    }
}

fn main() {
    let w = Wrapper(vec![
        String::from("hello"),
        String::from("world"),
        String::from("rust"),
    ]);
    println!("Display: {}", w);
    println!("Debug: {:?}", w);
}
