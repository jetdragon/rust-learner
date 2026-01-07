//! # 练习 10: 高级生命周期场景 - 解答
//!
//! 难度: 困难
//! 时间: 20 分钟
//! 前置知识: 所有生命周期概念
//! 学习目标:
//!   - 处理复杂的生命周期场景
//!   - 理解生命周期对 API 设计的影响

use std::collections::HashMap;

pub struct Cache<'a> {
    data: HashMap<String, &'a str>,
}

impl<'a> Cache<'a> {
    pub fn new() -> Self {
        Cache {
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: &'a str) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&&'a str> {
        self.data.get(key)
    }
}

pub fn find_and_append<'a>(text: &'a str, pattern: &str, suffix: &str) -> String
where
    'a: 'static,
{
    if text.contains(pattern) {
        format!("{}{}", text, suffix)
    } else {
        text.to_string()
    }
}

pub struct Parser<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser {
            input,
            position: 0,
        }
    }

    pub fn current(&self) -> Option<&'a str> {
        if self.position < self.input.len() {
            Some(&self.input[self.position..])
        } else {
            None
        }
    }

    pub fn advance(&mut self, n: usize) {
        self.position += n;
        if self.position > self.input.len() {
            self.position = self.input.len();
        }
    }

    pub fn take_while<F>(&mut self, predicate: F) -> &'a str
    where
        F: Fn(char) -> bool,
    {
        let start = self.position;
        while let Some(ch) = self.input[self.position..].chars().next() {
            if predicate(ch) {
                self.position += ch.len_utf8();
            } else {
                break;
            }
        }
        &self.input[start..self.position]
    }
}

fn main() {
    let static_text = "Hello, World!";
    let result = find_and_append(static_text, "World", "!!!");
    println!("Result: {}", result);

    let input = "123abc456";
    let mut parser = Parser::new(input);

    let digits = parser.take_while(|c| c.is_ascii_digit());
    println!("Digits: {}", digits);

    let letters = parser.take_while(|c| c.is_ascii_alphabetic());
    println!("Letters: {}", letters);
}
