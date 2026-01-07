//! # 练习 5: 链式操作解答
//!
//! **解答要点:**
//! 1. 使用 and_then() 进行 Result/Option 的链式调用
//! 2. 使用 map() 在成功时转换值
//! 3. 组合多个操作构建处理管道

/// 如果 n 为正数，返回 2*n，否则返回错误
pub fn double_if_positive(n: i32) -> Result<i32, String> {
    if n > 0 {
        Ok(n * 2)
    } else {
        Err(format!("{} 不是正数", n))
    }
}

/// 将 Option 转为 Result，然后调用 double_if_positive
pub fn validate_and_double(v: Option<i32>) -> Result<i32, String> {
    // ok_or() 将 Option 转为 Result
    // and_then() 只在成功时调用下一个函数
    v.ok_or_else(|| "没有值".to_string())
        .and_then(double_if_positive)
}

// /// 使用 match 的替代实现
// pub fn validate_and_double_match(v: Option<i32>) -> Result<i32, String> {
//     match v {
//         Some(n) => double_if_positive(n),
//         None => Err("没有值".to_string()),
//     }
// }

/// 链式调用示例
///
/// 这个函数展示了如何组合多个操作：
/// 1. 过滤掉 None 值
/// 2. 验证并处理每个值
/// 3. 收集所有结果
pub fn process_numbers(numbers: Vec<Option<i32>>) -> Result<Vec<i32>, String> {
    numbers
        .into_iter()
        .map(|opt| opt.ok_or_else(|| "遇到 None 值".to_string()))
        .map(|res| res.and_then(double_if_positive))
        .collect()
}

// /// 使用迭代器方法的另一个版本
// pub fn process_numbers_iter(numbers: Vec<Option<i32>>) -> Result<Vec<i32>, String> {
//     let mut results = Vec::new();
//     for opt in numbers {
//         let n = opt.ok_or_else(|| "遇到 None 值".to_string())?;
//         let doubled = double_if_positive(n)?;
//         results.push(doubled);
//     }
//     Ok(results)
// }

fn main() {
    println!("=== 验证并翻倍 ===");
    println!("{:?}", validate_and_double(Some(5)));
    println!("{:?}", validate_and_double(Some(-1)));
    println!("{:?}", validate_and_double(None));

    println!("\n=== 处理数字列表 ===");
    let numbers = vec![Some(1), Some(2), None, Some(3)];
    println!("{:?}", process_numbers(numbers));

    let numbers = vec![Some(1), Some(2), Some(3)];
    println!("{:?}", process_numbers(numbers));

    // 链式调用示例
    println!("\n=== 链式调用演示 ===");
    let result = Some(5)
        .ok_or("Error")
        .map_err(|e| e.to_string())
        .and_then(double_if_positive);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_and_double() {
        assert_eq!(validate_and_double(Some(5)), Ok(10));
        assert!(validate_and_double(Some(-1)).is_err());
        assert!(validate_and_double(None).is_err());
    }

    #[test]
    fn test_process_numbers() {
        let numbers = vec![Some(1), Some(2), Some(3)];
        assert_eq!(process_numbers(numbers), Ok(vec![2, 4, 6]));

        let numbers = vec![Some(1), Some(-2), Some(3)];
        assert!(process_numbers(numbers).is_err());
    }
}
