// examples/result_enum.rs
// Result 类型演示

use module_03_structs_enums::safe_divide;
use std::num::ParseIntError;

fn main() {
    println!("=== Result 类型演示 ===\n");

    // Result 基础
    println!("1. Result 基础:");
    use module_03_structs_enums::safe_divide;

    let success = safe_divide(10.0, 2.0);
    let failure = safe_divide(10.0, 0.0);

    println!("  成功: {:?}", success);
    println!("  失败: {:?}", failure);

    // 使用 match
    println!("\n2. 使用 match:");
    match safe_divide(10.0, 2.0) {
        Ok(result) => println!("  结果: {}", result),
        Err(e) => println!("  错误: {}", e),
    }

    // ? 操作符
    println!("\n3. 使用 ? 操作符:");
    match parse_and_double("5") {
        Ok(result) => println!("  解析并翻倍: {}", result),
        Err(e) => println!("  错误: {}", e),
    }

    match parse_and_double("abc") {
        Ok(result) => println!("  结果: {}", result),
        Err(e) => println!("  错误: {}", e),
    }

    // 组合 Result
    println!("\n4. 组合多个 Result:");
    match parse_and_divide("10", "2") {
        Ok(result) => println!("  10 / 2 = {}", result),
        Err(e) => println!("  错误: {}", e),
    }

    match parse_and_divide("10", "0") {
        Ok(result) => println!("  结果: {}", result),
        Err(e) => println!("  错误: {}", e),
    }

    // unwrap_or 提供默认值
    println!("\n5. 提供默认值:");
    let result = safe_divide(10.0, 0.0).unwrap_or(0.0);
    println!("  失败时返回 0: {}", result);

    // unwrap_or_else
    let result = safe_divide(10.0, 0.0).unwrap_or_else(|e| {
        println!("  发生错误: {}", e);
        0.0
    });
    println!("  使用自定义处理: {}", result);

    // map 转换值
    println!("\n6. 使用 map:");
    let result = safe_divide(10.0, 2.0).map(|x| x * 2.0);
    println!("  结果翻倍: {:?}", result);
}

// 使用 ? 简化错误传播
fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
    let parsed: i32 = s.parse()?;  // 如果失败，提前返回 Err
    Ok(parsed * 2)
}

// 组合多个 Result
fn parse_and_divide(a_str: &str, b_str: &str) -> Result<f64, String> {
    let a: f64 = a_str.parse().map_err(|_| "无法解析第一个数".to_string())?;
    let b: f64 = b_str.parse().map_err(|_| "无法解析第二个数".to_string())?;
    safe_divide(a, b)
}
