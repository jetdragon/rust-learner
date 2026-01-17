//! # 练习 2: 结构体生命周期
//!
//! 难度: 简单
//! 时间: 10 分钟
//! 前置知识: 生命周期参数基础
//! 学习目标:
//!   - 在结构体定义中使用生命周期
//!   - 理解结构体中的引用有效性

// TODO: 添加生命周期参数
pub struct ImportantExcerpt<'a> {
    pub part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    pub fn level(&self) -> i32 {
        3
    }

    // TODO: 这个方法需要生命周期参数吗？为什么？
    pub fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };

    println!("Excerpt: {}", excerpt.part);
    println!("Level: {}", excerpt.level());
}
