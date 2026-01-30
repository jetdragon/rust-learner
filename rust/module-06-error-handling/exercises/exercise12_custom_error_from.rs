// exercise12_custom_error_from.rs
#[derive(Debug, PartialEq)]
pub enum AppError {
    InvalidInput(String),
    ConversionError(String),
    IoError(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::InvalidInput(msg) => write!(f, "{}", msg),
            AppError::ConversionError(msg) => write!(f, "{}", msg),
            AppError::IoError(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl From<std::num::ParseIntError> for AppError {
    fn from(err: std::num::ParseIntError) -> Self {
        AppError::ConversionError(err.to_string())
    }
}

pub fn parse_int(s: &str) -> Result<i32, AppError> {
    s.parse().map_err(AppError::from)
}
