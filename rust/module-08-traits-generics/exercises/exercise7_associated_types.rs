//! # 练习 7: 关联类型
//!
//! 难度: 中等
//! 时间: 15 分钟
//! 前置知识: trait, 泛型
//! 学习目标:
//!   - 在 trait 中定义关联类型
//!   - 理解关联类型 vs 泛型参数的区别

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

pub struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { count: 0 }
    }
}

// TODO: 为 Counter 实现 Iterator trait
// 关联类型 Item 应该是 u32
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            let result = Some(self.count);
            self.count += 1;
            result
        } else {
            None
        }
    }
}

pub struct CharCounter {
    chars: Vec<char>,
    index: usize,
}

impl CharCounter {
    pub fn new(s: &str) -> CharCounter {
        CharCounter {
            chars: s.chars().collect(),
            index: 0,
        }
    }
}

// TODO: 为 CharCounter 实现 Iterator trait
// 关联类型 Item 应该是 char
impl Iterator for CharCounter {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.chars.len() {
            let result = Some(self.chars[self.index]);
            self.index += 1;
            result
        } else {
            None
        }
    }
}

fn main() {
    let mut counter = Counter::new();
    if let Some(v) = counter.next() {
        println!("Counter value: {}", v);
    }

    let mut char_counter = CharCounter::new("hello");
    if let Some(c) = char_counter.next() {
        println!("Char: {}", c);
    }
}
