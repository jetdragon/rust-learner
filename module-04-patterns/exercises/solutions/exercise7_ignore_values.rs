//! # 练习 7 解答: 忽略值
//!
//! **提示**：请先尝试自己实现，再参考此解答！
//!
//! # 实现思路
//!
//! 1. _ 通配符：忽略不需要的值
//! 2. _前缀：忽略未使用的变量
//! 3. .. 忽略剩余部分
//! 4. ..= 用于范围的包含边界

/// 只计算前三个元素的和
pub fn sum_first_three(tuple: (i32, i32, i32, i32, i32)) -> i32 {
    let (a, b, c, _, _) = tuple;
    a + b + c
}

/// 忽略元组的某些部分
pub fn first_and_last(tuple: (i32, i32, i32, i32)) -> (i32, i32) {
    let (first, _, _, last) = tuple;
    (first, last)
}

/// 忽略结构体字段
#[derive(Debug)]
pub struct Config {
    pub name: String,
    pub debug: bool,
    pub port: u16,
    pub max_connections: u32,
}

pub fn get_server_info(config: &Config) -> String {
    let Config { name, port, .. } = config;
    format!("Server '{}' on port {}", name, port)
}

/// 只获取奇数位置的和
pub fn sum_odd_positions(tuple: (i32, i32, i32, i32, i32)) -> i32 {
    let (_, a, _, b, _) = tuple;
    a + b
}

fn main() {
    let tuple = (1, 2, 3, 4, 5);
    println!("前三个元素之和: {}", sum_first_three(tuple));

    let config = Config {
        name: "MyServer".to_string(),
        debug: true,
        port: 8080,
        max_connections: 100,
    };
    println!("{}", get_server_info(&config));

    // 演示忽略值
    println!("\n=== 忽略值示例 ===");

    // 忽略函数返回值
    let _ = noisy_function();

    // 忽略循环计数器
    let _ = (1..=10).map(|_| println!("doing something"));

    // 在模式中忽略
    let numbers = (1, 2, 3, 4, 5);
    let (first, .., last) = numbers;
    println!("第一个: {}, 最后一个: {}", first, last);
}

fn noisy_function() -> i32 {
    println!("做一些噪音...");
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_first_three() {
        assert_eq!(sum_first_three((1, 2, 3, 4, 5)), 6);
        assert_eq!(sum_first_three((10, 20, 30, 40, 50)), 60);
    }

    #[test]
    fn test_first_and_last() {
        assert_eq!(first_and_last((1, 2, 3, 4)), (1, 4));
        assert_eq!(first_and_last((10, 20, 30, 40)), (10, 40));
    }

    #[test]
    fn test_get_server_info() {
        let config = Config {
            name: "TestServer".to_string(),
            debug: false,
            port: 3000,
            max_connections: 50,
        };
        assert_eq!(get_server_info(&config), "Server 'TestServer' on port 3000");
    }

    #[test]
    fn test_sum_odd_positions() {
        assert_eq!(sum_odd_positions((1, 2, 3, 4, 5)), 6); // 2 + 4
        assert_eq!(sum_odd_positions((10, 20, 30, 40, 50)), 60); // 20 + 40
    }

    #[test]
    fn test_double_dot_pattern() {
        // 使用 .. 忽略中间的多个值
        let tuple = (1, 2, 3, 4, 5);
        let (first, .., last) = tuple;
        assert_eq!(first, 1);
        assert_eq!(last, 5);

        // 使用 .. 忽略后面的所有值
        let (a, b, ..) = tuple;
        assert_eq!(a, 1);
        assert_eq!(b, 2);
    }

    #[test]
    fn test_underscore_unused() {
        // 使用 _ 避免未使用变量警告
        let _unused = 42;
        let (_x, y) = (1, 2);
        assert_eq!(y, 2);
    }
}
