// examples/rc_example.rs
// Rc 引用计数演示

use std::rc::Rc;

fn main() {
    println!("=== Rc 引用计数演示 ===\n");

    // 1. Rc 基础
    println!("1. Rc 基础:");
    let a = Rc::new(5);
    let b = Rc::clone(&a);
    let c = Rc::clone(&a);

    println!("  a = {}", a);
    println!("  b = {}", b);
    println!("  c = {}", c);
    println!("  引用计数: {}", Rc::strong_count(&a)); // 3

    // 2. 引用计数变化
    println!("\n2. 引用计数变化:");
    {
        let d = Rc::clone(&a);
        println!("  创建 d 后的引用计数: {}", Rc::strong_count(&a)); // 4
    }
    // d 离开作用域
    println!("  d 离开作用域后: {}", Rc::strong_count(&a)); // 3

    // 3. Rc 与 Cons List
    println!("\n3. Rc 与共享 List:");
    #[derive(Debug)]
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("  a = {:?}", a);
    println!("  a 的引用计数: {}", Rc::strong_count(&a)); // 1

    let b = Cons(3, Rc::clone(&a));
    println!("  b = {:?}", b);
    println!("  a 的引用计数 (b 共享后）: {}", Rc::strong_count(&a)); // 2

    let c = Cons(4, Rc::clone(&a));
    println!("  c = {:?}", c);
    println!("  a 的引用计数 (c 共享后）: {}", Rc::strong_count(&a)); // 3

    // 4. 实际应用：共享配置
    println!("\n4. 实际应用：共享配置:");
    use std::collections::HashMap;

    #[derive(Debug, Clone)]
    struct Config {
        name: String,
        timeout: u32,
    }

    let config = Rc::new(Config {
        name: String::from("App Config"),
        timeout: 30,
    });

    struct Worker {
        id: usize,
        config: Rc<Config>,
    }

    let mut workers = vec![];
    for i in 0..3 {
        let worker = Worker {
            id: i,
            config: Rc::clone(&config),
        };
        workers.push(worker);
    }

    println!("  所有工人共享同一个配置: {}", workers.len());
    for worker in &workers {
        println!("    Worker {}: {}", worker.id, worker.config.name);
    }

    // 修改配置（所有工人都会看到）
    // 注意：Rc 内部的数据不可变，需要配合内部可变性

    // 5. Rc 的性能考虑
    println!("\n5. Rc 性能考虑:");
    println!("  Rc::clone() 很快（只增加引用计数）");
    println!("  不复制底层数据");
    println!("  适用于需要共享的不可变数据");
}
