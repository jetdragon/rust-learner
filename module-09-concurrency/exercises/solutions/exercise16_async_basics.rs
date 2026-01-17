//! # Async/Await 练习解答
//!
//! **解答要点:**
//! 1. async fn 返回 impl Future<Output = T>
//! 2. .await 挂起当前 async 函数，等待 Future 完成
//! 3. 使用 tokio::time::sleep 进行异步延迟
//! 4. 使用 tokio::join! 并发执行多个 async 任务

use tokio::time::{sleep, Duration};

pub async fn async_hello() -> String {
    "Hello, async!".to_string()
}

pub async fn async_sleep(ms: u64) -> String {
    sleep(Duration::from_millis(ms)).await;
    format!("Slept {} ms", ms)
}

pub async fn async_add(a: u32, b: u32) -> u32 {
    sleep(Duration::from_millis(100)).await;
    a + b
}

pub async fn concurrent_tasks(tasks: Vec<u32>) -> Vec<String> {
    let mut results = Vec::new();

    for (i, &task) in tasks.iter().enumerate() {
        let result = async_add(task, i as u32 * 10).await;
        results.push(format!("Task {}: {}", i, result));
    }

    results
}

pub async fn race_tasks(tasks: Vec<u32>) -> u32 {
    let mut handles = Vec::new();

    for &task in &tasks {
        let handle = tokio::spawn(async move { async_add(task, 0).await });
        handles.push(handle);
    }

    // 使用 tokio::select! 等待第一个完成的任务
    let result = tokio::select! {
        result = handles[0], if result.is_ok() => {
            result.unwrap()
        } => {
            handles[1].await.unwrap()
        }
    };

    // 取消其他任务
    for handle in handles {
        handle.abort();
    }

    result
}

pub async fn timeout_example(duration_ms: u64) -> String {
    let result = tokio::time::timeout(Duration::from_millis(duration_ms), async_add(10, 20)).await;

    match result {
        Ok(value) => format!("Result: {}", value),
        Err(_) => "Timeout!".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_async_hello() {
        let result = async_hello().await;
        assert_eq!(result, "Hello, async!");
    }

    #[tokio::test]
    async fn test_async_sleep() {
        let result = async_sleep(100).await;
        assert!(result.contains("100"));
    }

    #[tokio::test]
    async fn test_async_add() {
        let result = async_add(10, 20).await;
        assert_eq!(result, 30);
    }

    #[tokio::test]
    async fn test_concurrent_tasks() {
        let tasks = vec![1, 2, 3, 4, 5];
        let results = concurrent_tasks(tasks).await;
        assert_eq!(results.len(), 5);
    }

    #[tokio::test]
    async fn test_timeout_success() {
        let result = timeout_example(200).await;
        assert!(result.contains("Result"));
        assert!(result.contains("30"));
    }

    #[tokio::test]
    async fn test_timeout_failure() {
        let result = timeout_example(50).await;
        assert_eq!(result, "Timeout!");
    }
}
