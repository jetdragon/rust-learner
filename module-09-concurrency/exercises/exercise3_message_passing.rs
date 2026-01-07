//! # 练习 3: 消息传递
//!
//! 难度: 中等
//! 时间: 15 分钟
//! 前置知识: 通道, 线程
//! 学习目标:
//!   - 使用 mpsc::channel 创建通道
//!   - 在线程间发送消息
//!   - 理解 tx 和 rx 的所有权

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn send_messages() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            // TODO: 发送消息到通道
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    // TODO: 接收消息
    for received in rx {
        println!("Got: {}", received);
    }
}

pub fn multiple_producers() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn main() {
    send_messages();
    multiple_producers();
}
