//! # 练习 7: 泛型与生命周期结合
//!
//! 难度: 中等
//! 时间: 15 分钟
//! 前置知识: 泛型, 生命周期
//! 学习目标:
//!   - 在函数中同时使用泛型和生命周期
//!   - 理解参数的顺序（生命周期、泛型、约束）

use std::fmt::Display;

// TODO: 添加泛型和生命周期参数
pub fn announce_and_return<'a, T>(item: &'a T, announcement: &str) -> &'a T
where
    T: Display,
{
    println!("Announcement: {}", announcement);
    item
}

pub struct Ref<'a, T: ?Sized> {
    value: &'a T,
}

impl<'a, T> Ref<'a, T> {
    pub fn new(value: &'a T) -> Self {
        Ref { value }
    }

    pub fn get(&self) -> &'a T {
        self.value
    }
}

fn main() {
    let number = 42;
    let result = announce_and_return(&number, "Here's a number");
    println!("Result: {}", result);

    let text = "Hello";
    let text_ref = Ref::new(text);
    println!("Text: {}", text_ref.get());
}
