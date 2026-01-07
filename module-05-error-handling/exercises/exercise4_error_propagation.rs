//! # 练习 4: 错误传播
//!
//! **难度**: 中等
//! **预计时间**: 15 分钟

/// 使用 ? 操作符简化代码
///
/// # TODO
/// 1. 解析 a_str 为 i32
/// 2. 解析 b_str 为 i32
/// 3. 调用 safe_divide
/// 使用 ? 传播错误
pub fn parse_and_divide(a_str: &str, b_str: &str) -> Result<i32, String> {
    todo!()
}

/// 读取并验证数字
pub fn read_and_validate(s: &str, min: i32, max: i32) -> Result<i32, String> {
    todo!()
}

fn main() {
    println!("=== 解析并除法 ===");
    match parse_and_divide("10", "2") {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("错误: {}", e),
    }

    match parse_and_divide("abc", "2") {
        Ok(result) => println!("结果: {}", result),
        Err(e) => println!("错误: {}", e),
    }

    println!("\n=== 验证数字 ===");
    match read_and_validate("50", 0, 100) {
        Ok(n) => println!("有效数字: {}", n),
        Err(e) => println!("错误: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_divide() {
        assert_eq!(parse_and_divide("10", "2"), Ok(5));
        assert!(parse_and_divide("abc", "2").is_err());
        assert!(parse_and_divide("10", "0").is_err());
    }

    #[test]
    fn test_read_and_validate() {
        assert_eq!(read_and_validate("50", 0, 100), Ok(50));
        assert!(read_and_validate("150", 0, 100).is_err());
        assert!(read_and_validate("-10", 0, 100).is_err());
    }
}
