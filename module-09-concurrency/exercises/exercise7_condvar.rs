//! # 练习 7: 条件变量
//!
//! 难度: 困难
//! 时间: 20 分钟
//! 前置知识: Mutex, Arc
//! 学习目标:
//!   - 使用 Condvar 等待条件
//!   - 实现生产者-消费者模式

use std::sync::{Arc, Condvar, Mutex};
use std::thread;

pub struct Queue<T> {
    data: Mutex<Vec<T>>,
    condvar: Condvar,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            data: Mutex::new(Vec::new()),
            condvar: Condvar::new(),
        }
    }

    pub fn push(&self, item: T) {
        let mut data = self.data.lock().unwrap();
        data.push(item);
        // TODO: 通知等待的线程
        self.condvar.notify_one();
    }

    pub fn pop(&self) -> T {
        let mut data = self.data.lock().unwrap();
        // TODO: 等待数据可用
        while data.is_empty() {
            data = self.condvar.wait(data).unwrap();
        }
        data.pop().unwrap()
    }
}

fn main() {
    let queue = Arc::new(Queue::new());
    let mut handles = vec![];

    // 生产者线程
    for i in 0..3 {
        let queue_clone = Arc::clone(&queue);
        let handle = thread::spawn(move || {
            for j in 0..3 {
                let item = format!("Item {}-{}", i, j);
                println!("Producing: {}", item);
                queue_clone.push(item);
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        });
        handles.push(handle);
    }

    // 消费者线程
    for _ in 0..2 {
        let queue_clone = Arc::clone(&queue);
        let handle = thread::spawn(move || {
            for _ in 0..5 {
                let item = queue_clone.pop();
                println!("Consuming: {}", item);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
