//! # 错误处理 (Error Handling)
//!
//! 本模块提供 Rust 错误处理的实用类型和函数示例。
//!
//! ## 模块内容
//!
//! - 自定义错误类型：`MathError`, `ParseError`
//! - Result 处理函数：除法、平方根等
//! - 错误传播和转换示例
//!
//! ## 示例
//!
//! ```
//! use module_05_error_handling::safe_divide;
//!
//! assert_eq!(safe_divide(10, 2), Ok(5));
//! assert!(safe_divide(10, 0).is_err());
//! ```

use std::fmt;

// ========== 自定义错误类型 ==========

/// 数学运算错误
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MathError {
    /// 除零错误
    DivisionByZero,
    /// 负数平方根错误
    NegativeSquareRoot,
    /// 数值超出范围
    OutOfRange,
    /// 其他错误
    Other(String),
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "除数不能为零"),
            MathError::NegativeSquareRoot => write!(f, "不能计算负数的平方根"),
            MathError::OutOfRange => write!(f, "数值超出范围"),
            MathError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for MathError {}

/// 解析错误
#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
}

impl ParseError {
    pub fn new(message: String, line: usize) -> Self {
        Self { message, line }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "解析错误 (行 {}): {}", self.line, self.message)
    }
}

impl std::error::Error for ParseError {}

// ========== Result 处理函数 ==========

/// 安全除法
///
/// # 示例
///
/// ```
/// use module_05_error_handling::{safe_divide, MathError};
///
/// assert_eq!(safe_divide(10, 2), Ok(5));
/// assert_eq!(safe_divide(10, 0), Err(MathError::DivisionByZero));
/// ```
pub fn safe_divide(a: i32, b: i32) -> Result<i32, MathError> {
    if b == 0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

/// 浮点数除法，返回字符串错误
pub fn safe_divide_float(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("除数不能为零".to_string())
    } else {
        Ok(a / b)
    }
}

/// 安全平方根
///
/// # 示例
///
/// ```
/// use module_05_error_handling::safe_sqrt;
///
/// assert_eq!(safe_sqrt(9.0), Ok(3.0));
/// assert!(safe_sqrt(-1.0).is_err());
/// ```
pub fn safe_sqrt(n: f64) -> Result<f64, MathError> {
    if n < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(n.sqrt())
    }
}

/// 将 Option 转换为 Result
pub fn option_to_result(opt: Option<i32>, err: String) -> Result<i32, String> {
    match opt {
        Some(value) => Ok(value),
        None => Err(err),
    }
}

/// 使用 if let 转换 Option 为 Result
pub fn option_to_result_if_let(opt: Option<i32>, err: String) -> Result<i32, String> {
    if let Some(value) = opt {
        Ok(value)
    } else {
        Err(err)
    }
}

/// 使用 ok_or_else 转换
pub fn option_to_result_combinator(opt: Option<i32>, err: String) -> Result<i32, String> {
    opt.ok_or_else(|| err)
}

// ========== 错误传播函数 ==========

/// 解析并除法
///
/// 解析两个字符串为整数并相除
pub fn parse_and_divide(a_str: &str, b_str: &str) -> Result<i32, String> {
    let a: i32 = a_str.parse().map_err(|_| "无法解析第一个数".to_string())?;
    let b: i32 = b_str.parse().map_err(|_| "无法解析第二个数".to_string())?;
    safe_divide(a, b).map_err(|e| e.to_string())
}

/// 解析并计算平方根
pub fn parse_and_sqrt(s: &str) -> Result<f64, String> {
    let n: f64 = s.parse().map_err(|_| "无法解析数字".to_string())?;
    safe_sqrt(n).map_err(|e| e.to_string())
}

// ========== Option 和 Result 组合 ==========

/// 如果值为正数则翻倍
pub fn double_if_positive(n: i32) -> Result<i32, String> {
    if n > 0 {
        Ok(n * 2)
    } else {
        Err("值必须为正数".to_string())
    }
}

/// 验证 Option 并翻倍
pub fn validate_and_double(v: Option<i32>) -> Result<i32, String> {
    let value = v.ok_or_else(|| "值为空".to_string())?;
    double_if_positive(value)
}

