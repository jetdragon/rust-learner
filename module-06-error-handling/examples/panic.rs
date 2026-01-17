// examples/panic.rs
// panic 和不可恢复错误演示

fn main() {
    println!("=== panic 和不可恢复错误演示 ===\n");

    // 1. 显式 panic!
    println!("1. 显式 panic! (注释掉避免程序退出):");
    // panic!("发生严重错误!");
    println!("  panic! 会立即终止程序");

    // 2. unwrap panic
    println!("\n2. unwrap 可能 panic:");
    let some_value = Some(10);
    println!("  Some(10).unwrap() = {}", some_value.unwrap());

    let _none_value: Option<i32> = None;
    // println!("  None.unwrap() = {}", none_value.unwrap()); // 会 panic!
    println!("  None.unwrap() 会 panic!");

    // 3. expect panic
    println!("\n3. expect 提供更好的错误消息:");
    let some_value = Some(10);
    println!(
        "  Some(10).expect('值不存在') = {}",
        some_value.expect("值不存在")
    );

    // let none_value: Option<i32> = None;
    // none_value.expect("值不应该为空"); // 会打印: '值不应该为空'
    println!("  None.expect('值不应该为空') 会 panic 并显示该消息");

    // 4. assert! panic
    println!("\n4. assert! 在测试中 panic:");
    let x = 5;
    assert!(x > 0, "x 必须大于 0");
    println!("  assert!(x > 0) 通过了");

    // assert!(x < 0, "x 必须小于 0"); // 会 panic!
    println!("  assert!(x < 0) 会 panic");

    // 5. assert_eq!
    println!("\n5. assert_eq! 比较值:");
    let a = 5;
    let b = 5;
    assert_eq!(a, b);
    println!("  assert_eq!(5, 5) 通过了");

    // assert_eq!(a, 10); // 会 panic!
    println!("  assert_eq!(5, 10) 会 panic");

    // 6. unreachable!()
    println!("\n6. unreachable!() 标记不可能的代码:");
    fn divide_by_two(x: i32) -> i32 {
        match x % 2 {
            0 => x / 2,
            1 => x / 2 + 1,
            _ => unreachable!(), // 这个分支永远无法到达
        }
    }

    println!("  divide_by_two(5) = {}", divide_by_two(5));

    // 7. unimplemented!()
    println!("\n7. unimplemented!() 标记未实现的代码:");
    // fn future_function() -> i32 {
    //     unimplemented!()
    // }
    println!("  unimplemented!() 提醒代码未完成");

    // 8. todo!()
    println!("\n8. todo!() 标记待办事项:");
    // fn todo_function() -> i32 {
    //     todo!("实现这个函数")
    // }
    println!("  todo!() 标记需要完成的代码");

    // 9. panic 捕获
    println!("\n9. 捕获 panic (使用 std::panic::catch_unwind):");
    use std::panic;

    let result = panic::catch_unwind(|| {
        panic!("捕获到的 panic!");
    });

    match result {
        Ok(_) => println!("  没有 panic"),
        Err(_) => println!("  捕获到 panic"),
    }

    // 10. panic 后的清理
    println!("\n10. panic 会调用 Drop:");
    struct DropDemo {
        name: &'static str,
    }

    impl Drop for DropDemo {
        fn drop(&mut self) {
            println!("  清理: {}", self.name);
        }
    }

    let _demo = DropDemo { name: "资源" };
    // panic!("发生错误"); // 即使 panic 也会调用 drop
    println!("  (注释掉的 panic 也会调用 Drop)");

    // 11. 条件 panic
    println!("\n11. 条件 panic:");
    let score = 150;
    if score > 100 {
        // panic!("分数不能超过 100");
        println!("  (注释掉) panic: 分数不能超过 100");
    }

    // 12. 最佳实践
    println!("\n12. panic 最佳实践:");
    println!("  - 只在真正无法恢复时使用 panic!");
    println!("  - 在示例和测试代码中使用 unwrap/expect");
    println!("  - 在库代码中优先返回 Result");
    println!("  - 使用 expect 而不是 unwrap 提供错误消息");
    println!("  - 使用 assert! 进行内部不变量检查");

    // 13. Option/Result vs panic
    println!("\n13. 何时使用 panic:");
    println!("  panic!: 示例、测试、不可能的情况");
    println!("  Result: 可能失败的操作");
    println!("  Option: 值可能缺失");
    println!("  assert!: 内部不变量检查");

    // 14. panic 和线程
    println!("\n14. panic 在线程中:");
    use std::thread;

    let handle = thread::spawn(|| {
        panic!("子线程 panic!");
    });

    match handle.join() {
        Ok(_) => println!("  线程正常完成"),
        Err(_) => println!("  线程 panic 了"),
    }
}
