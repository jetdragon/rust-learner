//! # 并发编程
//!
//! 本模块演示 Rust 的并发编程特性。
//!
//! ## 模块内容
//!
//! - 线程创建和管理
//! - 消息传递 (Channel)
//! - 共享状态 (Mutex, Arc)
//! - 原子类型
//!
//! ## 示例
//!
//! ```
//! use module_09_concurrency::{spawn_and_wait, calculate_in_thread};
//!
//! let result = calculate_in_thread(10, 20);
//! assert_eq!(result, 30);
//!
//! spawn_and_wait();
//! ```
//!
//! 注意: 并发测试在某些环境下可能不稳定

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// ========== 线程基础 ==========

/// 在新线程中计算两个数的和
pub fn calculate_in_thread(a: i32, b: i32) -> i32 {
    let handle = thread::spawn(move || a + b);
    handle.join().unwrap()
}

/// 生成线程并等待完成
pub fn spawn_and_wait() {
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("Thread: {}", i);
            thread::sleep(Duration::from_millis(10));
        }
    });

    for i in 1..=3 {
        println!("Main: {}", i);
        thread::sleep(Duration::from_millis(10));
    }

    handle.join().unwrap();
}

/// 返回线程 ID
pub fn get_thread_id() -> thread::ThreadId {
    thread::current().id()
}

/// 休眠指定时间
pub fn sleep_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

// ========== 消息传递 ==========

/// 发送并接收简单消息
pub fn send_message(message: String) -> String {
    let (tx, rx) = std::sync::mpsc::channel();

    thread::spawn(move || {
        tx.send(message).unwrap();
    });

    rx.recv().unwrap()
}

/// 发送多个消息
pub fn send_multiple() -> Vec<String> {
    let (tx, rx) = std::sync::mpsc::channel();

    thread::spawn(move || {
        for i in 1..=5 {
            tx.send(format!("Message {}", i)).unwrap();
        }
    });

    let mut result = Vec::new();
    for received in rx {
        result.push(received);
    }
    result
}

/// 使用 sync channel
pub fn sync_channel_example() -> String {
    let (tx, rx) = std::sync::mpsc::sync_channel(1);

    thread::spawn(move || {
        tx.send(String::from("Sync message")).unwrap();
    });

    rx.recv().unwrap()
}

// ========== 共享状态 ==========

/// 线程安全的计数器
#[derive(Debug)]
pub struct ThreadSafeCounter {
    counter: Arc<Mutex<i32>>,
}

impl ThreadSafeCounter {
    pub fn new() -> Self {
        Self {
            counter: Arc::new(Mutex::new(0)),
        }
    }

    pub fn increment(&self) {
        let mut count = self.counter.lock().unwrap();
        *count += 1;
    }

    pub fn get(&self) -> i32 {
        *self.counter.lock().unwrap()
    }

    pub fn add(&self, value: i32) {
        let mut count = self.counter.lock().unwrap();
        *count += value;
    }
}

impl Default for ThreadSafeCounter {
    fn default() -> Self {
        Self::new()
    }
}

