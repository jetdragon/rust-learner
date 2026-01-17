# 练习题

本模块包含以下练习题，帮助你巩固 Rust 智能指针的知识。

## 练习 1：Box 基础使用

**难度**：简单

**描述**：

使用 Box 在堆上存储大数据。

**代码框架**：

```rust
// exercises/box_basics.rs

fn main() {
    // TODO: 创建一个包含 10000 个元素的 Box<Vec<i32>>
    // TODO: 使用 Box 存储大型结构体
}
```

**要求**：

- 使用 `Box::new()` 在堆上创建数据
- 理解 Box 的自动解引用
- 访问 Box 中的数据

**提示**：

<details>
<summary>查看提示</summary>

- `Box::new(data)` 在堆上分配数据
- 访问时自动解引用：`boxed_data.field`
- `*boxed_value` 显式解引用
</details>

**验证**：

```bash
cargo test -p module-11-smart-pointers -- test_box_basics
```

---

## 练习 2：递归类型使用 Box

**难度**：中等

**描述**：

使用 Box 实现递归的链表结构。

**代码框架**：

```rust
// exercises/recursive_list.rs

// TODO: 定义 List 枚举，使用 Box 处理递归
enum List {
    Cons(i32, /* TODO */),
    Nil,
}

fn main() {
    // TODO: 创建一个链表：1 -> 2 -> 3 -> Nil
    let list = /* TODO */;
}
```

**要求**：

- 使用 `Box` 解决递归类型大小未知的问题
- 创建 Cons 列表结构
- 理解为什么递归类型需要 Box

**提示**：

<details>
<summary>查看提示</summary>

- Rust 无法在编译时确定递归类型的大小
- `Box` 提供固定大小的指针，指向堆上的实际数据
- `Cons(i32, Box<List>)` 是正确的模式
</details>

**验证**：

```bash
cargo test -p module-11-smart-pointers -- test_recursive_list
```

---

## 练习 3：Rc 多所有权

**难度**：中等

**描述**：

使用 Rc 实现多个所有者共享同一数据。

**代码框架**：

```rust
// exercises/rc_multiple_owners.rs

use std::rc::Rc;

struct SharedData {
    value: i32,
}

fn main() {
    // TODO: 创建 Rc<SharedData>
    // TODO: 创建多个 Rc 引用（克隆）
    // TODO: 验证引用计数

    let data = /* TODO */;
    let owner1 = /* TODO */;
    let owner2 = /* TODO */;
    let owner3 = /* TODO */;

    println!("当前引用计数: {}", Rc::strong_count(&data));
}
```

**要求**：

- 使用 `Rc::new()` 创建共享数据
- 使用 `Rc::clone()` 创建新的所有权
- 使用 `Rc::strong_count()` 检查引用计数
- 理解 Rc 只能在单线程使用

**提示**：

<details>
<summary>查看提示</summary>

- `Rc::clone()` 不会克隆数据，只会增加引用计数
- `Rc::strong_count(&rc_value)` 返回强引用数量
- Rc 不是深度克隆，只是增加所有权引用
</details>

**验证**：

```bash
cargo test -p module-11-smart-pointers -- test_rc_multiple_owners
```

---

## 练习 4：Arc 并发共享

**难度**：困难

**描述**：

使用 Arc 在多个线程间共享数据。

**代码框架**：

```rust
// exercises/arc_shared.rs

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // TODO: 创建 Arc<Mutex<Vec<i32>>> 用于在多个线程间共享数据
    // TODO: 创建多个线程，每个线程向 Vec 添加元素
    // TODO: 等待所有线程完成
    // TODO: 打印最终结果

    let data = /* TODO */;

    let mut handles = vec![];

    for i in 0..3 {
        let data_clone = /* TODO */;
        let handle = thread::spawn(move || {
            // TODO: 获取锁并修改数据
        });
        handles.push(handle);
    }

    for handle in handles {
        /* TODO */;
    }

    println!("最终数据: {:?}", *data.lock().unwrap());
}
```

**要求**：

- 使用 `Arc::new()` 创建线程安全的共享数据
- 使用 `Arc::clone()` 为每个线程创建引用
- 结合 `Mutex` 实现安全的并发修改
- 等待所有线程完成（`join()`）

