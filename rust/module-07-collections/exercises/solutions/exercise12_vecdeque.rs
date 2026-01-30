//! # 练习 12: VecDeque 双端操作解答
//!
//! **解答要点:**
//! 1. VecDeque 支持 push_front/pop_front 和 push_back/pop_back
//! 2. 双端队列可用于实现队列和栈
//! 3. 从两端访问使用 front() 和 back()

use std::collections::VecDeque;

/// 演示 VecDeque 的双端操作
pub fn deque_operations() -> (Vec<i32>, Vec<i32>) {
    let mut deque = VecDeque::new();

    // 从后端添加
    deque.push_back(1);
    deque.push_back(2);
    deque.push_back(3);

    // 从前端添加
    deque.push_front(0);
    deque.push_front(-1);

    (deque.iter().cloned().collect(), vec![-1, 0, 1, 2, 3])
}

/// 模拟队列（FIFO）
pub fn simulate_queue() -> Vec<i32> {
    let mut queue = VecDeque::new();

    // 入队
    queue.push_back(1);
    queue.push_back(2);
    queue.push_back(3);

    // 出队
    let mut result = vec![];
    while let Some(val) = queue.pop_front() {
        result.push(val);
    }

    result
}

/// 模拟栈（LIFO）
pub fn simulate_stack() -> Vec<i32> {
    let mut stack = VecDeque::new();

    // 入栈
    stack.push_back(1);
    stack.push_back(2);
    stack.push_back(3);

    // 出栈
    let mut result = vec![];
    while let Some(val) = stack.pop_back() {
        result.push(val);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deque_operations() {
        let (result, expected) = deque_operations();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_simulate_queue() {
        let result = simulate_queue();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_simulate_stack() {
        let result = simulate_stack();
        assert_eq!(result, vec![3, 2, 1]);
    }
}
