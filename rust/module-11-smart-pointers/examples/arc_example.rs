// examples/arc_example.rs
// Arc 原子引用计数演示

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Arc 原子引用计数演示 ===\n");

    // 1. Arc 基础
    println!("1. Arc 基础:");
    let a = Arc::new(5);
    let b = Arc::clone(&a);
    let c = Arc::clone(&a);

    println!("  a = {}", a);
    println!("  b = {}", b);
    println!("  c = {}", c);
    println!("  引用计数: {}", Arc::strong_count(&a)); // 3

    // 2. Arc 多线程共享
    println!("\n2. Arc 多线程共享:");
    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    let mut handles = vec![];

    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("  线程 {} 读取: {:?}", i, data_clone.as_slice());
            thread::sleep(Duration::from_millis(100));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // 3. Arc + Mutex 共享可变状态
    println!("\n3. Arc + Mutex 共享可变状态:");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..5 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
            println!("  线程 {} 增加计数器", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("  最终计数: {}", *counter.lock().unwrap());

    // 4. 实际应用：线程安全的缓存
    println!("\n4. 实际应用：线程安全缓存");
    use std::collections::HashMap;

    #[derive(Clone)]
    struct Cache {
        data: Arc<Mutex<HashMap<String, Vec<i32>>>>,
    }

    impl Cache {
        fn new() -> Self {
            Cache {
                data: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        fn get(&self, key: &str) -> Option<Vec<i32>> {
            let data = self.data.lock().unwrap();
            data.get(key).cloned()
        }

        fn set(&self, key: String, value: Vec<i32>) {
            let mut data = self.data.lock().unwrap();
            data.insert(key, value);
        }
    }

    let cache = Cache::new();
    cache.set("numbers".to_string(), vec![1, 2, 3, 4, 5]);

    let mut handles = vec![];
    for i in 0..3 {
        let cache = cache.clone();
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));
            if let Some(data) = cache.get("numbers") {
                println!("  线程 {} 读取缓存: {:?}", i, data);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // 5. Rc vs Arc 性能对比
    println!("\n5. Rc vs Arc 性能对比:");
    use std::rc::Rc;
    use std::time::Instant;

    const N: usize = 100_000;

    // Rc 测试
    let start = Instant::now();
    let rc_data = Rc::new(42);
    let mut rc_clones = vec![];
    for _ in 0..N {
        rc_clones.push(Rc::clone(&rc_data));
    }
    let rc_time = start.elapsed();
    println!("  Rc clone {} 次: {:?}", N, rc_time);

    // Arc 测试
    let start = Instant::now();
    let arc_data = Arc::new(42);
    let mut arc_clones = vec![];
    for _ in 0..N {
        arc_clones.push(Arc::clone(&arc_data));
    }
    let arc_time = start.elapsed();
    println!("  Arc clone {} 次: {:?}", N, arc_time);

    let ratio = arc_time.as_nanos() as f64 / rc_time.as_nanos() as f64;
    println!("  Arc 比 Rc 慢约 {:.2} 倍", ratio);

    println!("\n  结论:");
    println!("    Rc 在单线程更快（无原子操作）");
    println!("    Arc 提供线程安全");
    println!("    根据是否跨线程选择");

    // 6. Weak 引用（避免循环引用）
    println!("\n6. Weak 引用基础:");
    use std::sync::Weak;

    let strong = Arc::new(5);
    let weak = Arc::downgrade(&strong);

    println!("  Strong 引用计数: {}", Arc::strong_count(&strong));
    println!("  Weak 引用存在: {}", weak.upgrade().is_some());

    drop(strong);
    println!("  drop strong 后...");
    println!("  Weak 引用存在: {}", weak.upgrade().is_some());
}
