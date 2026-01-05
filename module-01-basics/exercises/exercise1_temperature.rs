//! 练习 1: 温度转换
//!
//! # 目标
//! 学习函数定义、基本数学运算和返回值
//!
//! # 任务
//! 实现一个函数，将摄氏温度转换为华氏温度
//!
//! # 难度
//! 简单
//!
//! # 预计时间
//! 10 分钟
//!
//! # 前置知识
//! - 函数定义
//! - 基本数据类型（f64）
//! - 数学运算符

/// 将摄氏温度转换为华氏温度
///
/// # 参数
/// * `c` - 摄氏温度
///
/// # 返回值
/// 对应的华氏温度
///
/// # 公式
/// F = C × 9/5 + 32
///
/// # 示例
/// ```rust
/// use module_01_basics::exercises::celsius_to_fahrenheit;
///
/// assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
/// assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
/// ```
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    // TODO: 实现温度转换公式
    // 提示: F = C × 9/5 + 32
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freezing_point() {
        // 水的冰点: 0°C = 32°F
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
    }

    #[test]
    fn test_boiling_point() {
        // 水的沸点: 100°C = 212°F
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
    }

    #[test]
    fn test_negative_temperature() {
        // -40°C = -40°F (摄氏和华氏的交汇点)
        assert_eq!(celsius_to_fahrenheit(-40.0), -40.0);
    }

    #[test]
    fn test_room_temperature() {
        // 室温: 25°C ≈ 77°F
        let result = celsius_to_fahrenheit(25.0);
        assert!((result - 77.0).abs() < 0.01);
    }
}
