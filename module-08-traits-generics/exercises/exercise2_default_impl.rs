//! # 练习 2: Trait 默认实现
//!
//! 难度: 简单
//! 时间: 10 分钟
//! 前置知识: trait 基础
//! 学习目标:
//!   - 为 trait 方法提供默认实现
//!   - 理解默认实现可以被覆盖

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }

    fn summarize_authored(&self) -> String {
        format!("(Read more from {}...)", self.get_author())
    }

    fn get_author(&self) -> String;
}

// TODO: 实现 Summary trait，只提供 get_author 方法
// 使用 summarize 的默认实现

pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}

fn main() {
    let post = Post {
        title: String::from("Rust Traits"),
        author: String::from("Jane Doe"),
        content: String::from("Traits are awesome..."),
    };

    println!("New post: {}", post.summarize());
}
