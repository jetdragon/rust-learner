//! # 练习 5 解答: 函数式编程
//!
//! **提示**：请先尝试自己实现，再参考此解答！
//!
//! # 实现思路
//!
//! 1. Rust 中函数可以作为参数传递给其他函数
//! 2. 闭包语法：`|参数| 表达式` 或 `|参数| { 语句块 }`
//! 3. 泛型参数 `F` 和特征约束 `where F: Fn(i32) -> bool` 使函数更通用
//! 4. 高阶函数模式让我们可以灵活地组合不同的条件判断

/// 判断一个数字是否是偶数
pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

/// 判断一个数字是否是正数
pub fn is_positive(n: i32) -> bool {
    n > 0
}

/// 通用的条件检查函数
pub fn check<F>(n: i32, condition: F) -> bool
where
    F: Fn(i32) -> bool,
{
    condition(n)
}

// 使用闭包语法简化：
// pub fn check<F>(n: i32, condition: F) -> bool
// where
//     F: Fn(i32) -> bool,
// {
//     condition(n)
// }

/// 使用闭包判断数字是否在指定范围内
pub fn in_range(n: i32, min: i32, max: i32) -> bool {
    check(n, |x| x >= min && x <= max)
}

// 或者直接实现，不使用 check 函数：
// pub fn in_range(n: i32, min: i32, max: i32) -> bool {
//     n >= min && n <= max
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_even() {
        assert!(is_even(2));
        assert!(is_even(0));
        assert!(is_even(-4));
        assert!(!is_even(3));
    }

    #[test]
    fn test_is_positive() {
        assert!(is_positive(1));
        assert!(is_positive(100));
        assert!(!is_positive(0));
        assert!(!is_positive(-1));
    }

    #[test]
    fn test_check_with_function() {
        assert!(check(4, is_even));
        assert!(!check(5, is_even));
        assert!(check(10, is_positive));
        assert!(!check(-5, is_positive));
    }

    #[test]
    fn test_check_with_closure() {
        assert!(check(5, |n| n > 3));
        assert!(!check(2, |n| n > 3));
        assert!(check(10, |n| n % 2 == 0));
    }

    #[test]
    fn test_in_range() {
        assert!(in_range(5, 1, 10));
        assert!(in_range(1, 1, 10));
        assert!(in_range(10, 1, 10));
        assert!(!in_range(0, 1, 10));
        assert!(!in_range(11, 1, 10));
    }
}
