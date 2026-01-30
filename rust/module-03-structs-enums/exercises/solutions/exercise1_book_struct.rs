//! # 练习 1 解答: 定义结构体
//!
//! **提示**：请先尝试自己实现，再参考此解答！
//!
//! # 实现思路
//!
//! 1. 结构体定义：使用 `pub struct` 定义公共结构体
//! 2. 字段：每个字段需要指定类型
//! 3. 实例化：使用结构体字面量语法创建实例
//! 4. 方法：在 `impl` 块中定义方法

/// 书籍结构体
pub struct Book {
    pub title: String,
    pub author: String,
    pub pages: u32,
    pub available: bool,
}

impl Book {
    /// 创建新书籍实例
    ///
    /// # 实现思路
    /// 使用字段初始化简写语法
    pub fn new(title: String, author: String, pages: u32) -> Self {
        Book {
            title,
            author,
            pages,
            available: true, // 新书默认可借阅
        }
    }

    /// 返回书籍描述
    ///
    /// # 实现思路
    /// 使用 format! 宏格式化字符串
    pub fn describe(&self) -> String {
        format!(
            "《{}》by {} - {}页 - {}",
            self.title,
            self.author,
            self.pages,
            if self.available { "可借阅" } else { "已借出" }
        )
    }
}

fn main() {
    let book = Book::new(
        String::from("The Rust Programming Language"),
        String::from("Steve Klabnik and Carol Nichols"),
        550,
    );

    println!("=== 书籍信息 ===");
    println!("{}", book.describe());

    // 演示直接访问字段
    println!("\n书名: {}", book.title);
    println!("作者: {}", book.author);
    println!("页数: {}", book.pages);
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

        assert!(book.available);
    }

    #[test]
    fn test_book_manual_creation() {
        // 也可以不使用 new 方法直接创建
        let book = Book {
            title: String::from("Manual Book"),
            author: String::from("Manual Author"),
            pages: 300,
            available: false,
        };

        assert_eq!(book.title, "Manual Book");
        assert!(!book.available);
    }

    #[test]
    fn test_update_syntax() {
        let book1 = Book::new(
            String::from("Original"),
            String::from("Author"),
            100,
        );

        // 使用结构体更新语法
        let book2 = Book {
            title: String::from("Updated"),
            ..book1
        };

        assert_eq!(book2.title, "Updated");
        assert_eq!(book2.author, "Author");
    }
}
