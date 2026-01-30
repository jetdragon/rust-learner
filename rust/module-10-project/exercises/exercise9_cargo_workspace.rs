//! # 练习 9: Cargo 工作空间
//!
//! 难度: 中等
//! 时间: 15 分钟
//! 前置知识: Cargo 基础
//! 学习目标:
//!   - 理解工作空间的结构
//!   - 使用工作空间中的库

// 在这个练习中，我们模拟一个工作空间项目
// 实际使用时，需要在 Cargo.toml 中配置

// [workspace]
// members = ["adder", "minigrep"]

// adder/Cargo.toml:
// [package]
// name = "adder"
// version = "0.1.0"
// edition = "2021"

// adder/src/lib.rs:
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

// minigrep/Cargo.toml:
// [package]
// name = "minigrep"
// version = "0.1.0"
// edition = "2021"

// [dependencies]
// adder = { path = "../adder" }

// minigrep/src/main.rs:
// use adder;

fn main() {
    println!("2 + 2 = {}", add(2, 2));
    println!("5 + 2 = {}", add_two(5));
}

// 工作空间的优点：
// 1. 共享依赖
// 2. 统一版本控制
// 3. 便于项目组织

mod adder {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

fn main() {
    use adder::add;
    println!("2 + 2 = {}", add(2, 2));
}
