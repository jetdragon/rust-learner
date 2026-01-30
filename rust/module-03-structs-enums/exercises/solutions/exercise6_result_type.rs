//! # 练习 6 解答: Result 类型使用
//!
//! **提示**：请先尝试自己实现，再参考此解答！
//!
//! # 实现思路
//!
//! 1. Result 类型：表示可能失败的操作
//! 2. Ok(T)：操作成功，包含成功值
//! 3. Err(E)：操作失败，包含错误信息
//! 4. 错误处理：使用 match、? 运算符或组合子

/// 安全的除法运算
///
/// # 实现思路
/// 检查除数是否为零，返回相应的 Result
pub fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("除数不能为零"))
    } else {
        Ok(a / b)
    }
}

/// 计算数组元素的平均值
///
/// # 实现思路
/// 检查数组是否为空
pub fn average(numbers: &[i32]) -> Result<f64, String> {
    if numbers.is_empty() {
        Err(String::from("数组不能为空"))
    } else {
        let sum: i32 = numbers.iter().sum();
        Ok(sum as f64 / numbers.len() as f64)
    }
}

/// 解析字符串为整数
///
/// # 实现思路
/// 使用 parse() 方法的 ok_or_else 转换错误
pub fn parse_integer(s: &str) -> Result<i32, String> {
    s.parse().map_err(|_| format!("'{}' 不是有效的整数", s))
}

/// 查找数组中的元素
///
/// # 实现思路
/// 使用 position 方法找到索引
pub fn find_index(numbers: &[i32], target: i32) -> Result<usize, String> {
    numbers
        .iter()
        .position(|&x| x == target)
        .ok_or_else(|| format!("元素 {} 不存在于数组中", target))
}

/// 执行除法并打印结果
pub fn print_divide_result(a: f64, b: f64) {
    match safe_divide(a, b) {
        Ok(result) => println!("{} / {} = {}", a, b, result),
        Err(e) => println!("错误: {}", e),
    }
}

/// 使用 ? 运算符链接多个可能失败的操作
pub fn divide_and_multiply(a: f64, b: f64, c: f64) -> Result<f64, String> {
    let result1 = safe_divide(a, b)?;
    let result2 = safe_divide(result1, c)?;
    Ok(result2)
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

    // 链式操作
    println!("\n--- 链式操作 ---");
    match divide_and_multiply(100.0, 5.0, 2.0) {
        Ok(result) => println!("100 / 5 / 2 = {}", result),
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
        assert_eq!(find_index(&numbers, 2), Ok(1)); // 返回第一个匹配
    }

    #[test]
    fn test_divide_and_multiply() {
        assert_eq!(divide_and_multiply(100.0, 5.0, 2.0), Ok(10.0));
        assert!(divide_and_multiply(10.0, 0.0, 2.0).is_err());
    }

    #[test]
    fn test_result_combinators() {
        // 演示 Result 的组合子
        let result: Result<i32, String> = Ok(5);

        // map: 转换 Ok 值
        assert_eq!(result.map(|x| x * 2), Ok(10));

        // map_err: 转换 Err 值
        let err: Result<i32, String> = Err("error".to_string());
        assert_eq!(err.map_err(|e| e.len()), Err(5));
    }

    #[test]
    fn test_result_and_then() {
        // 使用 and_then 链接操作
        fn parse_then_double(s: &str) -> Result<i32, String> {
            s.parse::<i32>()
                .map_err(|_| "parse error".to_string())
                .and_then(|n| if n > 0 { Ok(n * 2) } else { Err("not positive".to_string()) })
        }

        assert_eq!(parse_then_double("5"), Ok(10));
        assert!(parse_then_double("-5").is_err());
        assert!(parse_then_double("abc").is_err());
    }

    #[test]
    fn test_is_ok_and_is_err() {
        assert!(safe_divide(10.0, 2.0).is_ok());
        assert!(!safe_divide(10.0, 2.0).is_err());
        assert!(safe_divide(10.0, 0.0).is_err());
        assert!(!safe_divide(10.0, 0.0).is_ok());
    }
}
