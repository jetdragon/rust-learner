//! # Trait 与泛型
//!
//! 本模块演示 Rust 的 Trait 和泛型系统。
//!
//! ## 模块内容
//!
//! - Trait 定义和实现
//! - 泛型函数和类型
//! - Trait Bound
//! - 生命周期标注
//! - 运算符重载
//!
//! ## 示例
//!
//! ```
//! use module_07_traits_generics::{largest, Pair, Summary};
//!
//! let pair = Pair::new(1, 2);
//! assert_eq!(pair.first(), &1);
//!
//! let numbers = vec![1, 2, 3, 4, 5];
//! assert_eq!(largest(&numbers), Some(&5));
//! ```

use std::fmt::{Debug, Display};
use std::ops::Add;

// ========== Trait 定义 ==========

/// 摘要 Trait - 演示默认实现
pub trait Summary {
    /// 返回摘要
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }

    /// 返回作者
    fn author(&self) -> &str {
        "Unknown"
    }
}

/// 新闻文章
#[derive(Debug, Clone)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn author(&self) -> &str {
        &self.author
    }
}

/// 推文
#[derive(Debug, Clone)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn author(&self) -> &str {
        &self.username
    }
}

/// 通知带有 Trait Bound
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

/// 使用完整 Trait Bound 语法的通知
pub fn notify_explicit<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

/// 使用 where 子句的通知
pub fn notify_where<T>(item: &T)
where
    T: Summary,
{
    println!("Breaking news! {}", item.summarize());
}

/// 返回实现了 Summary 的类型
pub fn return_summarizable() -> impl Summary {
    NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    }
}

// ========== 泛型结构体 ==========

/// 泛型 Pair 结构体
#[derive(Debug, Clone)]
pub struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    pub fn new(first: T, second: T) -> Self {
        Self { first, second }
    }

    pub fn first(&self) -> &T {
        &self.first
    }

    pub fn second(&self) -> &T {
        &self.second
    }

    /// 交换两个值
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.first, &mut self.second);
    }
}

/// 为实现 Display 的类型提供打印方法
impl<T: Display> Pair<T> {
    pub fn print(&self) {
        println!("Pair: ({}, {})", self.first, self.second);
    }
}

/// 为实现 PartialOrd + Display 的类型提供比较方法
impl<T: PartialOrd + Display> Pair<T> {
    pub fn cmp_display(&self) {
        if self.first >= self.second {
            println!("The largest member is {}", self.first);
        } else {
            println!("The largest member is {}", self.second);
        }
    }
}

// ========== 泛型函数 ==========

/// 找出列表中最大的元素
pub fn largest<T: PartialOrd>(list: &[T]) -> Option<&T> {
    if list.is_empty() {
        return None;
    }
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    Some(largest)
}

/// 找出两个值中较大的
pub fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

// ========== 生命周期 ==========

/// 返回两个字符串切片中较长的一个
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/// 带生命周期的重要摘录结构体
#[derive(Debug)]
pub struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    pub fn new(part: &'a str) -> Self {
        Self { part }
    }

    pub fn part(&self) -> &str {
        self.part
    }

    pub fn level(&self) -> i32 {
        3
    }

    /// 生命周期省略规则第三条的示例
    pub fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// ========== 运算符重载 ==========

/// 二维点
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/// Point + i32
impl Add<i32> for Point {
    type Output = Self;

    fn add(self, other: i32) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

// ========== Iterator 示例 ==========

/// 计数器结构体
pub struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            let current = self.count;
            self.count += 1;
            Some(current)
        } else {
            None
        }
    }
}

// ========== Clone 和 Copy 示例 ==========

/// 可复制的消息
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct CopyableMessage {
    id: u32,
}

impl CopyableMessage {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}

/// 需要显式克隆的消息
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ClonableMessage {
    id: u32,
    data: String,
}

impl ClonableMessage {
    pub fn new(id: u32, data: String) -> Self {
        Self { id, data }
    }
}

// ========== PartialEq 和 Eq 示例 ==========

/// 可比较的值
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EquatableValue {
    Number(i32),
    Text(String),
}

// ========== From 和 Into 示例 ==========

/// 摄氏度
#[derive(Debug, Clone, Copy)]
pub struct Celsius(pub f64);

