//! # 练习 6: Result 类型使用
//!
//! **难度**: 中等
//! **预计时间**: 15 分钟
//!
//! **前置知识**:
//! - 枚举基础
//! - match 表达式
//! - 基本的错误处理概念
//!
//! **学习目标**:
//! - 理解 Result 类型的用途
//! - 学会处理可能失败的操作
//! - 掌握返回和传播错误

/// TODO: 安全的除法运算
///
/// # 参数
/// - `a`: 被除数
/// - `b`: 除数
///
/// # 返回
/// - `Ok(result)`: 除法成功的结果
/// - `Err(message)`: 除数为零时的错误信息
pub fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    unimplemented!()
}

/// TODO: 计算数组元素的平均值
///
/// # 返回
/// - `Ok(average)`: 计算成功
/// - `Err(message)`: 数组为空
pub fn average(numbers: &[i32]) -> Result<f64, String> {
    unimplemented!()
}

/// TODO: 解析字符串为整数
///
/// # 返回
/// - `Ok(value)`: 解析成功
/// - `Err(message)`: 解析失败
pub fn parse_integer(s: &str) -> Result<i32, String> {
    unimplemented!()
}

/// TODO: 查找数组中的元素
///
/// # 返回
/// - `Ok(index)`: 找到元素的索引
/// - `Err(message)`: 元素不存在
pub fn find_index(numbers: &[i32], target: i32) -> Result<usize, String> {
    unimplemented!()
}

/// 执行除法并打印结果
pub fn print_divide_result(a: f64, b: f64) {
    match safe_divide(a, b) {
        Ok(result) => println!("{} / {} = {}", a, b, result),
        Err(e) => println!("错误: {}", e),
    }
}

fn main() {
    println!("=== Result 类型使用 ===\n");

    // 安全除法
    println!("--- 安全除法 ---");
    print_divide_result(10.0, 2.0);
    print_divide_result(5.0, 0.0);

    // 平均值
    println!("\n--- 平均值计算 ---");
    let numbers1 = vec![1, 2, 3, 4, 5];
    let numbers2: Vec<i32> = vec![];

    match average(&numbers1) {
        Ok(avg) => println!("平均值: {:.2}", avg),
        Err(e) => println!("错误: {}", e),
    }

    match average(&numbers2) {
        Ok(avg) => println!("平均值: {:.2}", avg),
        Err(e) => println!("错误: {}", e),
    }

    // 解析整数
    println!("\n--- 解析整数 ---");
    match parse_integer("42") {
        Ok(n) => println!("解析成功: {}", n),
        Err(e) => println!("解析失败: {}", e),
    }

    match parse_integer("abc") {
        Ok(n) => println!("解析成功: {}", n),
        Err(e) => println!("解析失败: {}", e),
    }

    // 查找索引
    println!("\n--- 查找索引 ---");
    let numbers = vec![10, 20, 30, 40, 50];

    match find_index(&numbers, 30) {
        Ok(idx) => println!("找到 30 在索引: {}", idx),
        Err(e) => println!("错误: {}", e),
    }

    match find_index(&numbers, 99) {
        Ok(idx) => println!("找到 99 在索引: {}", idx),
        Err(e) => println!("错误: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_divide_ok() {
        assert_eq!(safe_divide(10.0, 2.0), Ok(5.0));
        assert_eq!(safe_divide(7.0, 2.0), Ok(3.5));
    }

    #[test]
    fn test_safe_divide_zero() {
        assert!(safe_divide(10.0, 0.0).is_err());
        assert!(safe_divide(0.0, 0.0).is_err());
    }

    #[test]
    fn test_safe_divide_negative() {
        assert_eq!(safe_divide(-10.0, 2.0), Ok(-5.0));
        assert_eq!(safe_divide(10.0, -2.0), Ok(-5.0));
    }

    #[test]
    fn test_average_ok() {
        assert_eq!(average(&[1, 2, 3, 4, 5]), Ok(3.0));
        assert_eq!(average(&[10, 20]), Ok(15.0));
    }

    #[test]
    fn test_average_empty() {
        assert!(average(&[]).is_err());
    }

    #[test]
    fn test_average_single() {
        assert_eq!(average(&[42]), Ok(42.0));
    }

    #[test]
    fn test_parse_integer_ok() {
        assert_eq!(parse_integer("42"), Ok(42));
        assert_eq!(parse_integer("0"), Ok(0));
        assert_eq!(parse_integer("-10"), Ok(-10));
    }

    #[test]
    fn test_parse_integer_err() {
        assert!(parse_integer("abc").is_err());
        assert!(parse_integer("12.5").is_err());
        assert!(parse_integer("").is_err());
    }

    #[test]
    fn test_find_index_ok() {
        let numbers = vec![10, 20, 30, 40, 50];
        assert_eq!(find_index(&numbers, 10), Ok(0));
        assert_eq!(find_index(&numbers, 30), Ok(2));
        assert_eq!(find_index(&numbers, 50), Ok(4));
    }

    #[test]
    fn test_find_index_err() {
        let numbers = vec![10, 20, 30];
        assert!(find_index(&numbers, 99).is_err());
        assert!(find_index(&[], 10).is_err());
    }

    #[test]
    fn test_find_index_duplicates() {
        let numbers = vec![1, 2, 3, 2, 1];
        // 应该返回第一个匹配的索引
        assert_eq!(find_index(&numbers, 2), Ok(1));
    }
}
