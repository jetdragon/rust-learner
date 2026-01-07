//! # 练习 5: 'static 生命周期
//!
//! 难度: 简单
//! 时间: 10 分钟
//! 前置知识: 生命周期基础
//! 学习目标:
//!   - 理解 'static 生命周期的含义
//!   - 区分字符串字面量和 String

// TODO: 为这些函数添加适当的返回类型
pub fn get_greeting() -> &'static str {
    "Hello, World!"
}

pub fn get_error_message() -> &'static str {
    "Something went wrong"
}

pub fn get_config_value(key: &str) -> Option<&'static str> {
    match key {
        "host" => Some("localhost"),
        "port" => Some("8080"),
        _ => None,
    }
}

fn main() {
    println!("Greeting: {}", get_greeting());
    println!("Error: {}", get_error_message());

    if let Some(host) = get_config_value("host") {
        println!("Host: {}", host);
    }

    // 对比: String vs &'static str
    let dynamic_string = String::from("Dynamic");
    let static_string = "Static";

    println!("Dynamic: {}, Static: {}", dynamic_string, static_string);
}
