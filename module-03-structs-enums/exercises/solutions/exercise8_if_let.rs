//! # 练习 8 解答: if let 使用
//!
//! **提示**：请先尝试自己实现，再参考此解答！
//!
//! # 实现思路
//!
//! 1. if let：简化只关心一种情况的匹配
//! 2. let else：简化只关心错误情况的处理
//! 3. 可读性：if let 比完整的 match 更简洁
//! 4. 权衡：不如 match 穷尽，但代码更简洁

/// 使用 if let 简化 Option 处理
///
/// # 实现思路
/// 如果值存在且大于 10，返回 Some(value * 2)
pub fn double_if_greater_than_ten(value: Option<i32>) -> Option<i32> {
    // 使用 if let 检查条件
    if let Some(v) = value {
        if v > 10 {
            Some(v * 2)
        } else {
            None
        }
    } else {
        None
    }
}

/// 使用 let else 处理 Result
///
/// # 实现思路
/// 如果是 Ok 且值小于 100，返回 Ok(value + 1)
pub fn increment_if_small(value: Result<i32, String>) -> Result<i32, String> {
    match value {
        Ok(v) if v < 100 => Ok(v + 1),
        Ok(v) => Ok(v),
        Err(e) => Err(e),
    }
}

/// 检查 Option 是否包含偶数
pub fn contains_even_number(value: Option<i32>) -> bool {
    // 使用 if let 简化
    if let Some(v) = value {
        v % 2 == 0
    } else {
        false
    }
}

/// 获取 Option 的值或默认值
pub fn value_or_default(value: Option<i32>, default: i32) -> i32 {
    value.unwrap_or(default)
}

/// 使用 let else 简化错误处理（Rust 1.65+）
#[cfg(feature = "let_else")]
pub fn parse_or_default(s: &str) -> i32 {
    let Ok(n) = s.parse::<i32>() else {
        return 0;
    };
    n
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

    // 演示 if let vs match
    println!("\n--- if let vs match ---");
    let value = Some(42);

    // 使用 match（冗长）
    let _match_result = match value {
        Some(v) => println!("match: 值为 {}", v),
        None => println!("match: 没有值"),
    };

    // 使用 if let（简洁）
    if let Some(v) = value {
        println!("if let: 值为 {}", v);
    }

    // 演示 if let 与守卫
    println!("\n--- if let 与守卫 ---");
    let numbers = vec![Some(5), Some(15), Some(25), None];

    for num in numbers {
        if let Some(n) = num {
            if n > 10 {
                println!("{} 大于 10", n);
            } else {
                println!("{} 不大于 10", n);
            }
        } else {
            println!("没有值");
        }
    }
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

    #[test]
    fn test_if_let_with_guard() {
        // if let 配合守卫
        let value = Some(10);

        let result = if let Some(v) = value {
            if v > 5 { "large" } else { "small" }
        } else {
            "none"
        };

        assert_eq!(result, "large");
    }

    #[test]
    fn test_multiple_if_let() {
        // 链式 if let
        let value = Some(Some(42));

        // 只在嵌套的 Some 时执行
        if let Some(inner) = value {
            if let Some(v) = inner {
                assert_eq!(v, 42);
            }
        }
    }

    #[test]
    fn test_let_else_pattern() {
        // 演示 let else 模式的思想
        // （注意：let else 语法是 Rust 1.65+ 的特性）

        // 传统方式
        fn parse_positive_old(s: &str) -> Option<i32> {
            let n = s.parse::<i32>().ok()?;
            if n > 0 { Some(n) } else { None }
        }

        // 使用 let else 的思想
        fn parse_positive_new(s: &str) -> Option<i32> {
            let n = s.parse::<i32>().ok()?;
            (n > 0).then_some(n)
        }

        assert_eq!(parse_positive_old("5"), Some(5));
        assert_eq!(parse_positive_old("-5"), None);
        assert_eq!(parse_positive_new("5"), Some(5));
        assert_eq!(parse_positive_new("-5"), None);
    }
}