/// 多线程递增计数器
pub fn increment_from_multiple_threads(count: usize, times: usize) -> i32 {
    let counter = Arc::new(ThreadSafeCounter::new());
    let mut handles = vec![];

    for _ in 0..count {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..times {
                counter_clone.increment();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    counter.get()
}

// ========== 原子类型 ==========

/// 原子计数器
#[derive(Debug)]
pub struct AtomicCounter {
    count: AtomicUsize,
}

impl AtomicCounter {
    pub fn new() -> Self {
        Self {
            count: AtomicUsize::new(0),
        }
    }

    pub fn increment(&self) -> usize {
        self.count.fetch_add(1, Ordering::SeqCst) + 1
    }

    pub fn get(&self) -> usize {
        self.count.load(Ordering::SeqCst)
    }

    pub fn add(&self, value: usize) -> usize {
        self.count.fetch_add(value, Ordering::SeqCst) + value
    }
}

impl Default for AtomicCounter {
    fn default() -> Self {
        Self::new()
    }
}

/// 多线程原子递增
pub fn atomic_increment_from_multiple_threads(count: usize, times: usize) -> usize {
    let counter = Arc::new(AtomicCounter::new());
    let mut handles = vec![];

    for _ in 0..count {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..times {
                counter_clone.increment();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    counter.get()
}

// ========== 简单的并行操作 ==========

/// 并行求和（简化版）
pub fn parallel_sum(numbers: Vec<i32>) -> i32 {
    let mid = numbers.len() / 2;
    let left = numbers[..mid].to_vec();
    let right = numbers[mid..].to_vec();

    let handle = thread::spawn(move || left.into_iter().sum::<i32>());
    let right_sum: i32 = right.into_iter().sum();

    handle.join().unwrap() + right_sum
}

/// 并行查找
pub fn parallel_search(numbers: Vec<i32>, target: i32) -> Option<usize> {
    if numbers.is_empty() {
        return None;
    }

    let mid = numbers.len() / 2;
    let left = numbers[..mid].to_vec();
    let right = numbers[mid..].to_vec();

    let handle = thread::spawn(move || left.iter().position(|&x| x == target));

    let right_result = right.iter().position(|&x| x == target);
    let left_result = handle.join().unwrap();

    match left_result {
        Some(i) => Some(i),
        None => right_result.map(|i| mid + i),
    }
}

// ========== 屏障同步 ==========

/// 使用屏障同步多个线程
pub fn barrier_example(num_threads: usize) {
    use std::sync::Barrier;

    let barrier = Arc::new(Barrier::new(num_threads));
    let mut handles = vec![];

    for i in 0..num_threads {
        let barrier_clone = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            println!("Thread {} - Phase 1", i);
            barrier_clone.wait();
            println!("Thread {} - Phase 2", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

// ========== 读写锁示例 ==========

/// 使用 RwLock 的缓存
#[derive(Debug)]
pub struct Cache {
    data: Arc<std::sync::RwLock<String>>,
}

impl Cache {
    pub fn new(initial: String) -> Self {
        Self {
            data: Arc::new(std::sync::RwLock::new(initial)),
        }
    }

    pub fn read(&self) -> String {
        self.data.read().unwrap().clone()
    }

    pub fn write(&self, new_data: String) {
        let mut data = self.data.write().unwrap();
        *data = new_data;
    }
}

/// 多读者单写者示例
pub fn multiple_readers_one_writer() {
    let cache = Arc::new(Cache::new(String::from("Initial")));
    let mut handles = vec![];

    // 创建多个读线程
    for i in 0..3 {
        let cache_clone = Arc::clone(&cache);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(10));
            println!("Reader {} reads: {}", i, cache_clone.read());
        });
        handles.push(handle);
    }

    // 创建一个写线程
    let cache_clone = Arc::clone(&cache);
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(5));
        cache_clone.write(String::from("Updated"));
        println!("Writer updated the cache");
    });
    handles.push(handle);

    for handle in handles {
        handle.join().unwrap();
    }
}

// ========== 测试 ==========

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_in_thread() {
        assert_eq!(calculate_in_thread(10, 20), 30);
    }

    #[test]
    fn test_send_message() {
        let result = send_message(String::from("Hello"));
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_send_multiple() {
        let result = send_multiple();
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], "Message 1");
    }

    #[test]
    fn test_thread_safe_counter() {
        let counter = ThreadSafeCounter::new();
        counter.increment();
        counter.increment();
        assert_eq!(counter.get(), 2);

        counter.add(5);
        assert_eq!(counter.get(), 7);
    }

    #[test]
    fn test_increment_from_multiple_threads() {
        let result = increment_from_multiple_threads(4, 100);
        assert_eq!(result, 400);
    }

    #[test]
    fn test_atomic_counter() {
        let counter = AtomicCounter::new();
        assert_eq!(counter.increment(), 1);
        assert_eq!(counter.increment(), 2);
        assert_eq!(counter.get(), 2);

        assert_eq!(counter.add(5), 7);
    }

    #[test]
    fn test_atomic_increment_from_multiple_threads() {
        let result = atomic_increment_from_multiple_threads(4, 100);
        assert_eq!(result, 400);
    }

    #[test]
    fn test_parallel_sum() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(parallel_sum(numbers), 55);
    }

    #[test]
    fn test_parallel_search() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(parallel_search(numbers, 3), Some(2));
        assert_eq!(parallel_search(vec![1, 2, 3], 5), None);
    }

    #[test]
    fn test_cache() {
        let cache = Cache::new(String::from("Initial"));
        assert_eq!(cache.read(), "Initial");

        cache.write(String::from("Updated"));
        assert_eq!(cache.read(), "Updated");
    }
}
