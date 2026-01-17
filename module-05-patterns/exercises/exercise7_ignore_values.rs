//! # 练习 7: 忽略值
//!
//! **难度**: 入门
//! **预计时间**: 10 分钟

/// TODO: 只计算前三个元素的和，忽略后两个
pub fn sum_first_three(tuple: (i32, i32, i32, i32, i32)) -> i32 {
    let (a, b, c, _, _) = tuple;
    a + b + c
}

/// TODO: 忽略不需要的结构体字段
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
    fn test_get_server_info() {
        let config = Config {
            name: "TestServer".to_string(),
            debug: false,
            port: 3000,
            max_connections: 50,
        };
        assert_eq!(get_server_info(&config), "Server 'TestServer' on port 3000");
    }
}
