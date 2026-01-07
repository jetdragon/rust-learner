//! # 练习 9: Trait 中的生命周期 - 解答
//!
//! 难度: 中等
//! 时间: 15 分钟
//! 前置知识: trait, 生命周期
//! 学习目标:
//!   - 在 trait 定义中使用生命周期
//!   - 为包含生命周期的类型实现 trait

pub trait Borrow {
    fn borrow(&self) -> &str;
}

pub struct Book<'a> {
    title: &'a str,
    author: &'a str,
}

impl<'a> Book<'a> {
    pub fn new(title: &'a str, author: &'a str) -> Self {
        Book { title, author }
    }
}

impl<'a> Borrow for Book<'a> {
    fn borrow(&self) -> &str {
        self.title
    }
}

pub trait WithLifetime<'a> {
    fn get(&self) -> &'a str;
    fn set(&mut self, value: &'a str);
}

pub struct Holder<'a> {
    value: &'a str,
}

impl<'a> Holder<'a> {
    pub fn new(value: &'a str) -> Self {
        Holder { value }
    }
}

impl<'a> WithLifetime<'a> for Holder<'a> {
    fn get(&self) -> &'a str {
        self.value
    }

    fn set(&mut self, value: &'a str) {
        self.value = value;
    }
}

fn main() {
    let title = String::from("The Rust Book");
    let author = String::from("Steve Klabnik");

    let book = Book::new(&title, &author);
    println!("Borrowing: {}", book.borrow());

    let text = String::from("Hello");
    let mut holder = Holder::new(&text);
    println!("Holder: {}", holder.get());

    let new_text = String::from("World");
    holder.set(&new_text);
    println!("Holder: {}", holder.get());
}
