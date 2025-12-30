// examples/if_let.rs
// if let 和 while let 简化演示

use module_04_patterns::{second, second_if_let, List};

fn main() {
    println!("=== if let 和 while let ===\n");

    // 1. if let 基础
    println!("1. if let 基础:");
    let some_value = Some(7);
    if let Some(n) = some_value {
        println!("  值是: {}", n);
    }

    let none_value: Option<i32> = None;
    if let Some(n) = none_value {
        println!("  值是: {}", n);
    } else {
        println!("  没有值");
    }

    // 2. 完整 match vs if let
    println!("\n2. 完整 match vs if let:");
    let value = Some(5);

    println!("  完整 match:");
    match value {
        Some(i) => println!("    值是: {}", i),
        None => println!("    没有值"),
    }

    println!("  if let 简化:");
    if let Some(i) = value {
        println!("    值是: {}", i);
    }

    // 3. when to use if let
    println!("\n3. 何时使用 if let:");
    println!("  - 只关心一种情况");
    println!("  - 其他情况不需要特殊处理");

    // 4. if let with else
    println!("\n4. if let with else:");
    let maybe_number = Some(10);
    if let Some(n) = maybe_number {
        println!("  有数字: {}", n);
    } else {
        println!("  没有数字");
    }

    // 5. 嵌套 if let
    println!("\n5. 嵌套 if let:");
    let result: Result<Result<i32, String>, String> = Ok(Ok(5));
    if let Ok(Ok(n)) = result {
        println!("  嵌套的 Ok 值: {}", n);
    }

    // 6. while let 循环
    println!("\n6. while let 循环:");
    let mut optional = Some(0);
    let mut count = 0;
    while let Some(i) = optional {
        if i < 5 {
            println!("  迭代 {}: {}", count, i);
            optional = Some(i + 1);
        } else {
            optional = None;
        }
        count += 1;
    }

    // 7. while let 遍历链表
    println!("\n7. while let 遍历链表:");
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
    let mut current = &list;
    print!("  链表: ");
    while let List::Cons(item, rest) = current {
        print!("{} -> ", item);
        current = rest;
    }
    println!("Nil");

    // 8. if let vs match - 性能考虑
    println!("\n8. 性能考虑:");
    println!("  - if let: 只匹配一个模式，更快");
    println!("  - match: 需要检查所有模式");

    // 9. 实际例子: second 函数
    println!("\n9. 实际例子 - second 函数:");
    let vec = vec![1, 2, 3, 4, 5];
    println!("  向量: {:?}", vec);
    println!("  second (match): {:?}", second(&vec));
    println!("  second_if_let (if let): {:?}", second_if_let(&vec));

    // 10. if let 用于错误处理
    println!("\n10. if let 用于错误处理:");
    let result: Result<i32, &str> = Ok(42);
    if let Ok(value) = result {
        println!("  成功: {}", value);
    }

    let error: Result<i32, &str> = Err("出错了");
    if let Err(msg) = error {
        println!("  错误: {}", msg);
    }

    // 11. let else (Rust 1.65+)
    println!("\n11. let else 模式:");
    let Some(value) = Some(10) else {
        println!("  没有值，提前返回");
        return;
    };
    println!("  有值: {}", value);

    // 12. if let 使用守卫
    println!("\n12. if let 使用守卫:");
    let some_number = Some(7);
    if let Some(n) = some_number {
        if n > 5 {
            println!("  {} 大于 5", n);
        }
    }
}
