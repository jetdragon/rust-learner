//! 练习 5: 函数式编程
//!
//! # 目标
//! 学习高阶函数、闭包和函数作为参数
//!
//! # 任务
//! 创建判断函数，并使用高阶函数模式进行条件检查
//!
//! # 难度
//! 困难
//!
//! # 预计时间
//! 20 分钟
//!
//! # 前置知识
//! - 函数指针
//! - 闭包 (closures)
//! - 高阶函数

/// 判断一个数字是否是偶数
///
/// # 参数
/// * `n` - 要检查的数字
///
/// # 返回值
/// 如果是偶数返回 true，否则返回 false
///
/// # 示例
/// ```rust
/// use module_01_basics::exercises::is_even;
///
/// assert!(is_even(4));
/// assert!(!is_even(5));
/// ```
pub fn is_even(n: i32) -> bool {
    // TODO: 实现偶数判断
    unimplemented!()
}

/// 判断一个数字是否是正数
///
/// # 参数
/// * `n` - 要检查的数字
///
/// # 返回值
/// 如果是正数返回 true，否则返回 false
pub fn is_positive(n: i32) -> bool {
    // TODO: 实现正数判断
    unimplemented!()
}

/// 通用的条件检查函数
///
/// # 参数
/// * `n` - 要检查的数字
/// * `condition` - 一个接受 i32 并返回 bool 的函数
///
/// # 返回值
/// 条件函数的返回值
///
/// # 示例
/// ```rust
/// use module_01_basics::exercises::*;
///
/// assert!(check(4, is_even));
/// assert!(!check(5, is_even));
/// assert!(check(5, |n| n > 0));
/// ```
pub fn check<F>(n: i32, condition: F) -> bool
where
    F: Fn(i32) -> bool,
{
    // TODO: 实现通用检查函数
    // 提示: 直接调用 condition 函数
    unimplemented!()
}

/// 使用闭包判断数字是否在指定范围内
///
/// # 参数
/// * `n` - 要检查的数字
/// * `min` - 范围最小值
/// * `max` - 范围最大值
///
/// # 返回值
/// 如果数字在 [min, max] 范围内返回 true
pub fn in_range(n: i32, min: i32, max: i32) -> bool {
    // TODO: 使用闭包实现
    // 提示: 可以定义一个闭包然后传给 check 函数
    unimplemented!()
}

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
        // 测试使用闭包
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
