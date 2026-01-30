//! 优先级枚举及其实现

use std::fmt;
use std::str::FromStr;

/// 任务优先级
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum Priority {
    /// 低优先级
    Low = 0,
    /// 中优先级
    #[default]
    Medium = 1,
    /// 高优先级
    High = 2,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Priority::Low => write!(f, "低"),
            Priority::Medium => write!(f, "中"),
            Priority::High => write!(f, "高"),
        }
    }
}

impl FromStr for Priority {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "low" | "低" => Ok(Priority::Low),
            "medium" | "中" => Ok(Priority::Medium),
            "high" | "高" => Ok(Priority::High),
            _ => Err(format!("无效的优先级: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Priority::Low), "低");
        assert_eq!(format!("{}", Priority::Medium), "中");
        assert_eq!(format!("{}", Priority::High), "高");
    }

    #[test]
    fn test_from_str_english() {
        assert_eq!("low".parse::<Priority>().unwrap(), Priority::Low);
        assert_eq!("medium".parse::<Priority>().unwrap(), Priority::Medium);
        assert_eq!("high".parse::<Priority>().unwrap(), Priority::High);
    }

    #[test]
    fn test_from_str_chinese() {
        assert_eq!("低".parse::<Priority>().unwrap(), Priority::Low);
        assert_eq!("中".parse::<Priority>().unwrap(), Priority::Medium);
        assert_eq!("高".parse::<Priority>().unwrap(), Priority::High);
    }

    #[test]
    fn test_from_str_case_insensitive() {
        assert_eq!("LOW".parse::<Priority>().unwrap(), Priority::Low);
        assert_eq!("Medium".parse::<Priority>().unwrap(), Priority::Medium);
        assert_eq!("HiGh".parse::<Priority>().unwrap(), Priority::High);
    }

    #[test]
    fn test_from_str_invalid() {
        assert!("invalid".parse::<Priority>().is_err());
    }

    #[test]
    fn test_ordering() {
        assert!(Priority::High > Priority::Medium);
        assert!(Priority::Medium > Priority::Low);
        assert!(Priority::Low < Priority::High);
    }

    #[test]
    fn test_default() {
        assert_eq!(Priority::default(), Priority::Medium);
    }
}
