//! # 练习 3: 生命周期省略规则
//!
//! 难度: 简单
//! 时间: 10 分钟
//! 前置知识: 生命周期标注
//! 学习目标:
//!   - 理解生命周期省略的三条规则
//!   - 识别何时需要显式标注生命周期

// TODO: 这个函数需要生命周期标注吗？为什么？
pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// TODO: 这个函数需要生命周期标注吗？为什么？
pub fn parse_command(input: &str) -> Result<&str, &str> {
    if input.starts_with("/") {
        Ok(&input[1..])
    } else {
        Err("invalid command")
    }
}

fn main() {
    let text = "hello world";
    println!("First word: {}", first_word(text));

    match parse_command("/help") {
        Ok(cmd) => println!("Command: {}", cmd),
        Err(e) => println!("Error: {}", e),
    }
}
