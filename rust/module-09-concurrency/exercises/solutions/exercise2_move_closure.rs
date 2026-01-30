//! # 练习 2: Move 闭包 - 解答
//!
//! 难度: 简单
//! 时间: 10 分钟
//! 学习目标: 理解为什么线程需要 move 闭包

use std::thread;

pub fn capture_by_move() {
    let v = vec![1, 2, 3];

    // 使用 move 修复代码
    // move 关键字将 v 的所有权转移到闭包中
    let handle = thread::spawn(move || {
        println!("Vector: {:?}", v);
    });

    handle.join().unwrap();

    // 下面这行不能编译，因为 v 的所有权已转移
    // println!("Vector in main: {:?}", v);
}

fn main() {
    capture_by_move();
}
