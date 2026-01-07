//! # 练习 4: 共享状态
//!
//! 难度: 中等
//! 时间: 15 分钟
//! 前置知识: Mutex, Arc
//! 学习目标:
//!   - 使用 Mutex 实现内部可变性
//!   - 使用 Arc 在线程间共享所有权
//!   - 理解锁的作用

use std::sync::{Arc, Mutex};
use std::thread;

pub fn mutex_basic() {
    let m = Mutex::new(5);

    {
        // TODO: 获取锁并修改值
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

pub fn shared_counter() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // TODO: 获取锁并增加计数器
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn main() {
    mutex_basic();
    shared_counter();
}
