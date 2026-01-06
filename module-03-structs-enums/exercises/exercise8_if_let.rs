//! # 练习 8: if let 使用
//!
//! **难度**: 入门
//! **预计时间**: 10 分钟
//!
//! **前置知识**:
//! - Option 类型基础
//! - match 表达式
//!
//! **学习目标**:
//! - 学习使用 if let 简化代码
//! - 理解 if let 的适用场景
//! - 掌握 if let 的 else 分支

/// TODO: 使用 if let 简化 Option 处理
///
/// 如果值存在且大于 10，返回 Some(value * 2)
/// 否则返回 None
pub fn double_if_greater_than_ten(value: Option<i32>) -> Option<i32> {
    unimplemented!()
}

/// TODO: 使用 if let 处理 Result
///
/// 如果是 Ok 且值小于 100，返回 Ok(value + 1)
/// 否则返回原值或错误
pub fn increment_if_small(value: Result<i32, String>) -> Result<i32, String> {
    unimplemented!()
}

/// TODO: 检查 Option 是否包含偶数
pub fn contains_even_number(value: Option<i32>) -> bool {
    unimplemented!()
}

/// TODO: 获取 Option 的值或默认值
pub fn value_or_default(value: Option<i32>, default: i32) -> i32 {
    unimplemented!()
}

fn main() {
    println!("=== if let 练习 ===\n");

    // 基本用法
    println!("--- 基本 if let ---");
    let some_value: Option<i32> = Some(7);

    if let Some(i) = some_value {
        println!("值为: {}", i);
    } else {
        println!("没有值");
    }

    let none_value: Option<i32> = None;
    if let Some(i) = none_value {
        println!("值为: {}", i);
    } else {
        println!("没有值");
    }

    // 高级用法
    println!("\n--- 高级用法 ---");

    let test_values = vec![Some(5), Some(15), Some(20), None];
    for val in test_values {
        let result = double_if_greater_than_ten(val);
        println!("{:?} -> {:?}", val, result);
    }

    // Result 处理
    println!("\n--- Result 处理 ---");
    let ok_values = vec![Ok(50), Ok(150), Err("错误".to_string())];
    for val in ok_values {
        let result = increment_if_small(val.clone());
        println!("{:?} -> {:?}", val, result);
    }

    // 偶数检查
    println!("\n--- 偶数检查 ---");
    println!("Some(4) 包含偶数: {}", contains_even_number(Some(4)));
    println!("Some(5) 包含偶数: {}", contains_even_number(Some(5)));
    println!("None 包含偶数: {}", contains_even_number(None));

    // 默认值
    println!("\n--- 默认值 ---");
    println!("Some(10) 或 0: {}", value_or_default(Some(10), 0));
    println!("None 或 100: {}", value_or_default(None, 100));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_if_greater_than_ten_some() {
        assert_eq!(double_if_greater_than_ten(Some(15)), Some(30));
        assert_eq!(double_if_greater_than_ten(Some(20)), Some(40));
    }

    #[test]
    fn test_double_if_greater_than_ten_small() {
        assert_eq!(double_if_greater_than_ten(Some(5)), None);
        assert_eq!(double_if_greater_than_ten(Some(10)), None);
    }

    #[test]
    fn test_double_if_greater_than_ten_none() {
        assert_eq!(double_if_greater_than_ten(None), None);
    }

    #[test]
    fn test_increment_if_small_ok() {
        assert_eq!(increment_if_small(Ok(50)), Ok(51));
        assert_eq!(increment_if_small(Ok(99)), Ok(100));
    }

    #[test]
    fn test_increment_if_small_large() {
        assert_eq!(increment_if_small(Ok(100)), Ok(100));
        assert_eq!(increment_if_small(Ok(200)), Ok(200));
    }

    #[test]
    fn test_increment_if_small_err() {
        assert_eq!(
            increment_if_small(Err("错误".to_string())),
            Err("错误".to_string())
        );
    }

    #[test]
    fn test_contains_even_number() {
        assert!(contains_even_number(Some(2)));
        assert!(contains_even_number(Some(0)));
        assert!(contains_even_number(Some(-4)));
        assert!(!contains_even_number(Some(3)));
        assert!(!contains_even_number(None));
    }

    #[test]
    fn test_value_or_default() {
        assert_eq!(value_or_default(Some(42), 0), 42);
        assert_eq!(value_or_default(None, 100), 100);
    }

    #[test]
    fn test_if_let_vs_match() {
        // if let 和 match 的对比

        // 使用 match
        let value = Some(5);
        let result_match = match value {
            Some(v) if v > 3 => v * 2,
            _ => 0,
        };

        // 使用 if let（需要 else）
        let result_if_let = if let Some(v) = value {
            if v > 3 { v * 2 } else { 0 }
        } else {
            0
        };

        assert_eq!(result_match, result_if_let);
    }

    #[test]
    fn test_if_let_pattern_matching() {
        // 测试 if let 的模式匹配
        let some_tuple = Some((1, 2));

        if let Some((x, y)) = some_tuple {
            assert_eq!(x + y, 3);
        }
    }
}
