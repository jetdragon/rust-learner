//! # 练习 4 解答: 变量隐藏与类型转换
//!
//! **提示**：请先尝试自己实现，再参考此解答！
//!
//! # 实现思路
//!
//! 1. 变量隐藏（Shadowing）允许我们用同一个名字声明新变量
//! 2. `.parse()` 将字符串转换为数字，返回 Result 类型
//! 3. `.expect()` 在解析失败时 panic 并显示错误信息
//! 4. 新的 `number` 变量会"隐藏"旧的字符串变量

fn main() {
    let number = "42";

    // 使用变量隐藏将字符串转换为 i32
    let number: i32 = number.parse().expect("解析失败：不是有效的数字");

    println!("number * 2 = {}", number * 2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_multiply() {
        let number = "42";
        let number: i32 = number.parse().expect("解析失败");
        assert_eq!(number, 42);
    }

    #[test]
    fn test_parse_negative() {
        let value = "-10";
        let value: i32 = value.parse().expect("解析失败");
        assert_eq!(value, -10);
    }

    #[test]
    fn test_parse_large_number() {
        let large = "1000000";
        let large: i32 = large.parse().expect("解析失败");
        assert_eq!(large, 1_000_000);
    }
}
