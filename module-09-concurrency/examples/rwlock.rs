// examples/rwlock.rs
// RwLock 读写锁演示

use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== RwLock 读写锁演示 ===\n");

    // 1. RwLock 基础
    println!("1. RwLock 基础:");
    let lock = RwLock::new(5);

    // 读锁
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        println!("  r1: {}, r2: {}", r1, r2);
        println!("  多个读锁可以共存");
    }

    // 写锁（排他）
    {
        let mut w = lock.write().unwrap();
        *w = 10;
        println!("  写入后: {}", *w);
    }

    println!("  写锁释放后: {}", *lock.read().unwrap());

    // 2. 多读者并发
    println!("\n2. 多读者并发:");
    let data = Arc::new(RwLock::new(vec![1, 2, 3, 4, 5]));
    let mut handles = vec![];

    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let reader = data_clone.read().unwrap();
            println!("  读者 {} 读取数据: {:?}", i, *reader);
            thread::sleep(Duration::from_millis(100));
            println!("  读者 {} 完成", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // 3. 写者优先
    println!("\n3. 写者 vs 读者:");
    let data = Arc::new(RwLock::new(0));
    let mut handles = vec![];

    // 创建读者
    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || loop {
            let reader = data_clone.read().unwrap();
            println!("  读者 {}: 读取到 {}", i, *reader);
            thread::sleep(Duration::from_millis(50));
            if *reader > 5 {
                break;
            }
        });
        handles.push(handle);
    }

    // 创建写者
    let data_clone = Arc::clone(&data);
    let writer = thread::spawn(move || {
        for i in 1..=6 {
            let mut writer = data_clone.write().unwrap();
            *writer = i;
            println!("  写者: 写入 {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });
    handles.push(writer);

    for handle in handles {
        handle.join().unwrap();
    }

    // 4. 实际应用：缓存
    println!("\n4. 实际应用：线程安全缓存");
    let cache = Cache::new();

    // 模拟多个线程读取缓存
    for i in 0..5 {
        let cache = cache.clone();
        thread::spawn(move || {
            for j in 0..3 {
                let key = format!("key_{}", j);
                if let Some(value) = cache.get(&key) {
                    println!("  线程 {} 读取 {}: {}", i, key, value);
                } else {
                    println!("  线程 {} 未找到 {}", i, key);
                }
                thread::sleep(Duration::from_millis(50));
            }
        });
    }

    // 模拟更新缓存
    for i in 0..3 {
        let cache = cache.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));
            let key = format!("key_{}", i);
            let value = format!("value_{}", i);
            cache.set(key.clone(), value.clone());
            println!("  写入: {} = {}", key, value);
        });
    }

    thread::sleep(Duration::from_secs(1));

    // 5. RwLock vs Mutex 性能对比
    println!("\n5. RwLock vs Mutex 性能对比 (读多写少)");
    use std::time::Instant;

    const N: usize = 100_000;

    // RwLock 测试
    let rwlock_data = Arc::new(RwLock::new(0));
    let start = Instant::now();

    let mut handles = vec![];
    for _ in 0..4 {
        let data = Arc::clone(&rwlock_data);
        let handle = thread::spawn(move || {
            for i in 0..N {
                if i % 100 == 0 {
                    // 写操作
                    let mut w = data.write().unwrap();
                    *w += 1;
                } else {
                    // 读操作
                    let _r = data.read().unwrap();
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let rwlock_time = start.elapsed();
    println!("  RwLock: {:?}", rwlock_time);

    // Mutex 测试
    let mutex_data = Arc::new(std::sync::Mutex::new(0));
    let start = Instant::now();

    let mut handles = vec![];
    for _ in 0..4 {
        let data = Arc::clone(&mutex_data);
        let handle = thread::spawn(move || {
            for i in 0..N {
                if i % 100 == 0 {
                    let mut m = data.lock().unwrap();
                    *m += 1;
                } else {
                    let _m = data.lock().unwrap();
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mutex_time = start.elapsed();
    println!("  Mutex: {:?}", mutex_time);

    let ratio = mutex_time.as_nanos() as f64 / rwlock_time.as_nanos() as f64;
    println!("  Mutex 比 RwLock 慢约 {:.2} 倍", ratio);

    println!("\n  结论:");
    println!("    读多写少场景，RwLock 性能更好");
    println!("    写操作时，RwLock 需要等待所有读锁释放");
    println!("    根据读写比例选择合适的锁");
}

#[derive(Clone)]
struct Cache {
    data: Arc<RwLock<std::collections::HashMap<String, String>>>,
}

impl Cache {
    fn new() -> Self {
        Cache {
            data: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    fn get(&self, key: &str) -> Option<String> {
        let data = self.data.read().unwrap();
        data.get(key).cloned()
    }

    fn set(&self, key: String, value: String) {
        let mut data = self.data.write().unwrap();
        data.insert(key, value);
    }
}
