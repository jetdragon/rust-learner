//! # 练习 6: 自定义错误解答
//!
//! **解答要点:**
//! 1. 定义枚举类型表示不同错误
//! 2. 实现方法提供错误描述
//! 3. 返回具体的错误变体而不是字符串

/// 自定义数学错误类型
#[derive(Debug, PartialEq)]
pub enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
    InvalidInput,
}

impl MathError {
    pub fn description(&self) -> &str {
        match self {
            MathError::DivisionByZero => "除数不能为零",
            MathError::NegativeSquareRoot => "不能计算负数的平方根",
            MathError::InvalidInput => "无效的输入",
        }
    }
}

// 可选：实现 std::error::Error trait
// impl std::error::Error for MathError {}

// 可选：实现 Display trait
// impl std::fmt::Display for MathError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.description())
//     }
// }

/// 使用自定义错误类型
pub fn divide_with_custom_error(a: i32, b: i32) -> Result<i32, MathError> {
    if b == 0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

pub fn sqrt_with_custom_error(n: f64) -> Result<f64, MathError> {
    if n < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(n.sqrt())
    }
}

// /// 使用 match 的替代实现
// pub fn divide_with_custom_error_match(a: i32, b: i32) -> Result<i32, MathError> {
//     match b {
//         0 => Err(MathError::DivisionByZero),
//         _ => Ok(a / b),
//     }
// }

fn main() {
    println!("=== 自定义错误 ===");

    match divide_with_custom_error(10, 2) {
        Ok(result) => println!("结果: {}", result),
        Err(e) => println!("错误: {}", e.description()),
    }

    match divide_with_custom_error(10, 0) {
        Ok(result) => println!("结果: {}", result),
        Err(e) => println!("错误: {:?}", e),
    }

    match sqrt_with_custom_error(9.0) {
        Ok(result) => println!("√9 = {}", result),
        Err(e) => println!("错误: {}", e.description()),
    }

    match sqrt_with_custom_error(-1.0) {
        Ok(result) => println!("结果: {}", result),
        Err(e) => println!("错误: {:?}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_with_custom_error() {
        assert_eq!(divide_with_custom_error(10, 2), Ok(5));
        assert_eq!(
            divide_with_custom_error(10, 0),
            Err(MathError::DivisionByZero)
        );
    }

    #[test]
    fn test_sqrt_with_custom_error() {
        assert_eq!(sqrt_with_custom_error(9.0), Ok(3.0));
        assert_eq!(
            sqrt_with_custom_error(-1.0),
            Err(MathError::NegativeSquareRoot)
        );
    }

    #[test]
    fn test_error_description() {
        assert_eq!(MathError::DivisionByZero.description(), "除数不能为零");
        assert_eq!(
            MathError::NegativeSquareRoot.description(),
            "不能计算负数的平方根"
        );
    }
}
