// examples/thread_pool.rs
// 线程池实现演示

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

fn main() {
    println!("=== 线程池演示 ===\n");

    // 1. 线程池基础使用
    println!("1. 线程池基础使用:");
    let pool = ThreadPool::new(4);

    for i in 0..8 {
        pool.execute(move || {
            println!("  任务 {} 在线程 {:?} 上执行", i, thread::current().id());
            thread::sleep(Duration::from_millis(100));
        });
    }

    thread::sleep(Duration::from_secs(1));
    println!("  所有任务完成");

    // 2. 线程池 vs 手动创建线程
    println!("\n2. 性能对比: 线程池 vs 手动创建");
    use std::time::Instant;

    const TASK_COUNT: usize = 1000;

    // 线程池
    let start = Instant::now();
    let pool = ThreadPool::new(4);
    for _ in 0..TASK_COUNT {
        pool.execute(|| {
            thread::sleep(Duration::from_micros(100));
        });
    }
    thread::sleep(Duration::from_millis(200));
    let pool_time = start.elapsed();
    println!("  线程池完成 {} 个任务: {:?}", TASK_COUNT, pool_time);

    // 手动创建线程
    let start = Instant::now();
    let mut handles = vec![];
    for _ in 0..TASK_COUNT {
        let handle = thread::spawn(|| {
            thread::sleep(Duration::from_micros(100));
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let manual_time = start.elapsed();
    println!("  手动线程完成 {} 个任务: {:?}", TASK_COUNT, manual_time);

    println!(
        "  线程池比手动创建快 {:.2} 倍",
        manual_time.as_millis() as f64 / pool_time.as_millis() as f64
    );

    // 3. 限流线程池
    println!("\n3. 限流线程池 (防止任务堆积）:");
    struct BoundedThreadPool {
        sender: mpsc::SyncSender<Job>,
    }

    impl BoundedThreadPool {
        fn new(size: usize, bound: usize) -> BoundedThreadPool {
            let (sender, receiver) = mpsc::sync_channel::<Job>(bound);
            let receiver = Arc::new(Mutex::new(receiver));

            for id in 0..size {
                thread::spawn({
                    let receiver = Arc::clone(&receiver);
                    move || loop {
                        let job = receiver.lock().unwrap().recv();
                        match job {
                            Ok(job) => {
                                println!("  线程 {} 执行任务", id);
                                job();
                            }
                            Err(_) => break,
                        }
                    }
                });
            }

            BoundedThreadPool { sender }
        }

        fn execute<F>(&self, f: F) -> Result<(), String>
        where
            F: FnOnce() + Send + 'static,
        {
            self.sender.send(Box::new(f)).map_err(|e| e.to_string())
        }
    }

    let pool = BoundedThreadPool::new(2, 5); // 2 线程，5 任务上限

    for i in 0..10 {
        match pool.execute(move || {
            println!("  限流任务 {}", i);
            thread::sleep(Duration::from_millis(200));
        }) {
            Ok(_) => println!("  任务 {} 提交成功", i),
            Err(_) => println!("  任务 {} 提交失败（队列已满）", i),
        }
        thread::sleep(Duration::from_millis(50));
    }

    thread::sleep(Duration::from_millis(1000));
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv();
            match job {
                Ok(job) => {
                    println!("    Worker {} 执行任务", id);
                    job();
                }
                Err(_) => {
                    println!("    Worker {} 退出", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

use std::time::Duration;
