# 11-智能指针 (Smart Pointers)

## 模块简介

智能指针是 Rust 中一种重要的数据结构，它不仅仅包含数据的指针，还拥有额外的元数据和功能。理解智能指针是掌握 Rust 所有权系统和内存管理的关键。本模块将深入探讨 Box、Rc、Arc、Cell、RefCell 等智能指针的使用场景和最佳实践。

## 学习目标

完成本模块后，你将能够：

- 理解智能指针的概念和与普通引用的区别
- 掌握 Box 的使用场景（堆分配、递归类型、Trait Objects）
- 掌握 Rc 的多所有权模式
- 掌握 Arc 的并发多所有权
- 理解 Cell 和 RefCell 的内部可变性
- 了解智能指针的性能考虑
- 避免循环引用问题

## 前置知识

在学习本模块之前，你应该掌握：

- [module-01-basics](../module-01-basics/) - 变量、函数基础
- [module-02-ownership](../module-02-ownership/) - 所有权系统
- [module-04-lifetimes](../module-04-lifetimes/) - 生命周期

## 核心概念

### 什么是智能指针

智能指针是一种数据结构，它的表现类似指针，但拥有额外的元数据和功能：

```rust
struct SmartPointer<T> {
    data: *const T,      // 指向数据
    metadata: usize,     // 额外元数据（如引用计数）
}
```

**常见智能指针**：

| 智能指针 | 特性 | 使用场景 |
|---------|------|---------|
| `Box<T>` | 单所有权，堆分配 | 大数据、递归类型、Trait Objects |
| `Rc<T>` | 单线程多所有权 | 图结构、共享数据 |
| `Arc<T>` | 多线程多所有权 | 并发共享数据 |
| `Cell<T>` | 内部可变性（单线程） | 需要内部可变的场景 |
| `RefCell<T>` | 内部可变性 + 借用检查（单线程） | 需要内部可变且借用检查的场景 |

### Box<T>

Box 是最简单的智能指针，提供堆分配和单所有权：

#### 1. 堆分配

```rust
let b = Box::new(5);  // 数据在堆上
println!("b = {}", b);   // 自动解引用
```

**使用场景**：
- 数据太大，不适合放在栈上
- 编译时不知道大小（递归类型）

#### 2. 递归类型

Rust 无法在编译时确定递归类型的大小，需要使用 Box：

```rust
enum List {
    Cons(i32, Box<List>),  // Box 提供已知大小
    Nil,
}

use List::{Cons, Nil};

let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))));
```

#### 3. Trait Objects

当需要存储不同类型的实现了相同 Trait 的值时：

```rust
trait Animal {
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) { println!("Woof!"); }
}

impl Animal for Cat {
    fn speak(&self) { println!("Meow!"); }
}

let animals: Vec<Box<dyn Animal>> = vec
![
    Box::new(Dog),
    Box::new(Cat),
];

for animal in animals {
    animal.speak();
}
```

### Rc<T> (引用计数)

Rc 允许多个所有者共享同一份数据（单线程）：

```rust
use std::rc::Rc;

let a = Rc::new(5);
let b = Rc::clone(&a);  // 增加引用计数
let c = Rc::clone(&a);  // 再增加

println!("a = {}, b = {}, c = {}", a, b, c);
println!("引用计数: {}", Rc::strong_count(&a));  // 3
```

**使用场景**：
- 图数据结构
- 多个所有者需要共享数据
- 需要在编译时不知道数量的共享引用

**重要**：Rc 是**单线程**的，不能跨线程使用。

#### Cons List 示例

```rust
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
let b = Cons(3, Rc::clone(&a));  // 共享 a 的尾部
let c = Cons(4, Rc::clone(&a));  // 共享 a 的尾部

// a 的引用计数现在为 3（a, b, c）
```

### Arc<T> (原子引用计数)

Arc 是 Rc 的线程安全版本，用于多线程场景：

```rust
use std::sync::{Arc, thread};
use std::time::Duration;

let data = Arc::new(vec
![1, 2, 3, 4, 5]);

let mut handles = vec![];

for _ in 0..3 {
    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        let slice = data_clone.as_slice();
        println!("子线程读取: {:?}", slice);
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}
```

**Rc vs Arc**：

| 特性 | Rc | Arc |
|------|-----|-----|
| 线程安全 | ❌ 单线程 | ✅ 多线程 |
| 性能 | 稍快 | 稍慢（原子操作） |
| 引用计数 | `Rc::strong_count()` | `Arc::strong_count()` |

### Cell<T> 和 RefCell<T>

它们提供**内部可变性**（Interior Mutability），允许在只有不可变引用的情况下修改数据。