/// 华氏度
#[derive(Debug, Clone, Copy)]
pub struct Fahrenheit(pub f64);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

impl From<Fahrenheit> for Celsius {
    fn from(f: Fahrenheit) -> Self {
        Celsius((f.0 - 32.0) * 5.0 / 9.0)
    }
}

// ========== Default 示例 ==========

/// 配置结构体
#[derive(Debug, Clone)]
pub struct Config {
    pub timeout: u32,
    pub max_retries: u32,
    pub debug: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            timeout: 30,
            max_retries: 3,
            debug: false,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_timeout(mut self, timeout: u32) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    pub fn with_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }
}

// ========== Trait 对象示例 ==========

/// 动物 Trait
pub trait Animal {
    fn make_sound(&self) -> &str;
    fn name(&self) -> &str;
}

/// 狗
#[derive(Debug)]
pub struct Dog {
    name: String,
}

impl Dog {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Animal for Dog {
    fn make_sound(&self) -> &str {
        "Woof!"
    }

    fn name(&self) -> &str {
        &self.name
    }
}

/// 猫
#[derive(Debug)]
pub struct Cat {
    name: String,
}

impl Cat {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Animal for Cat {
    fn make_sound(&self) -> &str {
        "Meow!"
    }

    fn name(&self) -> &str {
        &self.name
    }
}

/// 让所有动物发声
pub fn make_all_animals_speak(animals: &[Box<dyn Animal>]) {
    for animal in animals {
        println!("{} says {}", animal.name(), animal.make_sound());
    }
}

// ========== 测试 ==========

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summary() {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };

        assert_eq!(
            tweet.summarize(),
            "horse_ebooks: of course, as you probably already know, people"
        );
    }

    #[test]
    fn test_notify() {
        let article = NewsArticle {
            headline: String::from("Penguins win!"),
            location: String::from("Pittsburgh"),
            author: String::from("Iceburgh"),
            content: String::from("The Penguins are champions!"),
        };

        notify(&article);
        notify_explicit(&article);
        notify_where(&article);
    }

    #[test]
    fn test_largest() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(largest(&numbers), Some(&5));

        let empty: Vec<i32> = vec![];
        assert!(largest(&empty).is_none());
    }

    #[test]
    fn test_pair() {
        let mut pair = Pair::new(1, 2);
        assert_eq!(pair.first(), &1);
        assert_eq!(pair.second(), &2);

        pair.swap();
        assert_eq!(pair.first(), &2);
        assert_eq!(pair.second(), &1);
    }

    #[test]
    fn test_longest() {
        let string1 = String::from("long string is long");
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        assert_eq!(result, "long string is long");
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(1, 2);
        let p2 = Point::new(3, 4);
        let sum = p1 + p2;
        assert_eq!(sum, Point::new(4, 6));

        let p3 = Point::new(1, 2);
        let sum2 = p3 + 5;
        assert_eq!(sum2, Point::new(6, 7));
    }

    #[test]
    fn test_counter() {
        let sum: u32 = Counter::new().sum();
        assert_eq!(sum, 10); // 0 + 1 + 2 + 3 + 4 = 10
    }

    #[test]
    fn test_clone_copy() {
        let msg1 = CopyableMessage::new(1);
        let msg2 = msg1; // Copy
        assert_eq!(msg1.id, msg2.id);

        let msg3 = ClonableMessage::new(1, String::from("test"));
        let msg4 = msg3.clone(); // Clone
        assert_eq!(msg3.id, msg4.id);
    }

    #[test]
    fn test_from_into() {
        let c = Celsius(0.0);
        let f: Fahrenheit = c.into();
        assert!((f.0 - 32.0).abs() < 0.01);

        let f2 = Fahrenheit(32.0);
        let c2: Celsius = f2.into();
        assert!((c2.0 - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_default() {
        let config = Config::new();
        assert_eq!(config.timeout, 30);

        let config2 = Config::new().with_timeout(60).with_debug(true);
        assert_eq!(config2.timeout, 60);
        assert!(config2.debug);
    }

    #[test]
    fn test_animal_trait_object() {
        let animals: Vec<Box<dyn Animal>> = vec![
            Box::new(Dog::new(String::from("Buddy"))),
            Box::new(Cat::new(String::from("Whiskers"))),
        ];

        make_all_animals_speak(&animals);
    }
}
