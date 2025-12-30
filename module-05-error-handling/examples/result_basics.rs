// examples/result_basics.rs
// Result 基础演示

use module_05_error_handling::{option_to_result, safe_divide, safe_sqrt};

fn main() {
    println!("=== Result 基础演示 ===\n");

    // 1. Result 基础
    println!("1. Result 基础:");
    let success: Result<i32, &str> = Ok(200);
    let failure: Result<i32, &str> = Err("错误");

    println!("  成功: {:?}", success);
    println!("  失败: {:?}", failure);

    // 2. 使用 match 处理 Result
    println!("\n2. 使用 match 处理 Result:");
    match safe_divide(10, 2) {
        Ok(result) => println!("  结果: {}", result),
        Err(e) => println!("  错误: {}", e),
    }

    match safe_divide(10, 0) {
        Ok(result) => println!("  结果: {}", result),
        Err(e) => println!("  错误: {}", e),
    }

    // 3. unwrap 和 expect
    println!("\n3. unwrap 和 expect:");
    let ok_value: Result<i32, &str> = Ok(100);
    println!("  unwrap: {}", ok_value.unwrap());

    let err_value: Result<i32, &str> = Err("出错了");
    // println!("  unwrap: {}", err_value.unwrap()); // 会 panic!

    // expect 更安全，提供错误消息
    // let result = Err("错误").expect("这次失败了");

    // 4. unwrap_or 和 unwrap_or_default
    println!("\n4. unwrap_or 提供默认值:");
    let result: Result<i32, &str> = Err("错误");
    println!("  Err 时的默认值: {}", result.unwrap_or(0));

    let result: Result<i32, &str> = Ok(42);
    println!("  Ok 时的值: {}", result.unwrap_or(0));

    // unwrap_or_default 使用类型的默认值
    let result: Result<i32, &str> = Err("错误");
    println!("  默认值 (i32): {}", result.unwrap_or_default());

    let result: Result<String, &str> = Err("错误");
    println!("  默认值 (String): '{}'", result.unwrap_or_default());

    // 5. unwrap_or_else 懒惰计算
    println!("\n5. unwrap_or_else:");
    let result: Result<i32, &str> = Err("错误");
    let default = result.unwrap_or_else(|e| {
        println!("  计算默认值，因为: {}", e);
        0
    });
    println!("  结果: {}", default);

    // 6. map 转换成功值
    println!("\n6. map 转换成功值:");
    let result: Result<i32, &str> = Ok(5);
    let doubled = result.map(|x| x * 2);
    println!("  Ok(5) mapped: {:?}", doubled);

    let result: Result<i32, &str> = Err("错误");
    let doubled = result.map(|x| x * 2);
    println!("  Err mapped: {:?}", doubled);

    // 7. map_err 转换错误
    println!("\n7. map_err 转换错误:");
    let result: Result<i32, &str> = Err("IO错误");
    let mapped = result.map_err(|e| format!("严重错误: {}", e));
    println!("  错误映射: {:?}", mapped);

    // 8. Option 转换为 Result
    println!("\n8. Option 转换为 Result:");
    let some = Some(5);
    let none: Option<i32> = None;

    println!(
        "  Some(5) -> {:?}",
        option_to_result(some, "空".to_string())
    );
    println!("  None -> {:?}", option_to_result(none, "空".to_string()));

    // 9. 安全除法示例
    println!("\n9. 安全除法:");
    let tests = [(10, 2), (10, 0), (20, 4)];
    for (a, b) in tests {
        match safe_divide(a, b) {
            Ok(result) => println!("  {} / {} = {}", a, b, result),
            Err(e) => println!("  {} / {} 失败: {}", a, b, e),
        }
    }

    // 10. 安全平方根
    println!("\n10. 安全平方根:");
    let numbers = [9.0, 2.0, -1.0, 0.0];
    for n in numbers {
        match safe_sqrt(n) {
            Ok(result) => println!("  sqrt({}) = {}", n, result),
            Err(e) => println!("  sqrt({}) 失败: {}", n, e),
        }
    }

    // 11. 组合多个 Result
    println!("\n11. 组合操作:");
    fn parse_and_calc(s: &str) -> Result<i32, String> {
        let n: i32 = s.parse().map_err(|_| "解析失败".to_string())?;
        if n > 0 {
            Ok(n * 2)
        } else {
            Err("数字必须为正".to_string())
        }
    }

    for input in ["10", "abc", "-5", "0"] {
        match parse_and_calc(input) {
            Ok(result) => println!("  '{}' -> {}", input, result),
            Err(e) => println!("  '{}' 失败: {}", input, e),
        }
    }

    // 12. Result 是可比较的
    println!("\n12. Result 比较:");
    assert_eq!(Ok::<i32, ()>(5), Ok(5));
    assert_ne!(Ok::<i32, ()>(5), Ok(6));
    println!("  Ok(5) == Ok(5): true");
    println!("  Ok(5) != Ok(6): true");
}
