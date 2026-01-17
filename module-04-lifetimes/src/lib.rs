//! # 生命周期与高级类型
//!
//! 本模块演示 Rust 的生命周期系统和高级类型。
//!
//! ## 模块内容
//!
//! - 生命周期基础
//! - 生命周期省略规则
//! - 结构体中的生命周期
//! - 静态生命周期
//! - 类型别名
//! - Never 类型
//!
//! ## 示例
//!
//! ```
//! use module_08_lifetimes::{first_word, longest, Context};
//!
//! let s = String::from("hello world");
//! let word = first_word(&s);
//! assert_eq!(word, "hello");
//!
//! let result = longest("long string", "short");
//! assert_eq!(result, "long string");
//! ```

use std::borrow::Cow;
use std::fmt::Display;

// ========== 基础生命周期函数 ==========

/// 返回两个字符串切片中较长的一个
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/// 返回第一个单词
pub fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    s
}

/// 返回所有直到第一个空格的内容
pub fn first_part<'a>(s: &'a str) -> &'a str {
    let end = s.find(' ').unwrap_or(s.len());
    &s[..end]
}

// ========== 结构体中的生命周期 ==========

/// 持有引用的上下文结构体
#[derive(Debug)]
pub struct Context<'a> {
    name: &'a str,
    description: &'a str,
}

impl<'a> Context<'a> {
    pub fn new(name: &'a str, description: &'a str) -> Self {
        Self { name, description }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn description(&self) -> &str {
        self.description
    }

    /// 使用生命周期省略规则的方法
    pub fn display(&self) -> String {
        format!("{}: {}", self.name, self.description)
    }
}

/// 重要摘录
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

    /// 演示生命周期省略规则第三条
    pub fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// ========== 多个生命周期参数 ==========

/// 比较两个不同生命周期的字符串
pub fn compare_and_print<'a, 'b>(x: &'a str, y: &'b str) -> bool {
    x == y
}

/// 选择第一个参数，演示生命周期子类型
pub fn choose_first<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str
where
    'b: 'a, // 'b 必须至少和 'a 一样长
{
    x
}

// ========== 泛型与生命周期 ==========

/// 持有引用的泛型对
#[derive(Debug)]
pub struct Pair<'a, T: 'a> {
    first: &'a T,
    second: &'a T,
}

impl<'a, T: 'a> Pair<'a, T> {
    pub fn new(first: &'a T, second: &'a T) -> Self {
        Self { first, second }
    }

    pub fn first(&self) -> &T {
        self.first
    }

    pub fn second(&self) -> &T {
        self.second
    }

    pub fn swap(&self) -> (&T, &T) {
        (self.second, self.first)
    }
}

// ========== 类型别名 ==========

/// 字符串引用的类型别名
pub type StrRef<'a> = &'a str;

/// 字符串切片结果的类型别名
pub type StrResult<'a> = Result<&'a str, ParseError>;

/// 解析错误
#[derive(Debug, PartialEq, Clone)]
pub enum ParseError {
    Empty,
    Invalid,
    Unknown,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Empty => write!(f, "Input is empty"),
            ParseError::Invalid => write!(f, "Invalid syntax"),
            ParseError::Unknown => write!(f, "Unknown error"),
        }
    }
}

impl std::error::Error for ParseError {}

/// 使用类型别名的解析函数
pub fn parse_string(s: &str) -> StrResult {
    if s.is_empty() {
        return Err(ParseError::Empty);
    }
    Ok(s)
}

// ========== Cow (Clone on Write) ==========

/// 根据内容决定是否克隆
pub fn to_uppercase_cow(s: &str) -> Cow<str> {
    if s.chars().all(|c| c.is_uppercase()) {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(s.to_uppercase())
    }
}

/// 条件克隆
pub fn maybe_clone(s: &str, should_clone: bool) -> Cow<str> {
    if should_clone {
        Cow::Owned(s.to_string())
    } else {
        Cow::Borrowed(s)
    }
}

// ========== 迭代器中的生命周期 ==========

/// 字符串分割迭代器
pub struct SplitIter<'a> {
    text: &'a str,
    delimiter: char,
    position: usize,
}

impl<'a> SplitIter<'a> {
    pub fn new(text: &'a str, delimiter: char) -> Self {
        Self {
            text,
            delimiter,
            position: 0,
        }
    }
}

impl<'a> Iterator for SplitIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position > self.text.len() {
            return None;
        }

        let remaining = &self.text[self.position..];
        match remaining.find(self.delimiter) {
            Some(idx) => {
                let result = &remaining[..idx];
                self.position += idx + 1;
                Some(result)
            }
            None if remaining.is_empty() => None,
            None => {
                let result = remaining;
                self.position = self.text.len() + 1;
                Some(result)
            }
        }
    }
}

// ========== 静态生命周期 ==========

/// 获取静态引用
pub fn get_static_str() -> &'static str {
    "I have a static lifetime!"
}

/// 静态字符串常量
pub const GREETING: &str = "Hello, World!";