**提示**：

<details>
<summary>查看提示</summary>

- `Arc<T>` 是 Rc 的线程安全版本
- 并发修改需要 `Mutex<T>` 提供内部可变性
- `data.lock().unwrap()` 获取 MutexGuard
- 每个线程需要独立的 `Arc` 克隆
</details>

**验证**：

```bash
cargo test -p module-11-smart-pointers -- test_arc_shared
```

---

## 练习 5：Cell 内部可变性

**难度**：中等

**描述**：

使用 Cell 实现在不可变引用下修改字段。

**代码框架**：

```rust
// exercises/cell_interior.rs

use std::cell::Cell;

struct Counter {
    count: Cell<i32>,
}

impl Counter {
    fn new() -> Self {
        Counter { count: Cell::new(0) }
    }

    fn increment(&self) {
        // TODO: 使用 Cell 修改 count
    }

    fn get(&self) -> i32 {
        // TODO: 获取 count 的值
    }
}

fn main() {
    let counter = Counter::new();
    counter.increment();
    counter.increment();
    println!("计数: {}", counter.get());
}
```

**要求**：

- 使用 `Cell::new()` 创建内部可变字段
- 使用 `.set()` 修改 Cell 的值
- 使用 `.get()` 读取 Cell 的值
- 理解 Cell 只适用于实现了 `Copy` 的类型

**提示**：

<details>
<summary>查看提示</summary>

- `Cell::new(value)` 创建新的 Cell
- `cell.set(new_value)` 设置新值
- `cell.get()` 获取当前值
- Cell 没有借用检查，需要小心使用
</details>

**验证**：

```bash
cargo test -p module-11-smart-pointers -- test_cell_interior
```

---

## 练习 6：RefCell 运行时借用检查

**难度**：困难

**描述**：

使用 RefCell 实现运行时借用检查的内部可变性。

**代码框架**：

```rust
// exercises/refcell_borrow.rs

use std::cell::RefCell;

struct DataStore {
    values: RefCell<Vec<i32>>,
}

impl DataStore {
    fn new() -> Self {
        DataStore { values: RefCell::new(vec![]) }
    }

    fn add(&self, value: i32) {
        // TODO: 获取可变借用并添加值
    }

    fn get(&self, index: usize) -> Option<i32> {
        // TODO: 获取不可变借用并返回值
    }

    fn len(&self) -> usize {
        // TODO: 返回向量长度
    }
}

fn main() {
    let store = DataStore::new();
    store.add(1);
    store.add(2);
    store.add(3);

    println!("长度: {}", store.len());
    println!("第一个值: {:?}", store.get(0));
    println!("第二个值: {:?}", store.get(1));
}
```

**要求**：

- 使用 `.borrow_mut()` 获取可变借用
- 使用 `.borrow()` 获取不可变借用
- 理解借用规则在运行时检查
- 避免同时存在可变和不可变借用（会 panic）

**提示**：

<details>
<summary>查看提示</summary>

- `refcell.borrow_mut()` 返回 `RefMut<T>`
- `refcell.borrow()` 返回 `Ref<T>`
- 借用在作用域结束时自动释放
- 违反借用规则会触发 panic
</details>

**验证**：

```bash
cargo test -p module-11-smart-pointers -- test_refcell_borrow
```

---

## 练习 7：Weak 避免循环引用

**难度**：困难

**描述**：

使用 Weak 引用解决循环引用导致的内存泄漏。

**代码框架**：

```rust
// exercises/weak_reference.rs

use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        }
    }

    fn add_child(&self, child: Rc<Node>) {
        // TODO: 添加子节点
        // TODO: 设置子节点的父节点引用为 Weak
    }
}

fn main() {
    let parent = Rc::new(Node::new(1));
    let child1 = Rc::new(Node::new(2));
    let child2 = Rc::new(Node::new(3));

    parent.add_child(child1.clone());
    parent.add_child(child2);

    // TODO: 验证没有循环引用
    println!("父节点引用计数: {}", Rc::strong_count(&parent));
    println!("子节点引用计数: {}", Rc::strong_count(&child1));
}
```

**要求**：

