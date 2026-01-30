//! # 练习 6: RwLock 读写锁 - 解答
//!
//! 难度: 中等
//! 时间: 15 分钟
//! 学习目标: 理解 RwLock 与 Mutex 的区别

use std::sync::{Arc, RwLock};
use std::thread;

pub struct Database {
    data: Arc<RwLock<Vec<String>>>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            data: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn add_record(&self, record: String) {
        let mut data = self.data.write().unwrap();
        data.push(record);
        println!("Added record: {}", record);
    }

    pub fn read_records(&self) -> Vec<String> {
        let data = self.data.read().unwrap();
        data.clone()
    }

    pub fn count_records(&self) -> usize {
        let data = self.data.read().unwrap();
        data.len()
    }
}

fn main() {
    let db = Database::new();
    let mut handles = vec![];

    for i in 0..3 {
        let db_clone = Database {
            data: Arc::clone(&db.data),
        };

        let handle = thread::spawn(move || {
            db_clone.add_record(format!("Record {}", i));
        });
        handles.push(handle);
    }

    for i in 0..5 {
        let db_clone = Database {
            data: Arc::clone(&db.data),
        };

        let handle = thread::spawn(move || {
            let count = db_clone.count_records();
            println!("Reader {}: count = {}", i, count);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final records: {:?}", db.read_records());
}
