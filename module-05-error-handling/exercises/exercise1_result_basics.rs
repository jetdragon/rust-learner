//! # 练习 1: 基础 Result 处理
//!
//! **难度**: 入门
//! **预计时间**: 10 分钟

/// 安全除法函数
///
/// # TODO
/// 如果除数为零，返回 Err("除数不能为零")
/// 否则返回 Ok(a / b)
pub fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
    todo!()
}

/// 检查数字是否为偶数
pub fn is_even(n: i32) -> Result<bool, String> {
    todo!()
}

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
