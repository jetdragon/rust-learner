// examples/channels.rs
// Channel 消息传递演示

use module_09_concurrency::{send_message, send_multiple, sync_channel_example};

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Channel 消息传递演示 ===\n");

    // 1. 基础 channel
    println!("1. 基础 channel:");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(42).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("  Received: {}", received);

    // 2. 发送多个值
    println!("\n2. 发送多个值:");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for i in 1..=5 {
            tx.send(i).unwrap();
        }
    });

    for received in rx {
        println!("  Received: {}", received);
    }

    // 3. 多个生产者
    println!("\n3. 多个生产者:");
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    let tx2 = tx.clone();

    thread::spawn(move || {
        tx1.send("Producer 1").unwrap();
    });

    thread::spawn(move || {
        tx2.send("Producer 2").unwrap();
    });

    drop(tx); // 主线程不再持有发送者

    for received in rx {
        println!("  Received: {}", received);
    }

    // 4. 发送不同类型
    println!("\n4. 发送字符串:");
    let result = send_message(String::from("Hello from channel!"));
    println!("  Result: {}", result);

    // 5. try_recv 非阻塞
    println!("\n5. try_recv 非阻塞:");
    let (tx, rx) = mpsc::channel();

    match rx.try_recv() {
        Ok(msg) => println!("  Got: {}", msg),
        Err(_) => println!("  No message available"),
    }

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        tx.send("Delayed message").unwrap();
    });

    // 等待消息
    loop {
        match rx.try_recv() {
            Ok(msg) => {
                println!("  Got: {}", msg);
                break;
            }
            Err(_) => {
                println!("  Waiting...");
                thread::sleep(Duration::from_millis(50));
            }
        }
    }

    // 6. send_multiple 演示
    println!("\n6. send_multiple 演示:");
    let results = send_multiple();
    println!("  Received {} messages", results.len());
    for msg in results {
        println!("  - {}", msg);
    }

    // 7. recv_timeout
    println!("\n7. recv_timeout:");
    let (tx, rx) = mpsc::channel();

    match rx.recv_timeout(Duration::from_millis(100)) {
        Ok(msg) => println!("  Got: {}", msg),
        Err(_) => println!("  Timeout - no message"),
    }

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        tx.send("Quick message").unwrap();
    });

    match rx.recv_timeout(Duration::from_millis(100)) {
        Ok(msg) => println!("  Got: {}", msg),
        Err(_) => println!("  Timeout"),
    }

    // 8. sync_channel
    println!("\n8. sync_channel:");
    let result = sync_channel_example();
    println!("  Result: {}", result);

    // 9. 带所有权的数据
    println!("\n9. 带所有权的数据:");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let data = vec![1, 2, 3, 4, 5];
        tx.send(data).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("  Received vector: {:?}", received);

    // 10. 发送者关闭检测
    println!("\n10. 发送者关闭检测:");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(1).unwrap();
        tx.send(2).unwrap();
        // tx 在这里被 drop
    });

    // 接收所有消息直到 channel 关闭
    for received in rx {
        println!("  Got: {}", received);
    }
    println!("  Channel closed");
}
