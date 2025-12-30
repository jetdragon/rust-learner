// examples/shared_state.rs
// 共享状态演示

use module_09_concurrency::{increment_from_multiple_threads, Cache, ThreadSafeCounter};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== 共享状态演示 ===\n");

    // 1. Mutex 基础
    println!("1. Mutex 基础:");
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("  Value: {}", *m.lock().unwrap());

    // 2. Arc + Mutex
    println!("\n2. Arc + Mutex:");
    let counter = Arc::new(ThreadSafeCounter::new());
    counter.increment();
    counter.increment();
    println!("  Counter: {}", counter.get());

    // 3. 多线程共享
    println!("\n3. 多线程共享:");
    let result = increment_from_multiple_threads(4, 25);
    println!("  Final count: {}", result);

    // 4. Mutex 内部可变性
    println!("\n4. Mutex 内部可变性:");
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let data_clone = Arc::clone(&data);

    thread::spawn(move || {
        let mut data = data_clone.lock().unwrap();
        data.push(4);
    });

    thread::sleep(Duration::from_millis(50));
    println!("  Data: {:?}", *data.lock().unwrap());

    // 5. 结构体中的 Mutex
    println!("\n5. 结构体中的 Mutex:");
    let counter1 = Arc::new(ThreadSafeCounter::new());
    let counter2 = Arc::clone(&counter1);

    let counter1_clone = Arc::clone(&counter1);
    thread::spawn(move || {
        for _ in 0..5 {
            counter1_clone.increment();
        }
    });

    thread::spawn(move || {
        for _ in 0..3 {
            counter2.increment();
        }
    })
    .join()
    .unwrap();

    thread::sleep(Duration::from_millis(50));
    println!("  Counter: {}", counter1.get());

    // 6. RwLock 缓存
    println!("\n6. RwLock 缓存:");
    let cache = Arc::new(Cache::new(String::from("Initial Value")));

    let reader1 = {
        let cache_clone = Arc::clone(&cache);
        thread::spawn(move || {
            println!("  Reader 1: {}", cache_clone.read());
        })
    };

    let reader2 = {
        let cache_clone = Arc::clone(&cache);
        thread::spawn(move || {
            println!("  Reader 2: {}", cache_clone.read());
        })
    };

    reader1.join().unwrap();
    reader2.join().unwrap();

    // 7. 多读者单写者
    println!("\n7. 多读者单写者:");
    let cache_clone1 = Arc::clone(&cache);
    let cache_clone2 = Arc::clone(&cache);
    let cache_clone3 = Arc::clone(&cache);
    let handles = vec![
        thread::spawn(move || {
            println!("  Reader 1 starting");
            thread::sleep(Duration::from_millis(10));
            println!("  Reader 1: {}", cache_clone1.read());
        }),
        thread::spawn(move || {
            println!("  Reader 2 starting");
            thread::sleep(Duration::from_millis(10));
            println!("  Reader 2: {}", cache_clone2.read());
        }),
        thread::spawn(move || {
            println!("  Writer starting");
            thread::sleep(Duration::from_millis(5));
            cache_clone3.write(String::from("Updated value"));
            println!("  Writer finished");
        }),
    ];

    for handle in handles {
        handle.join().unwrap();
    }

    // 8. 复杂的共享状态
    println!("\n8. 复杂的共享状态:");
    let shared_data = Arc::new(Mutex::new(vec![]));
    let mut handles = vec![];

    for i in 0..3 {
        let data_clone = Arc::clone(&shared_data);
        let handle = thread::spawn(move || {
            let mut data = data_clone.lock().unwrap();
            data.push(i);
            println!("  Thread {} pushed {}", i, i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("  Final data: {:?}", *shared_data.lock().unwrap());

    // 9. 避免死锁
    println!("\n9. 避免死锁 (按顺序获取锁):");
    let mutex1 = Arc::new(Mutex::new(1));
    let mutex2 = Arc::new(Mutex::new(2));

    let m1_clone = Arc::clone(&mutex1);
    let m2_clone = Arc::clone(&mutex2);

    thread::spawn(move || {
        let _lock1 = m1_clone.lock().unwrap();
        thread::sleep(Duration::from_millis(10));
        let _lock2 = m2_clone.lock().unwrap();
        println!("  Thread 1 acquired both locks");
    })
    .join()
    .unwrap();

    // 10. 作用域中的锁
    println!("\n10. 作用域中的锁:");
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    {
        let mut data = data.lock().unwrap();
        data.push(4);
        println!("  In scope: {:?}", *data);
    } // 锁在这里释放

    println!("  After scope: {:?}", *data.lock().unwrap());
}