// ========== 默认值和转换 ==========

/// 获取向量第一个元素或默认值
pub fn get_first_or_default(vec: &[i32]) -> i32 {
    vec.first().copied().unwrap_or(0)
}

/// 使用 unwrap_or_else
pub fn get_first_or_default_lazy(vec: &[i32]) -> i32 {
    vec.first().copied().unwrap_or_else(|| {
        // 这里可以执行复杂的计算
        0
    })
}

/// 如果值存在则平方
pub fn square_if_positive(opt: Option<i32>) -> Option<i32> {
    opt.map(|n| n * n)
}

// ========== Result 组合 ==========

/// 组合两个 Result，求和
pub fn sum_results(a: Result<i32, String>, b: Result<i32, String>) -> Result<i32, String> {
    match (a, b) {
        (Ok(x), Ok(y)) => Ok(x + y),
        (Err(e), _) => Err(e),
        (Ok(_), Err(e)) => Err(e),
    }
}

/// 组合多个 Result
pub fn sum_all_results(results: Vec<Result<i32, String>>) -> Result<i32, String> {
    let mut sum = 0;
    for result in results {
        sum = result.map(|v| sum + v)?;
    }
    Ok(sum)
}

// ========== 自定义错误处理 ==========

/// 使用自定义错误类型的除法
pub fn divide_with_custom_error(a: i32, b: i32) -> Result<i32, MathError> {
    if b == 0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

/// 使用自定义错误类型的平方根
pub fn sqrt_with_custom_error(n: f64) -> Result<f64, MathError> {
    if n < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(n.sqrt())
    }
}

// ========== 链式操作示例 ==========

/// 多步计算链
pub fn calculate_chain(input: &str) -> Result<f64, String> {
    // 实际示例：
    let n: f64 = input.parse().map_err(|_| "解析失败".to_string())?;
    let sqrt = safe_sqrt(n).map_err(|e| e.to_string())?;
    let result = sqrt * 2.0;
    Ok(result)
}

// ========== 错误上下文 ==========

/// 添加错误上下文
pub fn divide_with_context(a: i32, b: i32) -> Result<i32, String> {
    safe_divide(a, b).map_err(|e| format!("除法失败 ({}/{}): {}", a, b, e))
}

/// 链式错误上下文
pub fn parse_divide_with_context(a_str: &str, b_str: &str) -> Result<i32, String> {
    let a = a_str
        .parse::<i32>()
        .map_err(|_| format!("无法解析 '{}'", a_str))?;
    let b = b_str
        .parse::<i32>()
        .map_err(|_| format!("无法解析 '{}'", b_str))?;
    divide_with_context(a, b)
}

// ========== 测试 ==========

#[cfg(test)]
mod tests {
    use super::*;

