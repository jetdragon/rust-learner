//! # 练习 9: Cargo 工作空间 - 解答
//!
//! 难度: 中等
//! 时间: 15 分钟
//! 学习目标: 理解工作空间的结构

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

mod adder {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

fn main() {
    use adder::add;
    println!("2 + 2 = {}", add(2, 2));
    println!("5 + 2 = {}", add_two(5));
}