#### Cell<T>

Cell 适用于实现了 Copy 的类型：

```rust
use std::cell::Cell;

struct Point {
    x: Cell<i32>,
    y: Cell<i32>,
}

let point = Point { x: Cell::new(10), y: Cell::new(20) };
point.x.set(15);  // 通过不可变引用修改
println!("x = {}, y = {}", point.x.get(), point.y.get());
```

**限制**：
- 只适用于实现了 `Copy` 的类型
- 没有借用检查

#### RefCell<T>

RefCell 适用于任意类型，提供运行时借用检查：

```rust
use std::cell::RefCell;

let data = RefCell::new(vec
![1, 2, 3]);

// 获取可变借用
let mut borrow = data.borrow_mut();
borrow.push(4);
borrow.push(5);
// borrow 在作用域结束时自动释放

// 获取不可变借用
let borrow = data.borrow();
println!("{:?}", borrow);
```

**借用规则（运行时检查）**：

```rust
let data = RefCell::new(5);

let borrow1 = data.borrow();    // ✅ 不可变借用
let borrow2 = data.borrow();    // ✅ 多个不可变借用

// let mut_borrow = data.borrow_mut();  // ❌ panic! 同时存在不可变借用
```

**使用场景**：
- 需要在不可变上下文中修改数据
- 需要运行时借用检查（编译时无法确定）

## 循环引用问题

使用 Rc 和 RefCell 时可能产生循环引用，导致内存泄漏：

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Node {
    value: i32,
    next: RefCell<Option<Rc<Node>>>,
}

let a = Rc::new(Node { value: 5, next: RefCell::new(None) });
let b = Rc::new(Node { value: 10, next: RefCell::new(None) });

*a.next.borrow_mut() = Some(Rc::clone(&b));  // a -> b
*b.next.borrow_mut() = Some(Rc::clone(&a));  // b -> a

// 循环引用：a 和 b 的引用计数都是 2，永远不会释放！
```

### 解决方案：Weak

Weak 不增加引用计数：

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,  // 使用 Weak
    children: RefCell<Vec<Rc<Node>>>,
}
```

## 代码示例

本模块的代码示例位于 `examples/` 目录：

- `box_basics.rs` - Box 基础演示
- `rc_example.rs` - Rc 和多所有权演示
- `arc_example.rs` - Arc 并发共享演示
- `refcell.rs` - Cell 和 RefCell 演示
- `cons_list.rs` - 递归类型演示
- `weak_reference.rs` - Weak 引用和循环引用

## 练习题

查看 [exercises.md](exercises.md) 完成练习题。

## 综合练习

查看 [综合练习.md](综合练习.md) 进行综合实践。

## 自检清单

完成学习后，使用 [自检清单.md](自检清单.md) 检查掌握情况。

## 常见问题

### Q: 智能指针和引用有什么区别？

A:
- **引用**：只是借用数据的指针，不拥有数据
- **智能指针**：拥有数据，有额外的元数据和功能

### Q: 什么时候用 Box？

A:
- 数据太大不适合栈
- 递归类型（需要间接）
- Trait Objects（动态分发）

### Q: Rc 和 Arc 如何选择？

A:
- **单线程**：使用 Rc（性能更好）
- **多线程**：使用 Arc（线程安全）

### Q: Cell 和 RefCell 如何选择？

A:
- **类型实现 Copy**：使用 Cell（更简单）
- **需要借用检查**：使用 RefCell（更安全）

### Q: 如何避免循环引用？

A:
- 使用 `Weak` 弱引用
- 设计时避免循环依赖
- 使用所有权树而非图结构

## 最佳实践

1. **优先使用所有权而非 Rc/Arc**：更简单、更安全
2. **避免过度使用 RefCell**：可能导致运行时 panic
3. **注意循环引用**：使用 Weak 或重构设计
4. **单线程用 Rc，多线程用 Arc**
5. **理解借用规则**：即使在 RefCell 中也要遵守

## 下一步

完成本模块后，继续学习：

- [module-12-iterators](../module-12-iterators/) - 迭代器与零成本抽象
- [module-09-concurrency](../module-09-concurrency/) - 并发编程

## 相关资源

- [Rust Book - Box](https://doc.rust-lang.org/book/ch15-01-box.html)
- [Rust Book - Rc](https://doc.rust-lang.org/book/ch15-04-rc.html)
- [Rust Book - RefCell](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html)
- [API 文档 - std::rc](https://doc.rust-lang.org/std/rc/)
- [API 文档 - std::cell](https://doc.rust-lang.org/std/cell/)
