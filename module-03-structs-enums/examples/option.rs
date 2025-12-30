// examples/option.rs
// Option 类型演示

fn main() {
    println!("=== Option 类型演示 ===\n");

    // 创建 Option 值
    println!("1. 创建 Option:");
    let some_number: Option<i32> = Some(5);
    let some_string: Option<&str> = Some("string");
    let absent_number: Option<i32> = None;

    println!("  Some(5): {:?}", some_number);
    println!("  Some(\"string\"): {:?}", some_string);
    println!("  None: {:?}", absent_number);

    // 使用 match 处理 Option
    println!("\n2. 使用 match:");
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("  plus_one(Some(5)) = {:?}", six);
    println!("  plus_one(None) = {:?}", none);

    // 使用 if let 简化
    println!("\n3. 使用 if let:");
    if let Some(n) = some_number {
        println!("  值是: {}", n);
    }

    // 组合 Option
    println!("\n4. 组合 Option:");
    let result = some_number.and_then(|n| if n > 3 { Some(n * 2) } else { None });
    println!("  大于 3 则翻倍: {:?}", result);

    // 提供默认值
    println!("\n5. 提供默认值:");
    let default = absent_number.unwrap_or(0);
    println!("  None 的默认值: {}", default);

    // Option 与数组
    println!("\n6. 在数组中查找:");
    let numbers = vec![1, 2, 3, 4, 5];
    let found = find(&numbers, &3);
    let not_found = find(&numbers, &10);

    println!("  找到 3: {:?}", found);
    println!("  找到 10: {:?}", not_found);
}

// 函数返回 Option
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// 在数组中查找元素
fn find<'a>(slice: &'a [i32], target: &i32) -> Option<&'a i32> {
    for item in slice {
        if item == target {
            return Some(item);
        }
    }
    None
}
