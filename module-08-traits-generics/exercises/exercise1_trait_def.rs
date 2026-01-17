//! # 练习 1: Trait 定义
//!
//! 难度: 简单
//! 时间: 10 分钟
//! 前置知识: struct, impl 块
//! 学习目标:
//!   - 定义自定义 trait
//!   - 为类型实现 trait
//!   - 理解 trait bound 的基本概念

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// TODO: 为 NewsArticle 实现 Summary trait
// 提示: 返回格式如 "[headline] by [author] in [location]"

// TODO: 为 Tweet 实现 Summary trait
// 提示: 返回格式如 "[username]: [content]"

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
