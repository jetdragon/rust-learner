//! # 练习 1: 定义结构体
//!
//! **难度**: 入门
//! **预计时间**: 10 分钟
//!
//! **前置知识**:
//! - 基本的 Rust 语法
//! - 变量和类型
//!
//! **学习目标**:
//! - 学习如何定义结构体
//! - 理解结构体字段
//! - 掌握创建结构体实例的方法

/// 书籍结构体
///
/// # TODO
///
/// 定义 Book 结构体，包含以下字段：
/// - title: String - 书名
/// - author: String - 作者
/// - pages: u32 - 页数
/// - available: bool - 是否可借阅
pub struct Book {
    pub title: String,
    pub author: String,
    pub pages: u32,
    pub available: bool,
}

impl Book {
    /// 创建新书籍实例
    pub fn new(title: String, author: String, pages: u32) -> Self {
        unimplemented!()
    }

    /// 返回书籍描述
    pub fn describe(&self) -> String {
        unimplemented!()
    }
}

fn main() {
    // TODO: 创建一个 Book 实例
    let book = Book::new(
        String::from("The Rust Programming Language"),
        String::from("Steve Klabnik and Carol Nichols"),
        550,
    );

    // 打印书籍信息
    println!("=== 书籍信息 ===");
    println!("{}", book.describe());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_book_struct() {
        let book = Book::new(
            String::from("Test Book"),
            String::from("Test Author"),
            100,
        );

        // 测试字段访问
        assert_eq!(book.title, "Test Book");
        assert_eq!(book.author, "Test Author");
        assert_eq!(book.pages, 100);
        assert!(book.available);
    }

    #[test]
    fn test_book_describe() {
        let book = Book::new(
            String::from("Rust in Action"),
            String::from("Tim McNamara"),
            350,
        );

        let description = book.describe();
        assert!(description.contains("Rust in Action"));
        assert!(description.contains("Tim McNamara"));
        assert!(description.contains("350"));
    }

    #[test]
    fn test_book_defaults() {
        let book = Book::new(
            String::from("Default Book"),
            String::from("Default Author"),
            200,
        );

        // 新书默认应该是可借阅的
        assert!(book.available);
    }
}
