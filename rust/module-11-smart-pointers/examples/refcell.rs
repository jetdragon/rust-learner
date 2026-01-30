// examples/refcell.rs
// Cell 和 RefCell 内部可变性演示

use std::cell::{Cell, RefCell};
use std::rc::Rc;

fn main() {
    println!("=== Cell 和 RefCell 演示 ===\n");

    // 1. Cell 基础
    println!("1. Cell 基础:");
    let x = Cell::new(42);
    println!("  初始值: {}", x.get());
    x.set(100);
    println!("  修改后: {}", x.get());

    // 2. Cell 适用于 Copy 类型
    println!("\n2. Cell 适用于 Copy 类型:");
    struct Point {
        x: Cell<i32>,
        y: Cell<i32>,
    }

    let point = Point {
        x: Cell::new(10),
        y: Cell::new(20),
    };
    println!("  初始点: ({}, {})", point.x.get(), point.y.get());
    point.x.set(15);
    point.y.set(25);
    println!("  修改后: ({}, {})", point.x.get(), point.y.get());

    // Cell 不能用于非 Copy 类型
    // let s = Cell::new(String::from("hello"));  // 编译错误！

    // 3. RefCell 基础
    println!("\n3. RefCell 基础:");
    let data = RefCell::new(vec![1, 2, 3]);
    {
        let mut borrow = data.borrow_mut();
        borrow.push(4);
        borrow.push(5);
        println!("  可变借用: {:?}", borrow);
    }
    let borrow = data.borrow();
    println!("  不可变借用: {:?}", borrow);

    // 4. RefCell 借用规则（运行时检查）
    println!("\n4. RefCell 借用规则:");
    let data = RefCell::new(5);

    // 多个不可变借用 OK
    let r1 = data.borrow();
    let r2 = data.borrow();
    println!("  多个不可变借用: {} = {}", *r1, *r2);
    drop(r1);
    drop(r2);

    // 可变借用 OK
    let mut w = data.borrow_mut();
    *w = 10;
    println!("  可变借用后: {}", *w);
    drop(w);

    // 同时存在不可变和可变借用会 panic!
    // let r = data.borrow();
    // let mut w = data.borrow_mut();  // panic!

    // 5. 实际应用：模拟可变字段
    println!("\n5. 实际应用：模拟可变字段");
    struct Counter {
        count: RefCell<u32>,
    }

    impl Counter {
        fn new() -> Self {
            Counter {
                count: RefCell::new(0),
            }
        }

        fn increment(&self) {
            let mut count = self.count.borrow_mut();
            *count += 1;
        }

        fn get(&self) -> u32 {
            *self.count.borrow()
        }
    }

    let counter = Counter::new();
    println!("  初始计数: {}", counter.get());
    counter.increment();
    counter.increment();
    counter.increment();
    println!("  增加后: {}", counter.get());

    // 6. Rc + RefCell 组合
    println!("\n6. Rc + RefCell 组合:");
    struct Node {
        value: i32,
        next: RefCell<Option<Rc<Node>>>,
    }

    let a = Rc::new(Node {
        value: 1,
        next: RefCell::new(None),
    });

    let b = Rc::new(Node {
        value: 2,
        next: RefCell::new(Some(Rc::clone(&a))),
    });

    let c = Rc::new(Node {
        value: 3,
        next: RefCell::new(Some(Rc::clone(&a))),
    });

    // 修改 a 的 next
    *a.next.borrow_mut() = Some(Rc::clone(&b));

    println!(
        "  a 的 next: {:?}",
        a.next.borrow().as_ref().map(|n| n.value)
    );
    println!(
        "  b 的 next: {:?}",
        b.next.borrow().as_ref().map(|n| n.value)
    );
    println!(
        "  c 的 next: {:?}",
        c.next.borrow().as_ref().map(|n| n.value)
    );

    // 7. 错误处理：borrow_mut 可能失败
    println!("\n7. 错误处理:");
    let data = RefCell::new(5);
    let borrow1 = data.borrow();
    match data.try_borrow_mut() {
        Ok(mut_borrow) => {
            *mut_borrow = 10;
            println!("  修改成功");
        }
        Err(e) => {
            println!("  借用失败: {:?}", e);
        }
    }
    drop(borrow1);

    // 8. 实际应用：观察者模式
    println!("\n8. 实际应用：观察者模式");
    use std::cell::RefCell;

    trait Observer {
        fn notify(&self, message: &str);
    }

    struct Subject {
        observers: RefCell<Vec<Rc<dyn Observer>>>,
    }

    impl Subject {
        fn new() -> Self {
            Subject {
                observers: RefCell::new(vec![]),
            }
        }

        fn attach(&self, observer: Rc<dyn Observer>) {
            self.observers.borrow_mut().push(observer);
        }

        fn notify_all(&self, message: &str) {
            for observer in self.observers.borrow().iter() {
                observer.notify(message);
            }
        }
    }

    struct ConsoleObserver;
    impl Observer for ConsoleObserver {
        fn notify(&self, message: &str) {
            println!("  通知: {}", message);
        }
    }

    let subject = Subject::new();
    subject.attach(Rc::new(ConsoleObserver));
    subject.attach(Rc::new(ConsoleObserver));

    subject.notify_all("事件发生！");
}
