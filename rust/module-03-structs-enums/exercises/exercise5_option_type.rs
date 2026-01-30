//! # 练习 5: Option 类型使用
//!
//! **难度**: 中等
//! **预计时间**: 20 分钟
//!
//! **前置知识**:
//! - 枚举基础
//! - match 表达式
//! - 数组和切片操作
//!
//! **学习目标**:
//! - 理解 Option 类型的用途
//! - 学会处理可能不存在的值
//! - 掌握 match 和 if let 处理 Option

/// TODO: 找出数组中第二大的值
///
/// # 参数
/// - `numbers`: 整数切片
///
/// # 返回
/// - `Some(second_largest)`: 如果找到第二大的值
/// - `None`: 如果数组元素少于 2 个
///
/// # 提示
/// 1. 处理数组长度小于 2 的情况
/// 2. 先找出最大值
/// 3. 再找出小于最大值的最大值
pub fn find_second_largest(numbers: &[i32]) -> Option<i32> {
    unimplemented!()
}

/// TODO: 找出数组中第一个偶数
///
/// # 返回
/// - `Some(even)`: 如果找到偶数
/// - `None`: 如果没有偶数
pub fn find_first_even(numbers: &[i32]) -> Option<i32> {
    unimplemented!()
}

/// TODO: 获取数组的最后一个元素
///
/// # 返回
/// - `Some(last)`: 如果数组不为空
/// - `None`: 如果数组为空
pub fn last_element(numbers: &[i32]) -> Option<&i32> {
    unimplemented!()
}

/// 将 Option 转换为字符串表示
pub fn option_to_string(opt: Option<i32>) -> String {
    match opt {
        Some(v) => format!("值为: {}", v),
        None => "没有值".to_string(),
    }
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
}
