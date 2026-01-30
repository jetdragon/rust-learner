//! # 练习 5 解答: Option 类型使用
//!
//! **提示**：请先尝试自己实现，再参考此解答！
//!
//! # 实现思路
//!
//! 1. Option 类型：表示值可能存在或不存在
//! 2. Some(T)：值存在
//! 3. None：值不存在
//! 4. 模式匹配：使用 match 或 if let 处理 Option

/// 找出数组中第二大的值
///
/// # 实现思路
/// 1. 处理数组长度小于 2 的情况
/// 2. 使用迭代器找出最大值
/// 3. 找出小于最大值的最大值
pub fn find_second_largest(numbers: &[i32]) -> Option<i32> {
    if numbers.len() < 2 {
        return None;
    }

    // 找出最大值
    let max = *numbers.iter().max()?;

    // 找出小于最大值的最大值
    numbers.iter().filter(|&&x| x < max).max().copied()
}

/// 找出数组中第一个偶数
///
/// # 实现思路
/// 使用 find 方法找到第一个满足条件的元素
pub fn find_first_even(numbers: &[i32]) -> Option<i32> {
    numbers.iter().find(|&&x| x % 2 == 0).copied()
}

/// 获取数组的最后一个元素
///
/// # 实现思路
/// 使用 last 方法或检查长度后访问
pub fn last_element(numbers: &[i32]) -> Option<&i32> {
    if numbers.is_empty() {
        None
    } else {
        numbers.last()
    }
}

/// 将 Option 转换为字符串表示
pub fn option_to_string(opt: Option<i32>) -> String {
    match opt {
        Some(v) => format!("值为: {}", v),
        None => "没有值".to_string(),
    }
}

/// 获取 Option 的值或默认值
pub fn unwrap_or(opt: Option<i32>, default: i32) -> i32 {
    opt.unwrap_or(default)
}

/// 使用 map 转换 Option 中的值
pub fn double_if_some(opt: Option<i32>) -> Option<i32> {
    opt.map(|v| v * 2)
}

fn main() {
    let numbers1 = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let numbers2 = vec![42];
    let numbers3: Vec<i32> = vec![];

    println!("=== Option 类型使用 ===\n");

    // 第二大值
    println!("数组: {:?}", numbers1);
    match find_second_largest(&numbers1) {
        Some(v) => println!("第二大的值: {}", v),
        None => println!("数组元素不足"),
    }

    println!("\n数组: {:?}", numbers2);
    match find_second_largest(&numbers2) {
        Some(v) => println!("第二大的值: {}", v),
        None => println!("数组元素不足"),
    }

    // 第一个偶数
    println!("\n第一个偶数:");
    match find_first_even(&numbers1) {
        Some(v) => println!("找到偶数: {}", v),
        None => println!("没有偶数"),
    }

    // 最后一个元素
    println!("\n最后一个元素:");
    println!("{}", option_to_string(last_element(&numbers1).copied()));
    println!("{}", option_to_string(last_element(&numbers3).copied()));

    // 演示 unwrap_or
    println!("\n=== unwrap_or ===");
    println!("Some(10) 或 0: {}", unwrap_or(Some(10), 0));
    println!("None 或 100: {}", unwrap_or(None, 100));

    // 演示 map
    println!("\n=== map 操作 ===");
    println!("Some(5) 翻倍: {:?}", double_if_some(Some(5)));
    println!("None 翻倍: {:?}", double_if_some(None));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_second_largest_normal() {
        let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
        assert_eq!(find_second_largest(&numbers), Some(6));
    }

    #[test]
    fn test_find_second_largest_duplicates() {
        let numbers = vec![5, 5, 5];
        assert_eq!(find_second_largest(&numbers), Some(5));
    }

    #[test]
    fn test_find_second_largest_two_elements() {
        let numbers = vec![1, 2];
        assert_eq!(find_second_largest(&numbers), Some(1));
    }

    #[test]
    fn test_find_second_largest_one_element() {
        let numbers = vec![42];
        assert_eq!(find_second_largest(&numbers), None);
    }

    #[test]
    fn test_find_second_largest_empty() {
        let numbers: Vec<i32> = vec![];
        assert_eq!(find_second_largest(&numbers), None);
    }

    #[test]
    fn test_find_second_largest_negative() {
        let numbers = vec![-5, -2, -10, -3];
        assert_eq!(find_second_largest(&numbers), Some(-3));
    }

    #[test]
    fn test_find_first_even() {
        let numbers = vec![1, 3, 4, 7, 9];
        assert_eq!(find_first_even(&numbers), Some(4));
    }

    #[test]
    fn test_find_first_even_no_even() {
        let numbers = vec![1, 3, 5, 7];
        assert_eq!(find_first_even(&numbers), None);
    }

    #[test]
    fn test_find_first_even_empty() {
        let numbers: Vec<i32> = vec![];
        assert_eq!(find_first_even(&numbers), None);
    }

    #[test]
    fn test_last_element() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(last_element(&numbers), Some(&5));
    }

    #[test]
    fn test_last_element_single() {
        let numbers = vec![42];
        assert_eq!(last_element(&numbers), Some(&42));
    }

    #[test]
    fn test_last_element_empty() {
        let numbers: Vec<i32> = vec![];
        assert_eq!(last_element(&numbers), None);
    }

    #[test]
    fn test_option_to_string_some() {
        assert_eq!(option_to_string(Some(42)), "值为: 42");
    }

    #[test]
    fn test_option_to_string_none() {
        assert_eq!(option_to_string(None), "没有值");
    }

    #[test]
    fn test_double_if_some() {
        assert_eq!(double_if_some(Some(5)), Some(10));
        assert_eq!(double_if_some(None), None);
    }

    #[test]
    fn test_combinators() {
        // 演示 Option 的组合子操作
        let result = Some(5)
            .map(|x| x * 2)
            .filter(|x| *x > 5)
            .ok_or("Too small");

        assert_eq!(result, Ok(10));
    }

    #[test]
    fn test_and_then() {
        // 使用 and_then 链接多个可能失败的操作
        fn parse_and_double(s: &str) -> Option<i32> {
            s.parse().ok().map(|x: i32| x * 2)
        }

        assert_eq!(parse_and_double("5"), Some(10));
        assert_eq!(parse_and_double("abc"), None);
    }
}
