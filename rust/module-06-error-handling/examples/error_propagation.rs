// examples/error_propagation.rs
// 错误传播演示

use module_05_error_handling::{safe_divide, safe_sqrt, MathError};

fn main() {
    println!("=== 错误传播演示 ===\n");

    // 1. 显式错误传播
    println!("1. 显式错误传播 (match):");
    fn divide_explicit(a: i32, b: i32) -> Result<i32, MathError> {
        match safe_divide(a, b) {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        }
    }

    match divide_explicit(10, 2) {
        Ok(r) => println!("  结果: {}", r),
        Err(e) => println!("  错误: {}", e),
    }

    // 2. 使用 ? 操作符
    println!("\n2. 使用 ? 操作符:");
    fn divide_with_question(a: i32, b: i32) -> Result<i32, MathError> {
        safe_divide(a, b)?; // 自动传播错误
        Ok(42) // 这个值会被返回
    }

    match divide_with_question(10, 2) {
        Ok(r) => println!("  结果: {}", r),
        Err(e) => println!("  错误: {}", e),
    }

    // 3. ? 与不同错误类型
    println!("\n3. ? 与错误转换:");
    fn parse_and_divide_custom(a_str: &str, b_str: &str) -> Result<i32, String> {
        let a: i32 = a_str.parse().map_err(|_| "无法解析第一个数".to_string())?;
        let b: i32 = b_str.parse().map_err(|_| "无法解析第二个数".to_string())?;
        safe_divide(a, b).map_err(|e| e.to_string())
    }

    match parse_and_divide_custom("10", "2") {
        Ok(r) => println!("  结果: {}", r),
        Err(e) => println!("  错误: {}", e),
    }

    match parse_and_divide_custom("abc", "2") {
        Ok(r) => println!("  结果: {}", r),
        Err(e) => println!("  错误: {}", e),
    }

    // 4. 链式调用
    println!("\n4. 链式调用:");
    fn calculate(s: &str) -> Result<f64, String> {
        let n: f64 = s.parse().map_err(|_| "解析失败".to_string())?;
        let sqrt = safe_sqrt(n).map_err(|e| e.to_string())?;
        let result = sqrt * 2.0;
        Ok(result)
    }

    match calculate("16") {
        Ok(r) => println!("  结果: {}", r),
        Err(e) => println!("  错误: {}", e),
    }

    // 5. 多个可能的失败点
    println!("\n5. 多个可能的失败点:");
    fn multi_step_operation(input: &str) -> Result<i32, String> {
        // 步骤 1: 解析
        let n: i32 = input.parse().map_err(|_| "解析失败")?;

        // 步骤 2: 验证
        if n <= 0 {
            return Err("数字必须为正".to_string());
        }

        // 步骤 3: 计算
        let result = safe_divide(100, n).map_err(|e| e.to_string())?;

        Ok(result)
    }

    for input in ["10", "abc", "-5", "0"] {
        print!("  输入 '{}': ", input);
        match multi_step_operation(input) {
            Ok(r) => println!("成功 -> {}", r),
            Err(e) => println!("失败 -> {}", e),
        }
    }

    // 6. 使用 and_then 链接
    println!("\n6. 使用 and_then 链接:");
    fn parse(s: &str) -> Result<i32, String> {
        s.parse().map_err(|_| "解析失败".to_string())
    }

    fn double(n: i32) -> Result<i32, String> {
        Ok(n * 2)
    }

    let result = parse("5").and_then(double);
    println!("  parse(5).and_then(double): {:?}", result);

    let result = parse("abc").and_then(double);
    println!("  parse(abc).and_then(double): {:?}", result);

    // 7. 使用 or_else 提供备用方案
    println!("\n7. 使用 or_else 提供备用方案:");
    fn try_primary() -> Result<i32, String> {
        Err("主要方法失败".to_string())
    }

    fn fallback() -> Result<i32, String> {
        Ok(42)
    }

    let result = try_primary().or_else(|_| fallback());
    println!("  备用方案结果: {:?}", result);

    // 8. 错误上下文
    println!("\n8. 添加错误上下文:");
    fn divide_with_context(a: i32, b: i32) -> Result<i32, String> {
        safe_divide(a, b).map_err(|e| format!("除法失败 ({}/{}): {}", a, b, e))
    }

    match divide_with_context(10, 0) {
        Ok(r) => println!("  结果: {}", r),
        Err(e) => println!("  错误: {}", e),
    }

    // 9. 提前返回
    println!("\n9. 提前返回:");
    fn check_all_positive(nums: &[i32]) -> Result<(), String> {
        for n in nums {
            if *n <= 0 {
                return Err(format!("{} 不是正数", n));
            }
        }
        Ok(())
    }

    match check_all_positive(&[1, 2, 3]) {
        Ok(()) => println!("  全部是正数"),
        Err(e) => println!("  错误: {}", e),
    }

    match check_all_positive(&[1, -2, 3]) {
        Ok(()) => println!("  全部是正数"),
        Err(e) => println!("  错误: {}", e),
    }

    // 10. 从 Option 传播
    println!("\n10. 从 Option 传播:");
    fn get_or_error(opt: Option<i32>) -> Result<i32, String> {
        opt.ok_or_else(|| "值为空".to_string())
    }

    println!("  Some(5): {:?}", get_or_error(Some(5)));
    println!("  None: {:?}", get_or_error(None));

    // 11. 收集多个 Result
    println!("\n11. 收集多个 Result:");
    let results: Vec<Result<i32, String>> = vec![Ok(1), Ok(2), Err("错误".to_string())];
    println!("  results: {:?}", results);

    // 提取所有成功值
    let successful: Vec<_> = results.into_iter().filter_map(|r| r.ok()).collect();
    println!("  成功的值: {:?}", successful);
}
