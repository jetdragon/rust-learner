// examples/basics.rs
// 线程基础演示

use module_09_concurrency::{calculate_in_thread, get_thread_id, sleep_ms, spawn_and_wait};
use std::thread;

fn main() {
    println!("=== 线程基础演示 ===\n");

    // 1. 创建线程
    println!("1. 创建线程:");
    let handle = thread::spawn(|| {
        println!("  Hello from spawned thread!");
    });
    handle.join().unwrap();

    // 2. 传递参数
    println!("\n2. 传递参数:");
    let handle = thread::spawn(|| {
        for i in 1..=3 {
            println!("  Spawned thread: {}", i);
            sleep_ms(10);
        }
    });

    for i in 1..=2 {
        println!("  Main thread: {}", i);
        sleep_ms(10);
    }
    handle.join().unwrap();

    // 3. move 闭包
    println!("\n3. move 闭包:");
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("  Vector in thread: {:?}", v);
    });
    handle.join().unwrap();

    // 4. 获取返回值
    println!("\n4. 获取返回值:");
    let result = calculate_in_thread(10, 20);
    println!("  10 + 20 = {}", result);

    // 5. 获取线程 ID
    println!("\n5. 获取线程 ID:");
    let main_id = get_thread_id();
    println!("  Main thread ID: {:?}", main_id);

    let handle = thread::spawn(|| {
        let spawned_id = get_thread_id();
        println!("  Spawned thread ID: {:?}", spawned_id);
    });
    handle.join().unwrap();

    // 6. 多个线程
    println!("\n6. 多个线程:");
    let mut handles = vec![];

    for i in 1..=3 {
        let handle = thread::spawn(move || {
            println!("  Thread {} starting", i);
            sleep_ms(50);
            println!("  Thread {} finishing", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // 7. spawn_and_wait 演示
    println!("\n7. spawn_and_wait 演示:");
    spawn_and_wait();

    // 8. 命名线程
    println!("\n8. 命名线程:");
    let handle = thread::Builder::new()
        .name("MyThread".to_string())
        .spawn(|| {
            println!("  Named thread running");
        })
        .unwrap();

    handle.join().unwrap();

    // 9. 线程栈大小
    println!("\n9. 自定义栈大小:");
    let handle = thread::Builder::new()
        .stack_size(1024 * 1024) // 1MB
        .spawn(|| {
            println!("  Thread with custom stack size");
        })
        .unwrap();

    handle.join().unwrap();

    // 10. 当前线程
    println!("\n10. 当前线程信息:");
    println!(
        "  Available parallelism: {:?}",
        thread::available_parallelism()
    );
    println!("  Current thread: {:?}", thread::current().name());
}
