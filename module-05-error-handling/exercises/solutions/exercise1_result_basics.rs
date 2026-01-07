//! # 练习 1: 基础 Result 处理解答
//!
//! **解答要点:**
//! 1. 使用 if/else 或 match 表达式处理 Result
//! 2. 返回 Ok(value) 表示成功，Err(error) 表示失败

/// 安全除法函数
///
/// 如果除数为零，返回 Err("除数不能为零")
/// 否则返回 Ok(a / b)
pub fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("除数不能为零".to_string())
    } else {
        Ok(a / b)
    }
}

/// 检查数字是否为偶数
///
/// 注意：这个函数实际上不会失败，但返回 Result 类型
/// 演示了如何在某些情况下总是返回 Ok
pub fn is_even(n: i32) -> Result<bool, String> {
    Ok(n % 2 == 0)
}

// /// 使用 match 的替代实现
// pub fn safe_divide_match(a: i32, b: i32) -> Result<i32, String> {
//     match b {
//         0 => Err("除数不能为零".to_string()),
//         _ => Ok(a / b),
//     }
// }

fn main() {
    println!("=== 安全除法 ===");
    match safe_divide(10, 2) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("错误: {}", e),
    }

    match safe_divide(10, 0) {
        Ok(result) => println!("结果: {}", result),
        Err(e) => println!("错误: {}", e),
    }

    println!("\n=== 偶数检查 ===");
    match is_even(4) {
        Ok(true) => println!("4 是偶数"),
        Ok(false) => println!("4 不是偶数"),
        Err(e) => println!("错误: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(10, 2), Ok(5));
        assert_eq!(safe_divide(10, 0), Err("除数不能为零".to_string()));
    }

    #[test]
    fn test_safe_divide_negative() {
        assert_eq!(safe_divide(-10, 2), Ok(-5));
        assert_eq!(safe_divide(10, -2), Ok(-5));
    }

    #[test]
    fn test_is_even() {
        assert_eq!(is_even(4), Ok(true));
        assert_eq!(is_even(5), Ok(false));
    }
}