    // 测试 safe_divide
    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(10, 2), Ok(5));
        assert_eq!(safe_divide(10, 3), Ok(3));
        assert_eq!(safe_divide(-10, 2), Ok(-5));
        assert_eq!(safe_divide(10, 0), Err(MathError::DivisionByZero));
    }

    #[test]
    fn test_safe_divide_float() {
        assert_eq!(safe_divide_float(10.0, 2.0), Ok(5.0));
        assert_eq!(
            safe_divide_float(10.0, 0.0),
            Err("除数不能为零".to_string())
        );
    }

    // 测试 safe_sqrt
    #[test]
    fn test_safe_sqrt() {
        assert_eq!(safe_sqrt(9.0), Ok(3.0));
        assert_eq!(safe_sqrt(0.0), Ok(0.0));
        assert_eq!(safe_sqrt(2.0), Ok(std::f64::consts::SQRT_2));
        assert_eq!(safe_sqrt(-1.0), Err(MathError::NegativeSquareRoot));
    }

    // 测试 option_to_result
    #[test]
    fn test_option_to_result() {
        assert_eq!(option_to_result(Some(5), "错误".to_string()), Ok(5));
        assert_eq!(
            option_to_result(None, "错误".to_string()),
            Err("错误".to_string())
        );
        assert_eq!(option_to_result_if_let(Some(5), "错误".to_string()), Ok(5));
        assert_eq!(
            option_to_result_combinator(Some(5), "错误".to_string()),
            Ok(5)
        );
    }

    // 测试 parse_and_divide
    #[test]
    fn test_parse_and_divide() {
        assert_eq!(parse_and_divide("10", "2"), Ok(5));
        assert!(parse_and_divide("abc", "2").is_err());
        assert!(parse_and_divide("10", "xyz").is_err());
        assert!(parse_and_divide("10", "0").is_err());
    }

    // 测试 parse_and_sqrt
    #[test]
    fn test_parse_and_sqrt() {
        assert_eq!(parse_and_sqrt("9"), Ok(3.0));
        assert!(parse_and_sqrt("-1").is_err());
        assert!(parse_and_sqrt("abc").is_err());
    }

    // 测试 double_if_positive
    #[test]
    fn test_double_if_positive() {
        assert_eq!(double_if_positive(5), Ok(10));
        assert_eq!(double_if_positive(0), Err("值必须为正数".to_string()));
        assert_eq!(double_if_positive(-5), Err("值必须为正数".to_string()));
    }

    // 测试 validate_and_double
    #[test]
    fn test_validate_and_double() {
        assert_eq!(validate_and_double(Some(5)), Ok(10));
        assert!(validate_and_double(Some(-1)).is_err());
        assert!(validate_and_double(None).is_err());
    }

    // 测试 get_first_or_default
    #[test]
    fn test_get_first_or_default() {
        assert_eq!(get_first_or_default(&[1, 2, 3]), 1);
        assert_eq!(get_first_or_default(&[]), 0);
        assert_eq!(get_first_or_default_lazy(&[1, 2, 3]), 1);
        assert_eq!(get_first_or_default_lazy(&[]), 0);
    }

    // 测试 square_if_positive
    #[test]
    fn test_square_if_positive() {
        assert_eq!(square_if_positive(Some(5)), Some(25));
        assert_eq!(square_if_positive(Some(-5)), Some(25));
        assert_eq!(square_if_positive(None), None);
    }

    // 测试 sum_results
    #[test]
    fn test_sum_results() {
        assert_eq!(sum_results(Ok(5), Ok(3)), Ok(8));
        assert_eq!(
            sum_results(Ok(5), Err("错误".to_string())),
            Err("错误".to_string())
        );
        assert_eq!(
            sum_results(Err("错误".to_string()), Ok(3)),
            Err("错误".to_string())
        );
    }

    // 测试 sum_all_results
    #[test]
    fn test_sum_all_results() {
        assert_eq!(sum_all_results(vec![Ok(1), Ok(2), Ok(3)]), Ok(6));
        assert!(sum_all_results(vec![Ok(1), Err("e".to_string())]).is_err());
    }

    // 测试自定义错误
    #[test]
    fn test_custom_errors() {
        assert_eq!(divide_with_custom_error(10, 2), Ok(5));
        assert_eq!(
            divide_with_custom_error(10, 0),
            Err(MathError::DivisionByZero)
        );

        assert_eq!(sqrt_with_custom_error(9.0), Ok(3.0));
        assert_eq!(
            sqrt_with_custom_error(-1.0),
            Err(MathError::NegativeSquareRoot)
        );
    }

    // 测试 MathError Display
    #[test]
    fn test_math_error_display() {
        assert_eq!(format!("{}", MathError::DivisionByZero), "除数不能为零");
        assert_eq!(
            format!("{}", MathError::NegativeSquareRoot),
            "不能计算负数的平方根"
        );
    }

    // 测试错误上下文
    #[test]
    fn test_error_context() {
        assert_eq!(
            divide_with_context(10, 0),
            Err("除法失败 (10/0): 除数不能为零".to_string())
        );
    }

    // 测试链式计算
    #[test]
    fn test_calculate_chain() {
        assert_eq!(calculate_chain("16"), Ok(8.0));
        assert!(calculate_chain("-1").is_err());
        assert!(calculate_chain("abc").is_err());
    }

    // 测试 parse_divide_with_context
    #[test]
    fn test_parse_divide_with_context() {
        assert_eq!(parse_divide_with_context("10", "2"), Ok(5));
        assert!(parse_divide_with_context("abc", "2").is_err());
        assert!(parse_divide_with_context("10", "0").is_err());
    }
}
