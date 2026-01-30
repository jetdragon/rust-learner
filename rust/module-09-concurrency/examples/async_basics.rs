// examples/async_basics.rs
// async/await 异步编程基础演示

// 注意：此示例需要 tokio 运行时
// 运行方式：cargo run --example async_basics

use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("=== async/await 基础演示 ===\n");

    // 1. async 函数基础
    println!("1. async 函数基础:");
    let result = hello_world().await;
    println!("  结果: {}", result);

    // 2. 多个 async 函数顺序执行
    println!("\n2. 顺序执行 async 任务:");
    let start = std::time::Instant::now();
    task1().await;
    task2().await;
    task3().await;
    println!("  总耗时: {:?}", start.elapsed());

    // 3. 并发执行 async 任务
    println!("\n3. 并发执行 async 任务:");
    let start = std::time::Instant::now();
    let (r1, r2, r3) = tokio::join!(task1(), task2(), task3());
    println!("  结果: {}, {}, {}", r1, r2, r3);
    println!("  总耗时: {:?}", start.elapsed());

    // 4. 使用 select! 等待第一个完成的任务
    println!("\n4. 使用 select! 等待第一个完成:");
    let result = race_tasks().await;
    println!("  获胜者: {}", result);

    // 5. 超时处理
    println!("\n5. 超时处理:");
    let result = with_timeout().await;
    match result {
        Ok(msg) => println!("  成功: {}", msg),
        Err(e) => println!("  超时: {}", e),
    }

    // 6. 错误处理
    println!("\n6. async 错误处理:");
    match async_error().await {
        Ok(data) => println!("  数据: {}", data),
        Err(e) => println!("  错误: {}", e),
    }

    // 7. 限制并发数
    println!("\n7. 限制并发数:");
    limit_concurrency(10, 3).await;
}

// async 函数返回一个 Future
async fn hello_world() -> String {
    "Hello, world!".to_string()
}

// 模拟异步任务 1
async fn task1() -> &'static str {
    sleep(Duration::from_millis(100)).await;
    "task1"
}

// 模拟异步任务 2
async fn task2() -> &'static str {
    sleep(Duration::from_millis(100)).await;
    "task2"
}

// 模拟异步任务 3
async fn task3() -> &'static str {
    sleep(Duration::from_millis(100)).await;
    "task3"
}

// 使用 select! 等待第一个完成的任务
async fn race_tasks() -> String {
    use tokio::select;

    let task_a = async {
        sleep(Duration::from_millis(300)).await;
        "Task A".to_string()
    };

    let task_b = async {
        sleep(Duration::from_millis(100)).await;
        "Task B".to_string()
    };

    select! {
        result = task_a => result,
        result = task_b => result,
    }
}

// 带超时的任务
async fn with_timeout() -> Result<String, String> {
    use tokio::select;

    let task = async {
        sleep(Duration::from_millis(500)).await;
        "任务完成".to_string()
    };

    let timeout = async {
        sleep(Duration::from_millis(300)).await;
        Err("超时了".to_string())
    };

    select! {
        result = task => Ok(result),
        err = timeout => err,
    }
}

// 返回错误的 async 函数
async fn async_error() -> Result<String, String> {
    Err("发生错误".to_string())
}

// 限制并发数的任务执行
async fn limit_concurrency(total: usize, limit: usize) {
    use std::sync::Arc;
    use tokio::sync::Semaphore;

    let semaphore = Arc::new(Semaphore::new(limit));

    println!("  执行 {} 个任务，最大并发数: {}", total, limit);

    let mut handles = vec![];

    for i in 0..total {
        let sem = semaphore.clone();
        let handle = tokio::spawn(async move {
            // 获取许可证
            let _permit = sem.acquire().await.unwrap();
            println!("    任务 {} 开始执行", i);
            sleep(Duration::from_millis(100)).await;
            println!("    任务 {} 完成", i);
        });
        handles.push(handle);
    }

    // 等待所有任务完成
    for handle in handles {
        handle.await.unwrap();
    }
}
