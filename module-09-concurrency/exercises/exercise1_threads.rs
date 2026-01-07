//! # 练习 1: 线程基础
//!
//! 难度: 简单
//! 时间: 15 分钟
//! 前置知识: 所有权基础
//! 学习目标:
//!   - 使用 thread::spawn 创建新线程
//!   - 理解 move 闭包的必要性
//!   - 使用 JoinHandle 等待线程完成

use std::thread;
use std::time::Duration;

pub fn spawn_thread() -> thread::JoinHandle<()> {
    // TODO: 创建一个线程，打印 "Hello from thread!"
    // 提示: 使用 thread::spawn 和 move 闭包
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

    // TODO: 等待线程结束
    handle.join().unwrap();
}

fn main() {
    run_with_delay();
}
