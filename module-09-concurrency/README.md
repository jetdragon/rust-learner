# 09-并发编程

## 学习目标

- 理解线程的创建和使用
- 掌握消息传递机制
- 学习共享状态并发
- 了解 Sync 和 Send trait

## 核心概念

### 1. 线程 (Threads)

Rust 标准库提供了线程原生支持。

```rust
use std::thread;
use std::time::Duration;

thread::spawn(|| {
    for i in 1..10 {
        println!("hi number {} from the spawned thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
});

for i in 1..5 {
    println!("hi number {} from the main thread!", i);
    thread::sleep(Duration::from_millis(1));
}
```

### 2. 等待线程结束 (Join)

```rust
let handle = thread::spawn(|| {
    42
});

let result = handle.join().unwrap();
```

### 3. 消息传递 (Message Passing)

使用 channel 进行线程间通信。

```rust
use std::sync::mpsc;

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    tx.send(42).unwrap();
});

let received = rx.recv().unwrap();
```

### 4. 共享状态 (Shared State)

使用 Mutex 和 Arc 实现共享状态。

```rust
use std::sync::{Arc, Mutex};

let counter = Arc::new(Mutex::new(0));
let counter_clone = Arc::clone(&counter);

thread::spawn(move || {
    *counter_clone.lock().unwrap() += 1;
});
```

### 5. Send 和 Sync

- `Send`: 类型可以在线程间转移所有权
- `Sync`: 类型可以在线程间共享引用

大多数类型自动实现这些 trait。

## 并发模式

### 1. 生产者-消费者

```rust
let (tx, rx) = mpsc::channel();

// 生产者
thread::spawn(move || {
    for i in 0..10 {
        tx.send(i).unwrap();
    }
});

// 消费者
for received in rx {
    println!("{}", received);
}
```

### 2. 工作队列

```rust
let (tx, rx) = mpsc::channel();
let rx = Arc::new(Mutex::new(rx));

for i in 0..4 {
    let rx_clone = Arc::clone(&rx);
    thread::spawn(move || {
        while let Ok(job) = rx_clone.lock().unwrap().recv() {
            job();
        }
    });
}
```

### 3. 屏障 (Barrier)

```rust
use std::sync::Barrier;

let barrier = Arc::new(Barrier::new(4));

for i in 0..4 {
    let barrier_clone = Arc::clone(&barrier);
    thread::spawn(move || {
        // ... 工作完成
        barrier_clone.wait();
    });
}
```

## 原子类型

```rust
use std::sync::atomic::{AtomicUsize, Ordering};

let counter = AtomicUsize::new(0);
counter.fetch_add(1, Ordering::SeqCst);
```

## 实践建议

1. **优先使用消息传递**：更安全，避免锁
2. **避免锁竞争**：最小化临界区
3. **使用 Arc 和 Mutex**：共享可变状态
4. **小心死锁**：按一致顺序获取锁
5. **考虑原子类型**：简单计数器场景

## 相关资源

- [Rust Book - 并发](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [Rust Book - 消息传递](https://doc.rust-lang.org/book/ch16-02-message-passing.html)
- [Rust Book - 共享状态](https://doc.rust-lang.org/book/ch16-03-shared-state.html)