// ========== 链式方法 ==========

/// 链式调用构建器
#[derive(Debug, Default)]
pub struct StringBuilder<'a> {
    parts: Vec<&'a str>,
}

impl<'a> StringBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append(mut self, part: &'a str) -> Self {
        self.parts.push(part);
        self
    }

    pub fn append_if(mut self, part: &'a str, condition: bool) -> Self {
        if condition {
            self.parts.push(part);
        }
        self
    }

    pub fn build(&self) -> String {
        self.parts.join("")
    }
}

// ========== 文本处理器 ==========

/// 文本处理器
pub struct TextProcessor<'a> {
    text: &'a str,
}

impl<'a> TextProcessor<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }

    pub fn trim(&self) -> &str {
        self.text.trim()
    }

    pub fn left(&self, n: usize) -> &str {
        &self.text[..n.min(self.text.len())]
    }

    pub fn right(&self, n: usize) -> &str {
        let len = self.text.len();
        &self.text[len.saturating_sub(n)..]
    }

    pub fn lines(&self) -> Vec<&str> {
        self.text.lines().collect()
    }

    pub fn contains(&self, pattern: &str) -> bool {
        self.text.contains(pattern)
    }

    pub fn starts_with(&self, pattern: &str) -> bool {
        self.text.starts_with(pattern)
    }

    pub fn ends_with(&self, pattern: &str) -> bool {
        self.text.ends_with(pattern)
    }
}

// ========== 引用包装器 ==========

/// 引用包装器
#[derive(Debug)]
pub struct RefWrapper<'a, T: 'a> {
    value: &'a T,
}

impl<'a, T: 'a> RefWrapper<'a, T> {
    pub fn new(value: &'a T) -> Self {
        Self { value }
    }

    pub fn get(&self) -> &T {
        self.value
    }

    pub fn map<U, F>(&self, f: F) -> U
    where
        F: FnOnce(&T) -> U,
    {
        f(self.value)
    }
}

// 为实现 Display 的类型提供方法
impl<'a, T: 'a + Display> RefWrapper<'a, T> {
    pub fn display(&self) {
        println!("{}", self.value);
    }
}

// 为实现 PartialEq 的类型提供方法
impl<'a, T: 'a + PartialEq> RefWrapper<'a, T> {
    pub fn equals(&self, other: &T) -> bool {
        self.value == other
    }
}

// ========== 测试 ==========

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        let s1 = "long string";
        let s2 = "short";
        assert_eq!(longest(s1, s2), s1);

        let s1 = "abc";
        let s2 = "defgh";
        assert_eq!(longest(s1, s2), s2);
    }

    #[test]
    fn test_first_word() {
        let s = String::from("hello world");
        assert_eq!(first_word(&s), "hello");

        let s = "single";
        assert_eq!(first_word(s), "single");
    }

    #[test]
    fn test_context() {
        let ctx = Context::new("test", "description");
        assert_eq!(ctx.name(), "test");
        assert_eq!(ctx.description(), "description");
        assert_eq!(ctx.display(), "test: description");
    }

    #[test]
    fn test_important_excerpt() {
        let excerpt = ImportantExcerpt::new("Hello, world!");
        assert_eq!(excerpt.part(), "Hello, world!");
        assert_eq!(excerpt.level(), 3);
    }

    #[test]
    fn test_pair() {
        let a = 10;
        let b = 20;
        let pair = Pair::new(&a, &b);
        assert_eq!(pair.first(), &10);
        assert_eq!(pair.second(), &20);
        assert_eq!(pair.swap(), (&20, &10));
    }

    #[test]
    fn test_parse_string() {
        assert!(parse_string("test").is_ok());
        assert_eq!(parse_string(""), Err(ParseError::Empty));
    }

    #[test]
    fn test_to_uppercase_cow() {
        let s = "UPPER";
        let result = to_uppercase_cow(s);
        assert!(matches!(result, Cow::Borrowed(..)));

        let s = "lower";
        let result = to_uppercase_cow(s);
        assert!(matches!(result, Cow::Owned(..)));
        assert_eq!(result, "LOWER");
    }

    #[test]
    fn test_split_iter() {
        let text = "a,b,c";
        let iter = SplitIter::new(text, ',');
        let parts: Vec<&str> = iter.collect();
        assert_eq!(parts, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_string_builder() {
        let result = StringBuilder::new()
            .append("Hello")
            .append(", ")
            .append("World")
            .build();
        assert_eq!(result, "Hello, World");
    }

    #[test]
    fn test_text_processor() {
        let text = "  Hello, World!  ";
        let processor = TextProcessor::new(text);
        assert_eq!(processor.trim(), "Hello, World!");
        assert_eq!(processor.left(5), "  Hel");
        assert_eq!(processor.right(6), "rld!  ");
    }

    #[test]
    fn test_ref_wrapper() {
        let value = 42;
        let wrapper = RefWrapper::new(&value);
        assert_eq!(wrapper.get(), &42);
        assert!(wrapper.equals(&42));
    }
}
