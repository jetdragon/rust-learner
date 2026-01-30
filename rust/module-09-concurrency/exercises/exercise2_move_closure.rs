//! # 练习 2: Move 闭包
//!
//! 难度: 简单
//! 时间: 10 分钟
//! 前置知识: 闭包, 所有权
//! 学习目标:
//!   - 理解为什么线程需要 move 闭包
//!   - 区分 &T 和 move 的所有权转移

use std::thread;

pub fn capture_by_reference() {
    let v = vec![1, 2, 3];

    // TODO: 这段代码为什么不能编译？
    // 提示: 线程可能在 v 被释放后还在运行
    let handle = thread::spawn(|| {
        println!("Vector: {:?}", v);
    });

    handle.join().unwrap();
}

pub fn capture_by_move() {
    let v = vec![1, 2, 3];

    // TODO: 使用 move 修复上面的代码
    let handle = thread::spawn(move || {
        println!("Vector: {:?}", v);
    });

    handle.join().unwrap();

    // TODO: 下面这行代码能编译吗？为什么？
    // println!("Vector in main: {:?}", v);
}

fn main() {
    capture_by_move();
}
