//! # 练习 5: Arc 与 Mutex 结合 - 解答
//!
//! 难度: 中等
//! 时间: 15 分钟
//! 学习目标: 理解为什么需要 Arc<Mutex<T>>

use std::sync::{Arc, Mutex};
use std::thread;

pub struct SharedBankAccount {
    balance: Arc<Mutex<f64>>,
}

impl SharedBankAccount {
    pub fn new(initial_balance: f64) -> Self {
        SharedBankAccount {
            balance: Arc::new(Mutex::new(initial_balance)),
        }
    }

    pub fn deposit(&self, amount: f64) {
        let mut balance = self.balance.lock().unwrap();
        *balance += amount;
        println!("Deposited ${:.2}, new balance: ${:.2}", amount, *balance);
    }

    pub fn withdraw(&self, amount: f64) -> bool {
        let mut balance = self.balance.lock().unwrap();
        if *balance >= amount {
            *balance -= amount;
            println!("Withdrew ${:.2}, new balance: ${:.2}", amount, *balance);
            true
        } else {
            println!("Insufficient funds for ${:.2}", amount);
            false
        }
    }

    pub fn get_balance(&self) -> f64 {
        *self.balance.lock().unwrap()
    }
}

fn main() {
    let account = SharedBankAccount::new(1000.0);
    let mut handles = vec![];

    for _ in 0..5 {
        let account_clone = SharedBankAccount {
            balance: Arc::clone(&account.balance),
        };

        let handle = thread::spawn(move || {
            account_clone.deposit(100.0);
            account_clone.withdraw(50.0);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final balance: ${:.2}", account.get_balance());
}
