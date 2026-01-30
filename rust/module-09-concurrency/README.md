# 09-并发编程

## 学习目标

- 理解线程的创建和使用
- 掌握消息传递机制
- 学习共享状态并发（Mutex、RwLock、Barrier）
- 了解 async/await 异步编程
- 理解线程池的实现
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

### 6. RwLock (读写锁)

RwLock 允许多个读者或单个写者同时访问数据：

```rust
use std::sync::{Arc, RwLock};

let data = Arc::new(RwLock::new(5));

// 多个读锁可以共存
{
    let r1 = data.read().unwrap();
    let r2 = data.read().unwrap();
    println!("r1: {}, r2: {}", r1, r2);
}

// 写锁是排他的
{
    let mut w = data.write().unwrap();
    *w += 1;
}
```

**使用场景**：
- 读多写少的情况
- 需要多个读者并发访问
- 读操作不需要互斥

### 7. Condvar (条件变量)

Condvar 用于在线程间等待条件：

```rust
use std::sync::{Arc, Mutex, Condvar};

let pair = Arc::new((Mutex::new(false), Condvar::new()));
let pair2 = Arc::clone(&pair);

// 等待线程
thread::spawn(move || {
    let (lock, cvar) = &*pair2;
    let mut started = lock.lock().unwrap();
    *started = true;
    cvar.notify_one();
});

// 主线程等待
let (lock, cvar) = &*pair;
let mut started = lock.lock().unwrap();
while !*started {
    started = cvar.wait(started).unwrap();
}
```

**使用场景**：
- 生产者-消费者模式
- 线程间协调
- 等待特定条件满足

### 8. async/await 异步编程

Rust 支持异步编程，使用 async/await 语法：

```rust
// async 函数返回一个 Future
async fn hello() -> String {
    "Hello, world!".to_string()
}

// 使用 await 等待 Future 完成
#[tokio::main]
async fn main() {
    let result = hello().await;
    println!("{}", result);
}

// 并发执行多个 async 任务
async fn task1() -> &'static str { "task1" }
async fn task2() -> &'static str { "task2" }

let (r1, r2) = tokio::join!(task1(), task2());
```

**关键概念**：
- `async fn` 返回 `impl Future<Output = T>`
- `.await` 暂停当前 async 函数，等待 Future 完成
- 使用 runtime（如 tokio）执行 async 代码

### 9. Future trait

Future trait 表示一个可能尚未完成的异步计算：

```rust
use std::future::Future;

// 自定义 Future
struct MyFuture {
    completed: bool,
}

impl Future for MyFuture {
    type Output = i32;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if self.completed {
            std::task::Poll::Ready(42)
        } else {
            self.completed = true;
            cx.waker().wake_by_ref();
            std::task::Poll::Pending
        }
    }
}
```

### 10. 线程池

线程池管理一组可重用的工作线程：

```rust
struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            job();
        });

        Worker { id, thread: Some(thread) }
    }
}
```


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

### 4. 读多写少模式

```rust
use std::sync::{Arc, RwLock};

struct Cache {
    data: Arc<RwLock<HashMap<String, String>>>,
}

impl Cache {
    fn new() -> Self {
        Cache {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn get(&self, key: &str) -> Option<String> {
        let data = self.data.read().unwrap();
        data.get(key).cloned()
    }

    fn set(&self, key: String, value: String) {
        let mut data = self.data.write().unwrap();
        data.insert(key, value);
    }
}
```

### 5. 限流模式

```rust
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

struct RateLimiter {
    max: usize,
    current: Arc<AtomicUsize>,
}

impl RateLimiter {
    fn new(max: usize) -> Self {
        RateLimiter {
            max,
            current: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn acquire(&self) -> bool {
        let mut count = self.current.load(Ordering::Relaxed);

        while count < self.max {
            match self.current.compare_exchange_weak(
                count,
                count + 1,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => return true,
                Err(new_count) => count = new_count,
            }
        }

        false
    }
}
```

### 6. 异步并发模式

```rust
use tokio::time::{sleep, Duration};

// 并发执行多个任务
async fn run_concurrently() {
    let task1 = async {
        sleep(Duration::from_secs(1)).await;
        "task1"
    };

    let task2 = async {
        sleep(Duration::from_secs(1)).await;
        "task2"
    };

    // 并发执行
    let (r1, r2) = tokio::join!(task1, task2);
    println!("{}, {}", r1, r2);
}

// 选择第一个完成的任务
async fn race() {
    let task1 = sleep(Duration::from_secs(3));
    let task2 = sleep(Duration::from_secs(1));

    // task2 先完成
    tokio::select! {
        _ = task1 => println!("task1 finished first"),
        _ = task2 => println!("task2 finished first"),
    }
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
6. **读多写少用 RwLock**：而不是 Mutex
7. **使用线程池**：避免频繁创建销毁线程
8. **异步编程**：IO 密集型任务使用 async/await
9. **避免忙等待**：使用 sleep 或条件变量而不是循环
10. **注意闭包所有权**：使用 `move` 转移所有权到线程

## 代码示例

本模块的代码示例位于 `examples/` 目录：

- `threads.rs` - 线程基础演示
- `channels.rs` - Channel 消息传递演示
- `shared_state.rs` - Mutex 共享状态演示
- `rwlock.rs` - RwLock 读写锁演示
- `condvar.rs` - 条件变量演示
- `atomic.rs` - 原子类型演示
- `async_basics.rs` - async/await 基础演示
- `thread_pool.rs` - 线程池实现演示

## 相关资源

- [Rust Book - 并发](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [Rust Book - 消息传递](https://doc.rust-lang.org/book/ch16-02-message-passing.html)
- [Rust Book - 共享状态](https://doc.rust-lang.org/book/ch16-03-shared-state.html)
