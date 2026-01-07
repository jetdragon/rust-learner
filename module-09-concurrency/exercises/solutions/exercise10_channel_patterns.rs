//! # 练习 10: 通道模式 - 解答
//!
//! 难度: 困难
//! 时间: 20 分钟
//! 学习目标: 实现工作队列和扇出-扇入模式

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub struct ThreadPool {
    workers: Vec<thread::JoinHandle<()>>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(std::sync::Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            let receiver = Arc::clone(&receiver);
            let handle = thread::spawn(move || loop {
                let job = receiver.lock().unwrap().recv();

                match job {
                    Ok(job) => {
                        println!("Worker {} executing job", id);
                        job();
                    }
                    Err(_) => {
                        println!("Worker {} disconnected, shutting down.", id);
                        break;
                    }
                }
            });
            workers.push(handle);
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

fn main() {
    let pool = ThreadPool::new(4);

    for i in 0..8 {
        pool.execute(move || {
            println!("Task {} running", i);
            thread::sleep(Duration::from_millis(500));
            println!("Task {} completed", i);
        });
    }

    thread::sleep(Duration::from_secs(3));

    // 扇出-扇入模式
    let (task_tx, task_rx) = mpsc::channel();
    let (result_tx, result_rx) = mpsc::channel();

    for worker_id in 0..3 {
        let task_rx = task_rx.clone();
        let result_tx = result_tx.clone();

        thread::spawn(move || {
            for (i, num) in task_rx.iter().enumerate() {
                let result = num * num;
                println!("Worker {} processed {}: {}^2 = {}", worker_id, i, num, result);
                result_tx.send((worker_id, result)).unwrap();
            }
        });
    }

    for num in 1..=10 {
        task_tx.send(num).unwrap();
    }
    drop(task_tx);

    let mut results = vec![];
    for (worker_id, result) in result_rx.iter().take(10) {
        results.push((worker_id, result));
    }

    println!("All results: {:?}", results);
}
