// examples/vecdeque.rs
// VecDeque 双端队列演示

fn main() {
    println!("=== VecDeque 演示 ===\n");

    use std::collections::VecDeque;

    // 1. VecDeque 基础
    println!("1. VecDeque 基础:");
    let mut deque: VecDeque<i32> = VecDeque::new();

    deque.push_back(1);
    deque.push_back(2);
    deque.push_front(0);
    deque.push_front(-1);

    println!("  双端队列内容: {:?}", deque);
    println!("  从前到后: {:?}", deque.iter().collect::<Vec<_>>());

    // 2. 前后端操作
    println!("\n2. 前后端操作:");
    let mut deque = VecDeque::new();

    // 从后端添加
    deque.push_back(10);
    deque.push_back(20);
    deque.push_back(30);
    println!("  push_back 10, 20, 30: {:?}", deque);

    // 从前端添加
    deque.push_front(5);
    println!("  push_front 5: {:?}", deque);

    // 从后端移除
    if let Some(value) = deque.pop_back() {
        println!("  pop_back: {}", value);
    }
    println!("  当前: {:?}", deque);

    // 从前端移除
    if let Some(value) = deque.pop_front() {
        println!("  pop_front: {}", value);
    }
    println!("  当前: {:?}", deque);

    // 3. 访问元素
    println!("\n3. 访问元素:");
    let deque: VecDeque<_> = vec![10, 20, 30, 40, 50].into_iter().collect();

    println!("  队列: {:?}", deque);
    println!("  front(): {:?}", deque.front()); // Some(10)
    println!("  back(): {:?}", deque.back()); // Some(50)
    println!("  [0]: {}", deque[0]); // 10
    println!("  [2]: {}", deque[2]); // 30
    println!("  [4]: {}", deque[4]); // 50

    // 4. 实现队列（FIFO）
    println!("\n4. 实现队列（先进先出）:");
    struct Queue<T> {
        data: VecDeque<T>,
    }

    impl<T> Queue<T> {
        fn new() -> Self {
            Queue {
                data: VecDeque::new(),
            }
        }

        fn enqueue(&mut self, item: T) {
            self.data.push_back(item);
        }

        fn dequeue(&mut self) -> Option<T> {
            self.data.pop_front()
        }

        fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        fn len(&self) -> usize {
            self.data.len()
        }
    }

    let mut queue = Queue::new();
    queue.enqueue("第一个");
    queue.enqueue("第二个");
    queue.enqueue("第三个");

    println!("  队列长度: {}", queue.len());
    while let Some(item) = queue.dequeue() {
        println!("  出队: {}", item);
    }
    println!("  队列是否为空: {}", queue.is_empty());

    // 5. 实现栈（LIFO）
    println!("\n5. 实现栈（后进先出）:");
    struct Stack<T> {
        data: VecDeque<T>,
    }

    impl<T> Stack<T> {
        fn new() -> Self {
            Stack {
                data: VecDeque::new(),
            }
        }

        fn push(&mut self, item: T) {
            self.data.push_back(item);
        }

        fn pop(&mut self) -> Option<T> {
            self.data.pop_back()
        }

        fn peek(&self) -> Option<&T> {
            self.data.back()
        }
    }

    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("  栈顶: {:?}", stack.peek());
    while let Some(item) = stack.pop() {
        println!("  出栈: {}", item);
    }

    // 6. 滑动窗口
    println!("\n6. 滑动窗口应用:");
    fn sliding_window(nums: &[i32], k: usize) -> Vec<i32> {
        let mut result = Vec::new();
        let mut window: VecDeque<usize> = VecDeque::new();

        for (i, &num) in nums.iter().enumerate() {
            // 移除超出窗口的元素
            while let Some(&front) = window.front() {
                if front <= i - k {
                    window.pop_front();
                } else {
                    break;
                }
            }

            // 维护单调递减
            while let Some(&back) = window.back() {
                if nums[back] <= num {
                    window.pop_back();
                } else {
                    break;
                }
            }

            window.push_back(i);

            // 记录窗口最大值
            if i >= k - 1 {
                result.push(nums[*window.front().unwrap()]);
            }
        }

        result
    }

    let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let k = 3;
    let max_values = sliding_window(&nums, k);
    println!("  数组: {:?}", nums);
    println!("  窗口大小: {}", k);
    println!("  滑动窗口最大值: {:?}", max_values);

    // 7. 实际应用：任务调度器
    println!("\n7. 实际应用：任务调度器:");
    #[derive(Debug, Clone)]
    struct Task {
        name: String,
        priority: u32,
    }

    struct TaskScheduler {
        high_priority: VecDeque<Task>,
        normal_priority: VecDeque<Task>,
        low_priority: VecDeque<Task>,
    }

    impl TaskScheduler {
        fn new() -> Self {
            TaskScheduler {
                high_priority: VecDeque::new(),
                normal_priority: VecDeque::new(),
                low_priority: VecDeque::new(),
            }
        }

        fn add_task(&mut self, task: Task) {
            if task.priority >= 80 {
                self.high_priority.push_back(task);
            } else if task.priority >= 50 {
                self.normal_priority.push_back(task);
            } else {
                self.low_priority.push_back(task);
            }
        }

        fn get_next_task(&mut self) -> Option<Task> {
            if let Some(task) = self.high_priority.pop_front() {
                Some(task)
            } else if let Some(task) = self.normal_priority.pop_front() {
                Some(task)
            } else if let Some(task) = self.low_priority.pop_front() {
                Some(task)
            } else {
                None
            }
        }
    }

    let mut scheduler = TaskScheduler::new();
    scheduler.add_task(Task {
        name: String::from("低优先级任务"),
        priority: 30,
    });
    scheduler.add_task(Task {
        name: String::from("高优先级任务"),
        priority: 90,
    });
    scheduler.add_task(Task {
        name: String::from("普通任务"),
        priority: 60,
    });
    scheduler.add_task(Task {
        name: String::from("另一个高优先级"),
        priority: 85,
    });

    println!("  执行顺序:");
    while let Some(task) = scheduler.get_next_task() {
        println!("    - {} (优先级: {})", task.name, task.priority);
    }

    // 8. VecDeque 与 Vec 的性能对比
    println!("\n8. VecDeque vs Vec 性能对比:");
    use std::time::Instant;

    const N: usize = 10_000;

    // Vec 在头部插入（慢）
    let start = Instant::now();
    let mut vec = Vec::new();
    for i in 0..N {
        vec.insert(0, i); // O(n) - 需要移动所有元素
    }
    let vec_insert_front_time = start.elapsed();
    println!("  Vec 在头部插入 {} 个元素: {:?}", N, vec_insert_front_time);

    // VecDeque 在前端插入（快）
    let start = Instant::now();
    let mut deque = VecDeque::new();
    for i in 0..N {
        deque.push_front(i); // O(1) - 常数时间
    }
    let deque_insert_front_time = start.elapsed();
    println!(
        "  VecDeque 在前端插入 {} 个元素: {:?}",
        N, deque_insert_front_time
    );

    // 比较差异
    let ratio = vec_insert_front_time.as_nanos() as f64 / deque_insert_front_time.as_nanos() as f64;
    println!("  Vec 比 VecDeque 慢约 {:.2} 倍", ratio);

    println!("\n  结论:");
    println!("    VecDeque 在两端操作时效率更高（O(1)）");
    println!("    Vec 只在尾部操作时高效（O(1)）");
    println!("    频繁在两端操作时优先选择 VecDeque");
}
