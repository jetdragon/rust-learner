//! 练习 2: 数组操作
//!
//! # 目标
//! 学习数组引用、迭代器使用和类型转换
//!
//! # 任务
//! 实现一个函数，计算整数数组的平均值
//!
//! # 难度
//! 简单
//!
//! # 预计时间
//! 15 分钟
//!
//! # 前置知识
//! - 数组和切片 (&[T])
//! - 迭代器方法 (sum, len)
//! - 类型转换

/// 计算整数数组的平均值
///
/// # 参数
/// * `numbers` - 整数数组的引用
///
/// # 返回值
/// 数组的平均值，空数组返回 0.0
///
/// # 示例
/// ```rust
/// use module_01_basics::exercises::average;
///
/// assert_eq!(average(&[1, 2, 3, 4, 5]), 3.0);
/// assert_eq!(average(&[]), 0.0);
/// ```
pub fn average(numbers: &[i32]) -> f64 {
    // TODO: 计算数组的平均值
    // 提示:
    // 1. 检查数组是否为空
    // 2. 使用 iter().sum() 计算总和
    // 3. 使用 len() 获取长度
    // 4. 记得类型转换 as f64
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_numbers() {
        assert_eq!(average(&[1, 2, 3, 4, 5]), 3.0);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(average(&[42]), 42.0);
    }

    #[test]
    fn test_empty_array() {
        assert_eq!(average(&[]), 0.0);
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(average(&[-5, 0, 5]), 0.0);
    }

    #[test]
    fn test_mixed_numbers() {
        let result = average(&[10, 20, 30]);
        assert_eq!(result, 20.0);
    }
}
