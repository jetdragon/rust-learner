//! # 练习 3: 自定义错误类型
//!
//! 难度: 中等
//! 时间: 15 分钟
//! 前置知识: trait, 错误处理
//! 学习目标:
//!   - 定义自定义错误类型
//!   - 实现 Error trait

use std::fmt;
use std::error::Error as StdError;

#[derive(Debug)]
pub enum ConfigError {
    MissingQuery,
    MissingFilename,
    InvalidFilename(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::MissingQuery => write!(f, "Missing query string"),
            ConfigError::MissingFilename => write!(f, "Missing filename"),
            ConfigError::InvalidFilename(filename) => {
                write!(f, "Invalid filename: {}", filename)
            }
        }
    }
}

impl StdError for ConfigError {}

#[derive(Debug)]
pub enum SearchError {
    FileNotFound(String),
    IoError(std::io::Error),
}

impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SearchError::FileNotFound(filename) => {
                write!(f, "File not found: {}", filename)
            }
            SearchError::IoError(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl StdError for SearchError {}

impl From<std::io::Error> for SearchError {
    fn from(err: std::io::Error) -> Self {
        SearchError::IoError(err)
    }
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, ConfigError> {
        if args.len() < 3 {
            return Err(ConfigError::MissingQuery);
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        if filename.is_empty() {
            return Err(ConfigError::InvalidFilename(filename));
        }

        Ok(Config { query, filename })
    }
}

fn main() {
    let args: Vec<String> = vec![
        String::from("program"),
        String::from("hello"),
        String::from("test.txt"),
    ];

    match Config::new(&args) {
        Ok(config) => {
            println!(
                "Config: query={}, filename={}",
                config.query, config.filename
            );
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
