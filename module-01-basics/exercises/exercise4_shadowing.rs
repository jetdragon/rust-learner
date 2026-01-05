//! 练习 4: 变量隐藏与类型转换
//!
//! # 目标
//! 学习变量隐藏（shadowing）和字符串解析
//!
//! # 任务
//! 修复代码，使用变量隐藏将字符串转换为数字并进行运算
//!
//! # 难度
//! 中等
//!
//! # 预计时间
//! 10 分钟
//!
//! # 前置知识
//! - 变量隐藏 (shadowing)
//! - 字符串解析 (parse)
//! - 错误处理 (expect)

fn main() {
    let number = "42";

    // TODO: 使用变量隐藏将字符串转换为数字
    // 提示: 使用 .parse().expect("解析失败")
    // let number = ...

    println!("number * 2 = {}", number * 2);
}

// 不要修改下面的测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_multiply() {
        let number = "42";
        // TODO: 实现：将字符串转换为 i32 并乘以 2
        // let number = ...
        // assert_eq!(number, 84);
    }

    #[test]
    fn test_parse_negative() {
        let value = "-10";
        // TODO: 实现：将字符串转换为 i32
        // let value = ...
        // assert_eq!(value, -10);
    }

    #[test]
    fn test_parse_large_number() {
        let large = "1000000";
        // TODO: 实现：将字符串转换为 i32
        // let large = ...
        // assert_eq!(large, 1_000_000);
    }
}
