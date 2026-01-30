//! # 练习 9: Send 和 Sync trait
//!
//! 难度: 中等
//! 时间: 15 分钟
//! 前置知识: trait, 并发基础
//! 学习目标:
//!   - 理解 Send 和 Sync marker trait
//!   - 知道哪些类型是 Send/Sync

use std::rc::Rc;
use std::sync::Arc;
use std::thread;

// TODO: 这个函数为什么能编译？
pub fn use_arc() {
    let arc = Arc::new(5);
    let arc_clone = Arc::clone(&arc);

    thread::spawn(move || {
        println!("Arc in thread: {}", arc_clone);
    })
    .join()
    .unwrap();
}

// TODO: 这个函数为什么不能编译？
// pub fn use_rc() {
//     let rc = Rc::new(5);
//     let rc_clone = Rc::clone(&rc);
//
//     thread::spawn(move || {
//         println!("Rc in thread: {}", rc_clone);
//     })
//     .join()
//     .unwrap();
// }

// TODO: 实现一个不是 Send 的类型
use std::rc::Rc as NotSendRc;

pub struct MyStruct {
    rc: NotSendRc<i32>,
}

// TODO: MyStruct 是 Send 吗？为什么？
// unsafe impl Send for MyStruct {}

fn main() {
    use_arc();
    // use_rc(); // 取消注释看看会发生什么

    // 常见类型的 Send/Sync 属性
    // - i32, f64, bool: Send + Sync
    // - String, Vec<T>: Send (T: Send) + Sync (T: Sync)
    // - Arc<T>: Send + Sync (T: Send + Sync)
    // - Rc<T>: !Send, !Sync
    // - Cell<T>: !Sync (但 T: Send 时是 Send)
    // - RefCell<T>: !Send, !Sync
}
