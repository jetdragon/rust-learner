//! 错误类型定义

use std::fmt;
use std::io;
use std::path::Path;

/// 应用错误类型
#[derive(Debug)]
pub enum Error {
    /// 任务不存在
    NotFound(u32),
    /// 无效输入
    InvalidInput(String),
    /// IO 错误
    Io(io::Error),
    /// JSON 序列化/反序列化错误
    Json(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NotFound(id) => write!(f, "任务 {} 不存在", id),
            Error::InvalidInput(msg) => write!(f, "无效输入: {}", msg),
            Error::Io(e) => write!(f, "IO 错误: {}", e),
            Error::Json(e) => write!(f, "JSON 错误: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Json(e)
    }
}

/// 数据可持久化 trait
pub trait Saveable {
    /// 保存到文件
    fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Error>;
    /// 从文件加载
    fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error>
    where
        Self: Sized;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display_not_found() {
        let err = Error::NotFound(123);
        assert_eq!(format!("{}", err), "任务 123 不存在");
    }

    #[test]
    fn test_error_display_invalid_input() {
        let err = Error::InvalidInput(String::from("test"));
        assert_eq!(format!("{}", err), "无效输入: test");
    }

    #[test]
    fn test_error_from_io() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let err: Error = io_err.into();
        assert!(matches!(err, Error::Io(_)));
    }

    #[test]
    fn test_error_from_json() {
        let json_err = serde_json::from_str::<serde_json::Value>("invalid").unwrap_err();
        let err: Error = json_err.into();
        assert!(matches!(err, Error::Json(_)));
    }
}
