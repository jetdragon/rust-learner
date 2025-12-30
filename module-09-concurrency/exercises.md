# 09-并发编程 - 练习题

## 基础练习

### 1. 创建线程

创建一个线程并计算 1 到 100 的和。

```rust
use std::thread;

pub fn calculate_in_thread() -> i32 {
    // TODO: 创建线程计算 1 到 100 的和
    // 提示: 使用 join() 等待线程完成并获取结果
}
```

### 2. 消息传递

使用 channel 发送消息。

```rust
use std::sync::mpsc;

pub fn send_message(message: String) -> String {
    let (tx, rx) = mpsc::channel();

    // TODO: 在新线程中发送消息
    // TODO: 在主线程中接收消息

    received
}
```

### 3. Mutex 计数器

使用 Mutex 实现线程安全的计数器。

```rust
use std::sync::{Arc, Mutex};

pub struct ThreadSafeCounter {
    counter: Arc<Mutex<i32>>,
}

impl ThreadSafeCounter {
    pub fn new() -> Self {
        // TODO: 创建新的计数器
    }

    pub fn increment(&self) {
        // TODO: 增加计数
    }

    pub fn get(&self) -> i32 {
        // TODO: 获取当前值
    }
}
```

### 4. 多线程求和

使用多个线程计算向量元素的和。

```rust
pub fn parallel_sum(numbers: Vec<i32>) -> i32 {
    // TODO: 使用 4 个线程并行求和
    // 提示: 使用 Arc<Mutex<i32>> 或 channel 收集结果
}
```

## 进阶练习

### 5. 生产者-消费者模型

实现一个简单的生产者-消费者系统。

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn producer_consumer() {
    let (tx, rx) = mpsc::channel();

    // TODO: 创建生产者线程，每 100ms 发送一个数字
    // TODO: 创建消费者线程，接收并打印数字

    // 让主线程等待 1 秒
    thread::sleep(Duration::from_secs(1));
}
```

### 6. 工作队列

实现一个工作队列，多个线程处理任务。

```rust
use std::sync::{Arc, Mutex, mpsc};

pub struct ThreadPool {
    workers: Vec<thread::JoinHandle<()>>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        // TODO: 创建指定数量的工作线程
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // TODO: 发送任务到工作队列
    }
}
```

### 7. 读写锁

使用 RwLock 实现读写分离。

```rust
use std::sync::RwLock;

pub struct Cache {
    data: RwLock<String>,
}

impl Cache {
    pub fn new(initial: String) -> Self {
        // TODO: 创建缓存
    }

    pub fn read(&self) -> String {
        // TODO: 读取数据（多个读线程可以同时进行）
    }

    pub fn write(&self, new_data: String) {
        // TODO: 写入数据（写操作独占访问）
    }
}
```

### 8. 原子操作

使用原子类型实现无锁计数器。

```rust
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct LockFreeCounter {
    counter: AtomicUsize,
}

impl LockFreeCounter {
    pub fn new() -> Self {
        // TODO: 创建新的无锁计数器
    }

    pub fn increment(&self) -> usize {
        // TODO: 原子地增加计数
    }

    pub fn get(&self) -> usize {
        // TODO: 获取当前值
    }
}
```

## 挑战练习

### 9. 并行 Map

实现并行 map 操作。

```rust
pub fn parallel_map<F>(items: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32 + Send + 'static,
{
    // TODO: 使用多个线程并行处理每个元素
    // 提示: 使用 channel 收集结果
}
```

### 10. 屏障同步

使用 Barrier 实现多线程同步。

```rust
use std::sync::{Arc, Barrier};

pub fn synchronize_with_barrier(num_threads: usize) {
    let barrier = Arc::new(Barrier::new(num_threads));
    let mut handles = vec![];

    for i in 0..num_threads {
        let barrier_clone = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            // TODO: 第一阶段工作
            println!("Thread {} phase 1", i);

            // TODO: 等待所有线程到达
            barrier_clone.wait();

            // TODO: 第二阶段工作
            println!("Thread {} phase 2", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
```

### 11. 超时控制

实现带超时的线程操作。

```rust
use std::time::Duration;

pub fn with_timeout<F, T>(f: F, timeout: Duration) -> Option<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    // TODO: 在线程中执行函数 f
    // TODO: 如果超时返回 None，否则返回 Some(result)
}
```

### 12. 取消令牌

实现可取消的长时间运行任务。

```rust
use std::sync::atomic::{AtomicBool, Ordering};

pub struct CancellableTask {
    cancelled: AtomicBool,
}

impl CancellableTask {
    pub fn new() -> Self {
        // TODO: 创建新任务
    }

    pub fn cancel(&self) {
        // TODO: 设置取消标志
    }

    pub fn is_cancelled(&self) -> bool {
        // TODO: 检查是否取消
    }

    pub fn run(&self, f: impl Fn(usize) -> bool) {
        // TODO: 运行任务 f，定期检查取消状态
    }
}
```

## 实战练习

### 13. 并行下载器

```rust
pub struct DownloadTask {
    url: String,
    content: Option<String>,
}

pub fn parallel_downloads(urls: Vec<String>) -> Vec<DownloadTask> {
    // TODO: 并行下载多个 URL
    // 提示: 使用 thread pool 或 channel
}
```

### 14. 进度条

```rust
use std::sync::{Arc, Mutex};

pub struct ProgressBar {
    total: usize,
    current: Arc<Mutex<usize>>,
}

impl ProgressBar {
    pub fn new(total: usize) -> Self {
        // TODO: 创建进度条
    }

    pub fn update(&self, delta: usize) {
        // TODO: 更新进度
    }

    pub fn display(&self) {
        // TODO: 显示进度条
    }
}
```

### 15. 并发缓存

```rust
use std::collections::HashMap;
use std::sync::RwLock;

pub struct ConcurrentCache<K, V>
where
    K: Eq + std::hash::Hash,
{
    data: RwLock<HashMap<K, V>>,
}

impl<K, V> ConcurrentCache<K, V>
where
    K: Eq + std::hash::Hash,
{
    pub fn new() -> Self {
        // TODO: 创建新缓存
    }

    pub fn get(&self, key: &K) -> Option<V>
    where
        V: Clone,
    {
        // TODO: 获取值
    }

    pub fn set(&self, key: K, value: V) {
        // TODO: 设置值
    }

    pub fn remove(&self, key: &K) -> Option<V> {
        // TODO: 移除值
    }
}
```

## 测试验证

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_in_thread() {
        assert_eq!(calculate_in_thread(), 5050);
    }

    #[test]
    fn test_send_message() {
        let result = send_message(String::from("Hello"));
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_counter() {
        let counter = ThreadSafeCounter::new();
        counter.increment();
        counter.increment();
        assert_eq!(counter.get(), 2);
    }

    // TODO: 为每个练习添加测试
}
```

## 学习检查

完成练习后，检查你是否理解：

- [ ] 线程的创建和等待
- [ ] Channel 的使用（发送和接收）
- [ ] Mutex 的使用（加锁和解锁）
- [ ] Arc 的使用（多线程共享）
- [ ] RwLock 的读写分离
- [ ] 原子类型的使用
- [ ] 死锁的避免
- [ ] 常见的并发模式
