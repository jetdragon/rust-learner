// examples/advanced.rs
// 高级并发演示

use module_09_concurrency::{
    atomic_increment_from_multiple_threads, barrier_example, parallel_search, parallel_sum,
    AtomicCounter,
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== 高级并发演示 ===\n");

    // 1. 原子计数器
    println!("1. 原子计数器:");
    let counter = AtomicCounter::new();
    println!("  Initial: {}", counter.get());
    println!("  After increment: {}", counter.increment());
    println!("  After add(5): {}", counter.add(5));

    // 2. 多线程原子操作
    println!("\n2. 多线程原子操作:");
    let result = atomic_increment_from_multiple_threads(4, 25);
    println!("  Final count: {}", result);

    // 3. AtomicBool 标志
    println!("\n3. AtomicBool 标志:");
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);

    let handle = thread::spawn(move || {
        let mut count = 0;
        while running_clone.load(Ordering::SeqCst) {
            count += 1;
            thread::sleep(Duration::from_millis(10));
            if count >= 3 {
                break;
            }
        }
        println!("  Thread finished after {} iterations", count);
    });

    thread::sleep(Duration::from_millis(50));
    running.store(false, Ordering::SeqCst);
    handle.join().unwrap();

    // 4. 屏障同步
    println!("\n4. 屏障同步:");
    barrier_example(3);

    // 5. 并行求和
    println!("\n5. 并行求和:");
    let numbers: Vec<i32> = (1..=100).collect();
    let sum = parallel_sum(numbers);
    println!("  Sum of 1..100 = {}", sum);

    // 6. 并行搜索
    println!("\n6. 并行搜索:");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let target = 7;
    match parallel_search(numbers, target) {
        Some(index) => println!("  Found {} at index {}", target, index),
        None => println!("  {} not found", target),
    }

    // 7. 条件变量模式
    println!("\n7. 条件变量模式:");
    let pair: Arc<Mutex<(i32, i32)>> = Arc::new(Mutex::new((0, 0)));
    let pair_clone = Arc::clone(&pair);

    let handle = thread::spawn(move || {
        let mut pair = pair_clone.lock().unwrap();
        pair.0 = 10;
        println!("  Thread: set pair.0 to {}", pair.0);
    });

    thread::sleep(Duration::from_millis(10));
    let mut pair = pair.lock().unwrap();
    pair.1 = 20;
    println!("  Main: set pair.1 to {}", pair.1);

    handle.join().unwrap();
    println!("  Final pair: {:?}", *pair);

    // 8. Once 单次初始化
    println!("\n8. Once 单次初始化:");
    use std::sync::Once;
    static INIT: Once = Once::new();
    let mut result = 0;

    for i in 0..3 {
        INIT.call_once(|| {
            println!("  Initializing...");
            result = 42;
        });
        println!("  Iteration {}: result = {}", i, result);
    }

    // 9. 线程本地存储
    println!("\n9. 线程本地存储:");
    use std::cell::RefCell;
    thread_local! {
        static THREAD_LOCAL_DATA: RefCell<Vec<i32>> = RefCell::new(Vec::new());
    }

    let handle1 = thread::spawn(|| {
        THREAD_LOCAL_DATA.with_borrow_mut(|data| {
            data.push(1);
            data.push(2);
        });
        println!(
            "  Thread 1: {:?}",
            THREAD_LOCAL_DATA.with_borrow(|data| data.clone())
        );
    });

    let handle2 = thread::spawn(|| {
        THREAD_LOCAL_DATA.with_borrow_mut(|data| {
            data.push(10);
            data.push(20);
        });
        println!(
            "  Thread 2: {:?}",
            THREAD_LOCAL_DATA.with_borrow(|data| data.clone())
        );
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    // 10. 超时模式
    println!("\n10. 超时模式:");
    let (tx, rx) = std::sync::mpsc::channel();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(200));
        tx.send("Delayed").unwrap();
    });

    match rx.recv_timeout(Duration::from_millis(100)) {
        Ok(msg) => println!("  Got: {}", msg),
        Err(_) => println!("  Timeout (100ms)"),
    }

    match rx.recv_timeout(Duration::from_millis(200)) {
        Ok(msg) => println!("  Got: {}", msg),
        Err(_) => println!("  Timeout (200ms)"),
    }

    // 11. 资源池模式
    println!("\n11. 资源池模式:");
    let pool: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(vec![1, 2, 3]));
    let mut handles = vec![];

    for i in 0..3 {
        let pool_clone = Arc::clone(&pool);
        let handle = thread::spawn(move || {
            let mut resources = pool_clone.lock().unwrap();
            if let Some(resource) = resources.pop() {
                println!("  Thread {} acquired resource: {}", i, resource);
                thread::sleep(Duration::from_millis(50));
                println!("  Thread {} releasing resource: {}", i, resource);
                resources.push(resource);
            } else {
                println!("  Thread {} no resource available", i);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // 12. 并行迭代器模式
    println!("\n12. 并行迭代器模式:");
    let data = vec![1, 2, 3, 4, 5];
    let mid = data.len() / 2;
    let left = data[..mid].to_vec();
    let right = data[mid..].to_vec();

    let handle = thread::spawn(move || left.into_iter().map(|x| x * 2).collect::<Vec<_>>());

    let right_result: Vec<_> = right.into_iter().map(|x| x * 2).collect();
    let left_result = handle.join().unwrap();

    let mut combined = left_result;
    combined.extend(right_result);
    println!("  Parallel map result: {:?}", combined);
}
