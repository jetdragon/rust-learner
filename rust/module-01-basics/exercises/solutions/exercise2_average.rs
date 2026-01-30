//! # 练习 2 解答: 数组操作
//!
//! **提示**：请先尝试自己实现，再参考此解答！
//!
//! # 实现思路
//!
//! 1. 首先检查数组是否为空，避免除以零
//! 2. 使用 `iter()` 创建迭代器
//! 3. 使用 `sum()` 方法计算总和
//! 4. 使用 `len()` 获取元素个数
//! 5. 将总和转换为 f64 后除以元素个数
//! 6. 注意：需要先将 i32 转换为 f64 进行浮点除法

/// 计算整数数组的平均值
pub fn average(numbers: &[i32]) -> f64 {
    if numbers.is_empty() {
        return 0.0;
    }

    let sum: i32 = numbers.iter().sum();
    sum as f64 / numbers.len() as f64
}

// 替代实现 1：使用 fold
// pub fn average(numbers: &[i32]) -> f64 {
//     if numbers.is_empty() {
//         return 0.0;
//     }
//
//     let sum = numbers.iter().fold(0, |acc, &x| acc + x);
//     sum as f64 / numbers.len() as f64
// }

// 替代实现 2：显式循环
// pub fn average(numbers: &[i32]) -> f64 {
//     if numbers.is_empty() {
//         return 0.0;
//     }
//
//     let mut sum = 0;
//     for &num in numbers {
//         sum += num;
//     }
//     sum as f64 / numbers.len() as f64
// }

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
        assert_eq!(average(&[10, 20, 30]), 20.0);
    }
}
