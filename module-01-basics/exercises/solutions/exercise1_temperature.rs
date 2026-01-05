//! # 练习 1 解答: 温度转换
//!
//! **提示**：请先尝试自己实现，再参考此解答！
//!
//! # 实现思路
//!
//! 1. 温度转换公式：F = C × 9/5 + 32
//! 2. Rust 中的数学运算：使用 `*` 表示乘法，`/` 表示除法
//! 3. 注意浮点数除法会保留小数部分
//! 4. 运算符优先级：先乘除后加减

/// 将摄氏温度转换为华氏温度
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

// 也可以写成：
// pub fn celsius_to_fahrenheit(c: f64) -> f64 {
//     (c * 9.0 / 5.0) + 32.0
// }

// 或者更清晰地分步计算：
// pub fn celsius_to_fahrenheit(c: f64) -> f64 {
//     let ratio = 9.0 / 5.0;
//     let fahrenheit = c * ratio + 32.0;
//     fahrenheit
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freezing_point() {
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
    }

    #[test]
    fn test_boiling_point() {
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
    }

    #[test]
    fn test_negative_temperature() {
        assert_eq!(celsius_to_fahrenheit(-40.0), -40.0);
    }

    #[test]
    fn test_room_temperature() {
        let result = celsius_to_fahrenheit(25.0);
        assert!((result - 77.0).abs() < 0.01);
    }
}
