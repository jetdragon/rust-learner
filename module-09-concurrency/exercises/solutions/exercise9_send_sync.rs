//! # 练习 9: Send 和 Sync trait - 解答
//!
//! 难度: 中等
//! 时间: 15 分钟
//! 学习目标: 理解 Send 和 Sync marker trait

use std::sync::Arc;
use std::thread;

pub fn use_arc() {
    let arc = Arc::new(5);
    let arc_clone = Arc::clone(&arc);

    thread::spawn(move || {
        println!("Arc in thread: {}", arc_clone);
    })
    .join()
    .unwrap();
}

fn main() {
    use_arc();

    // Send/Sync 属性说明:
    // - i32: Send + Sync
    // - Arc<T>: Send + Sync (T: Send + Sync)
    // - Rc<T>: !Send, !Sync
}
