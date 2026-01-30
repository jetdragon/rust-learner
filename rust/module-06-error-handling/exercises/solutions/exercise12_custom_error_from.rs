//! # 练习 12: 自定义错误类型与 From trait 解答
//!
//! **解答要点:**
//! 1. 自定义错误类型应派生 Debug 和 PartialEq
//! 2. 实现 Display trait 提供错误描述
//! 3. 实现 std::error::Error trait
//! 4. 为常见错误实现 From trait 简化错误转换

#[derive(Debug, PartialEq)]
pub enum AppError {
    InvalidInput(String),
    ConversionError(String),
    IoError(String),
}

impl AppError {
    pub fn description(&self) -> &str {
        match self {
            AppError::InvalidInput(msg) => msg,
            AppError::ConversionError(msg) => msg,
            AppError::IoError(msg) => msg,
        }
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl std::error::Error for AppError {}

// 实现 From trait
impl From<std::num::ParseIntError> for AppError {
    fn from(err: std::num::ParseIntError) -> Self {
        AppError::ConversionError(err.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoError(err.to_string())
    }
}

pub fn parse_int(s: &str) -> Result<i32, AppError> {
    s.parse().map_err(AppError::from)
}

pub fn validate_positive(n: i32) -> Result<i32, AppError> {
    if n > 0 {
        Ok(n)
    } else {
        Err(AppError::InvalidInput("数字必须大于 0".to_string()))
    }
}

pub fn parse_and_validate(s: &str) -> Result<i32, AppError> {
    let n = parse_int(s)?;
    validate_positive(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_int_valid() {
        assert_eq!(parse_int("42"), Ok(42));
    }

    #[test]
    fn test_parse_int_invalid() {
        assert!(parse_int("abc").is_err());
    }

    #[test]
    fn test_validate_positive_valid() {
        assert_eq!(validate_positive(5), Ok(5));
    }

    #[test]
    fn test_validate_positive_invalid() {
        assert!(validate_positive(-1).is_err());
    }

    #[test]
    fn test_parse_and_validate_valid() {
        assert_eq!(parse_and_validate("10"), Ok(10));
    }

    #[test]
    fn test_parse_and_validate_invalid_parse() {
        assert!(parse_and_validate("abc").is_err());
    }

    #[test]
    fn test_parse_and_validate_invalid_positive() {
        assert!(parse_and_validate("-5").is_err());
    }
}
