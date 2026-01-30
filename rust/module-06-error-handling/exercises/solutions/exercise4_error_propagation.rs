//! # 练习 4: 错误传播解答
//!
//! **解答要点:**
//! 1. 使用 ? 操作符自动传播错误
//! 2. ? 只能在返回 Result 的函数中使用
//! 3. ? 会自动进行类型转换（通过 From trait）

/// 使用 ? 操作符简化代码
///
/// 1. 解析 a_str 为 i32
/// 2. 解析 b_str 为 i32
/// 3. 调用 safe_divide
/// 使用 ? 传播错误
pub fn parse_and_divide(a_str: &str, b_str: &str) -> Result<i32, String> {
    // ? 操作符会在 Err 时提前返回错误
    let a: i32 = a_str.parse().map_err(|_| "无法解析第一个数字".to_string())?;
    let b: i32 = b_str.parse().map_err(|_| "无法解析第二个数字".to_string())?;

    // safe_divide 也返回 Result<i32, String>
    // ? 会传播它的错误
    safe_divide(a, b)
}

// /// 不使用 ? 的冗长版本
// pub fn parse_and_divide_verbose(a_str: &str, b_str: &str) -> Result<i32, String> {
//     match a_str.parse::<i32>() {
//         Ok(a) => match b_str.parse::<i32>() {
//             Ok(b) => safe_divide(a, b),
//             Err(_) => Err("无法解析第二个数字".to_string()),
//         },
//         Err(_) => Err("无法解析第一个数字".to_string()),
//     }
// }

/// 读取并验证数字
pub fn read_and_validate(s: &str, min: i32, max: i32) -> Result<i32, String> {
    let n: i32 = s.parse().map_err(|_| format!("'{}' 不是有效的数字", s))?;

    if n < min {
        return Err(format!("数字 {} 小于最小值 {}", n, min));
    }
    if n > max {
        return Err(format!("数字 {} 大于最大值 {}", n, max));
    }

    Ok(n)
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

    match read_and_validate("150", 0, 100) {
        Ok(n) => println!("有效数字: {}", n),
        Err(e) => println!("错误: {}", e),
    }
}

// 辅助函数
fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("除数不能为零".to_string())
    } else {
        Ok(a / b)
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
