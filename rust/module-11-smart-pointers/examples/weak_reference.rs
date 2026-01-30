// examples/weak_reference.rs
// Weak 引用和循环引用演示

use std::cell::RefCell;
use std::rc::{Rc, Weak};

fn main() {
    println!("=== Weak 引用和循环引用演示 ===\n");

    // 1. Weak 基础
    println!("1. Weak 基础:");
    let strong = Rc::new(5);
    let weak = Rc::downgrade(&strong);

    println!("  Strong 引用计数: {}", Rc::strong_count(&strong));
    println!("  Weak 升级: {:?}", weak.upgrade()); // Some(5)

    drop(strong);

    println!("  Strong 释放后:");
    println!("  Weak 升级: {:?}", weak.upgrade()); // None

    // 2. 循环引用问题
    println!("\n2. 循环引用问题:");
    struct Node {
        value: i32,
        next: RefCell<Option<Rc<Node>>>,
    }

    // 创建循环
    let a = Rc::new(Node {
        value: 1,
        next: RefCell::new(None),
    });
    let b = Rc::new(Node {
        value: 2,
        next: RefCell::new(None),
    });

    *a.next.borrow_mut() = Some(Rc::clone(&b));
    *b.next.borrow_mut() = Some(Rc::clone(&a));

    println!("  a 的引用计数: {}", Rc::strong_count(&a)); // 2
    println!("  b 的引用计数: {}", Rc::strong_count(&b)); // 2

    println!("  内存泄漏！循环引用导致永远无法释放");

    // 3. 使用 Weak 打破循环
    println!("\n3. 使用 Weak 打破循环:");
    struct SmartNode {
        value: i32,
        parent: RefCell<Weak<SmartNode>>, // 使用 Weak
        children: RefCell<Vec<Rc<SmartNode>>>,
    }

    impl SmartNode {
        fn new(value: i32) -> Self {
            SmartNode {
                value,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![]),
            }
        }

        fn add_child(parent: &Rc<SmartNode>, child: Rc<SmartNode>) {
            *child.parent.borrow_mut() = Rc::downgrade(parent);
            parent.children.borrow_mut().push(child);
        }
    }

    let parent = Rc::new(SmartNode::new(1));
    let child1 = Rc::new(SmartNode::new(2));
    let child2 = Rc::new(SmartNode::new(3));

    SmartNode::add_child(&parent, child1);
    SmartNode::add_child(&parent, child2);

    println!("  父节点引用计数: {}", Rc::strong_count(&parent)); // 3
    println!("  父节点可访问:");
    if let Some(p) = parent.parent.borrow().upgrade() {
        println!("    父节点的父节点: {}", p.value);
    } else {
        println!("    没有父节点");
    }

    // 4. 树遍历
    println!("\n4. 树遍历（从子节点找父节点）:");
    fn print_tree(node: &Rc<SmartNode>, depth: usize) {
        println!("  {}节点 {}", "  ".repeat(depth), node.value);

        for child in node.children.borrow().iter() {
            print_tree(child, depth + 1);
        }
    }

    println!("  树结构:");
    print_tree(&parent, 0);

    println!("\n  从子节点向上:");
    if let Some(child) = parent.children.borrow().first() {
        let mut current = Some(Rc::clone(child));
        let mut depth = 0;

        while let Some(node) = current {
            println!("  {}节点 {}", "  ".repeat(depth), node.value);

            if let Some(parent) = node.parent.borrow().upgrade() {
                current = Some(parent);
                depth += 1;
            } else {
                current = None;
            }
        }
    }

    // 5. 缓存实现（使用 Weak）
    println!("\n5. 缓存实现（使用 Weak）:");
    use std::collections::HashMap;

    #[derive(Debug)]
    struct CachedValue {
        key: String,
        value: i32,
    }

    struct Cache {
        data: RefCell<HashMap<String, Rc<CachedValue>>>,
        // 使用 Weak 存储对数据的弱引用
        // 这样即使外部持有 Rc，也不会阻止 GC（如果有）
    }

    impl Cache {
        fn new() -> Self {
            Cache {
                data: RefCell::new(HashMap::new()),
            }
        }

        fn get(&self, key: &str) -> Option<Rc<CachedValue>> {
            self.data.borrow().get(key).cloned()
        }

        fn set(&self, key: String, value: i32) {
            let cached = Rc::new(CachedValue {
                key: key.clone(),
                value,
            });
            self.data.borrow_mut().insert(key, cached);
        }

        fn clear(&self) {
            self.data.borrow_mut().clear();
        }
    }

    let cache = Cache::new();
    cache.set("key1".to_string(), 100);
    cache.set("key2".to_string(), 200);

    if let Some(cached) = cache.get("key1") {
        println!("  缓存命中: {:?}", cached);
    }

    // 6. 观察者模式（使用 Weak）
    println!("\n6. 观察者模式（使用 Weak）:");
    struct Subject {
        observers: RefCell<Vec<Weak<dyn Observer>>>,
    }

    trait Observer {
        fn notify(&self, message: &str);
    }

    impl Subject {
        fn new() -> Self {
            Subject {
                observers: RefCell::new(vec![]),
            }
        }

        fn attach(&self, observer: Rc<dyn Observer>) {
            self.observers.borrow_mut().push(Rc::downgrade(&observer));
        }

        fn notify_all(&self, message: &str) {
            self.observers.borrow_mut().retain(|weak| {
                if let Some(observer) = weak.upgrade() {
                    observer.notify(message);
                    true // 保留存活的观察者
                } else {
                    false // 移除失效的观察者
                }
            });
        }
    }

    struct PrintObserver {
        id: usize,
    }

    impl Observer for PrintObserver {
        fn notify(&self, message: &str) {
            println!("  观察者 {}: {}", self.id, message);
        }
    }

    let subject = Subject::new();
    let obs1 = Rc::new(PrintObserver { id: 1 });
    let obs2 = Rc::new(PrintObserver { id: 2 });

    subject.attach(obs1.clone());
    subject.attach(obs2.clone());

    println!("  通知所有观察者:");
    subject.notify_all("第一个消息");

    println!("  移除观察者 1");
    drop(obs1);

    println!("  再次通知:");
    subject.notify_all("第二个消息");

    // 7. 实际应用：对象池
    println!("\n7. 实际应用：对象池");
    use std::cell::RefCell;

    struct ObjectPool<T> {
        pool: RefCell<Vec<T>>,
    }

    impl<T: Default> ObjectPool<T> {
        fn new(size: usize) -> Self {
            let pool = (0..size).map(|_| T::default()).collect();
            ObjectPool {
                pool: RefCell::new(pool),
            }
        }

        fn acquire(&self) -> Option<T> {
            self.pool.borrow_mut().pop()
        }

        fn release(&self, item: T) {
            self.pool.borrow_mut().push(item);
        }

        fn available(&self) -> usize {
            self.pool.borrow().len()
        }
    }

    #[derive(Debug, Default)]
    struct Resource {
        id: usize,
    }

    let pool = ObjectPool::<Resource>::new(3);
    println!("  可用资源: {}", pool.available());

    if let Some(resource) = pool.acquire() {
        println!("  获取资源: {:?}", resource);
        pool.release(resource);
    }

    println!("  释放后可用资源: {}", pool.available());

    // 8. 总结
    println!("\n8. Weak 引用总结:");
    println!("  ✅ 避免循环引用导致的内存泄漏");
    println!("  ✅ 允许从子节点引用父节点");
    println!("  ✅ 自动清理失效引用");
    println!("  ⚠️  需要手动 upgrade() 并检查 Option");
    println!("  ⚠️  性能略低于 Rc");
}
