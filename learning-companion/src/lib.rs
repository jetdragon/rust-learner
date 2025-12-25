//! 学习伴侣库 - 暴露公共 API 给测试使用

pub mod db;
pub mod exercise;
pub mod progress;
pub mod repo;
pub mod storage;
pub mod notify;

// 暴露 TUI 模块给测试
#[cfg(test)]
pub mod tui;
