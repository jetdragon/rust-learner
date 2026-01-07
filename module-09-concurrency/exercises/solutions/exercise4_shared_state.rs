//! # 练习 4: 共享状态 - 解答
//!
//! 难度: 中等
//! 时间: 15 分钟
//! 学习目标: 使用 Mutex 和 Arc 在线程间共享状态

use std::sync::{Arc, Mutex};
use std::thread;

pub fn mutex_basic() {
    let m = Mutex::new(5);

    {
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