- 使用 `Weak::new()` 创建弱引用
- 理解 Weak 不增加引用计数
- 使用 `weak.upgrade()` 尝试升级为 Rc
- 避免循环引用导致的内存泄漏

**提示**：

<details>
<summary>查看提示</summary>

- `Weak::new()` 创建空的弱引用
- `weak.upgrade()` 尝试升级为 `Option<Rc<T>>`
- 如果数据已被释放，upgrade 返回 None
- Weak 不会阻止数据被释放
</details>

**验证**：

```bash
cargo test -p module-11-smart-pointers -- test_weak_reference
```

---

## 练习 8：Trait Objects 与 Box

**难度**：中等

**描述**：

使用 Box<dyn Trait> 存储不同类型的对象。

**代码框架**：

```rust
// exercises/trait_objects.rs

trait Animal {
    fn speak(&self);
    fn name(&self) -> &str;
}

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

// TODO: 为 Dog 和 Cat 实现 Animal trait

fn make_sounds(animals: &[Box<dyn Animal>]) {
    // TODO: 遍历并让每个动物发声
}

fn main() {
    let animals: Vec<Box<dyn Animal>> = vec![
        // TODO: 创建 Dog 和 Cat 的 Box
    ];

    make_sounds(&animals);
}
```

**要求**：

- 为 Dog 和 Cat 实现 Animal trait
- 使用 `Box::new()` 创建 Trait Object
- 存储在 `Vec<Box<dyn Animal>>` 中
- 理解动态分发 vs 静态分发

**提示**：

<details>
<summary>查看提示</summary>

- `Box<dyn Trait>` 使用动态分发
- 每个方法调用都有虚函数开销
- Trait Object 必须是对象安全的（无泛型参数）
- 使用 `Box::new(Dog { name: ... })` 创建
</details>

**验证**：

```bash
cargo test -p module-11-smart-pointers -- test_trait_objects
```

---

## 练习 9：综合应用 - 文件系统

**难度**：困难

**描述**：

使用智能指针实现简单的文件系统结构，支持父子节点和避免循环引用。

**要求**：

1. 定义 `FileNode` 结构体：
   - `name: String`
   - `parent: RefCell<Weak<FileNode>>`
   - `children: RefCell<Vec<Rc<FileNode>>>`

2. 实现 `FileNode` 方法：
   - `new(name: String) -> Self` - 创建新节点
   - `add_child(&self, child: Rc<FileNode>)` - 添加子节点
   - `get_parent(&self) -> Option<Rc<FileNode>>` - 获取父节点（通过 Weak::upgrade）
   - `list_children(&self) -> Vec<String>` - 列出所有子节点名称

3. 创建文件系统：
   - 根目录
   - 多级子目录
   - 验证没有循环引用

**验证**：

```bash
cargo test -p module-11-smart-pointers -- test_filesystem
```

---

## 练习 10：综合应用 - 图结构

**难度**：困难

**描述**：

实现一个简单的有向图结构，使用 Rc 和 Arc 处理所有权。

**项目需求**：

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct GraphNode {
    id: u32,
    edges: RefCell<Vec<Weak<GraphNode>>>,
}

impl GraphNode {
    fn new(id: u32) -> Self { /* TODO */ }

    fn add_edge(&self, other: Rc<GraphNode>) {
        /* TODO: 添加边，使用 Weak 避免循环引用 */
    }

    fn get_neighbors(&self) -> Vec<Rc<GraphNode>> {
        /* TODO: 返回所有相邻节点 */
    }
}
```

**功能要求**：

1. 创建图结构
2. 添加边（使用 Weak 避免循环）
3. 遍历图（BFS 或 DFS）
4. 检测孤岛节点

**验收标准**：

- [ ] 所有方法正确实现
- [ ] 使用 Weak 避免循环引用
- [ ] 正确处理节点的生命周期
- [ ] 通过所有单元测试

**验证**：

```bash
cargo test -p module-11-smart-pointers -- test_graph_structure
```

---

## 参考答案

完成练习后，可以查看 `exercises/solutions/` 目录中的参考答案对比学习。

⚠️ **注意**：建议先自己完成练习，再查看参考答案。
