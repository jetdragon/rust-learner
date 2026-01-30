//! # 练习 8: 原子类型
//!
//! 难度: 中等
//! 时间: 15 分钟
//! 前置知识: 基础并发
//! 学习目标:
//!   - 使用原子类型实现无锁并发
//!   - 理解 Ordering 类型

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

pub struct Counter {
    count: AtomicUsize,
}

impl Counter {
    pub fn new() -> Self {
        Counter {
            count: AtomicUsize::new(0),
        }
    }

    pub fn increment(&self) {
        // TODO: 使用合适的 Ordering 增加计数
        self.count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn get(&self) -> usize {
        self.count.load(Ordering::SeqCst)
    }
}

fn main() {
    let counter = Arc::new(Counter::new());
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter_clone.increment();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final count: {}", counter.get());

    // TODO: 对比 Mutex vs Atomic 的性能
    // 使用 AtomicUsize 比 Mutex<usize> 更高效
}
