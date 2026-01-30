//! # 练习 1: 线程基础 - 解答
//!
//! 难度: 简单
//! 时间: 15 分钟
//! 学习目标: 使用 thread::spawn 创建新线程

use std::thread;
use std::time::Duration;

pub fn spawn_thread() -> thread::JoinHandle<()> {
    // 创建一个线程，打印消息
    thread::spawn(move || {
        for i in 1..=5 {
            println!("Thread count: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    })
}

pub fn run_with_delay() {
    let handle = spawn_thread();

    // 主线程也做一些工作
    for i in 1..=3 {
        println!("Main count: {}", i);
        thread::sleep(Duration::from_millis(100));
    }

    // 等待线程结束
    handle.join().unwrap();
}

fn main() {
    run_with_delay();
}
