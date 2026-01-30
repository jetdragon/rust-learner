//! # 练习 6: 测试
//!
//! 难度: 简单
//! 时间: 15 分钟
//! 前置知识: 函数基础
//! 学习目标:
//!   - 编写单元测试
//!   - 使用断言宏

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

pub fn search(query: &str, text: &str) -> Vec<&str> {
    text.lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2), Ok(5));
        assert_eq!(divide(10, 0), Err(String::from("Division by zero")));
    }

    #[test]
    fn test_search() {
        let text = "Hello World\nHello Rust\nProgramming";
        let results = search("Hello", text);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0], "Hello World");
        assert_eq!(results[1], "Hello Rust");
    }

    #[test]
    fn test_search_no_results() {
        let text = "Hello World\nHello Rust";
        let results = search("Python", text);
        assert!(results.is_empty());
    }

    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_divide_panic() {
        divide(10, 0).unwrap();
    }
}

fn main() {
    println!("add(2, 3) = {}", add(2, 3));
    println!("divide(10, 2) = {:?}", divide(10, 2));

    let text = "Hello World\nHello Rust";
    println!("Search results: {:?}", search("Hello", text));
}
