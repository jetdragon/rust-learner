# 所有权系统 (Ownership)

## 模块简介

所有权（Ownership）是 Rust 最独特、最重要的特性。它让 Rust 无需垃圾回收（GC）就能保证内存安全，是 Rust 的核心魅力所在。本模块将深入理解所有权规则，这是掌握 Rust 的必经之路。

## 学习目标

完成本模块后，你将能够：

- 理解 Rust 所有权系统的三条基本规则
- 掌握变量作用域与内存管理
- 理解移动（Move）语义和拷贝（Copy）语义
- 掌握借用（Borrowing）规则：不可变借用和可变借用
- 理解引用（References）的使用
- 掌握切片（Slices）类型的使用

## 前置知识

在学习本模块之前，你应该掌握：

- [module-01-basics](../module-01-basics/) - 变量、数据类型、函数基础

## 核心概念

### 所有权三条规则

Rust 所有权系统遵循三条基本规则：

1. **每个值都有一个变量作为它的所有者**
2. **同一时间只能有一个所有者**
3. **当所有者超出作用域，值将被丢弃**

```rust
{
    let s = String::from("hello");  // s 是 "hello" 的所有者
    // 使用 s...
}  // 作用域结束，s 被丢弃，内存被释放
```

### 变量作用域

作用域是变量在程序中有效的范围：

```rust
{                       // s 在此处无效，尚未声明
    let s = "hello";    // s 从此处开始有效
    // 使用 s...
}                       // 作用域结束，s 不再有效
```

### String 类型

为了理解所有权，需要先了解 String 类型（不同于字符串字面量 &str）：

```rust
let s = String::from("hello");
// String 由三部分组成：
// - 指向堆上数据的指针
// - 长度（len）
// - 容量（capacity）
```

### 移动语义 (Move Semantics)

当将一个变量赋值给另一个变量时，默认发生**移动**而非拷贝：

```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 的所有权移动到 s2
// println!("{}, world!", s1);  // 错误！s1 已失效
println!("{}, world!", s2);  // 正确
```

**为什么会移动？**

String 包含指向堆的指针，如果简单拷贝，会导致：
- 两个指针指向同一块内存
- 作用域结束时两次释放同一块内存（double free）→ 内存错误

Rust 通过移动语义避免了这个问题，保证内存安全。

### 拷贝语义 (Copy Semantics)

某些类型实现了 `Copy` trait，赋值时会自动拷贝而非移动：

```rust
let x = 5;
let y = x;  // i32 实现了 Copy，所以 x 仍然有效
println!("x = {}, y = {}", x, y);  // 都可以使用
```

**实现 Copy 的类型**：
- 所有整数类型（i32, u8, etc.）
- 布尔类型 bool
- 浮点类型（f32, f64）
- 字符类型 char
- 元组（如果所有元素都是 Copy 的）

### 函数调用与所有权

函数调用也会发生所有权的转移：

```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s);  // s 的所有权移动到函数
    // println!("{}", s);  // 错误！s 已失效

    let x = 5;
    makes_copy(x);  // i32 是 Copy 类型，x 仍然有效
    println!("{}", x);  // 正确
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}  // some_string 被丢弃

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}  // some_integer 超出作用域，但因为是 Copy，无特殊处理
```

### 返回值与作用域

返回值也可以转移所有权：

```rust
fn main() {
    let s1 = gives_ownership();  // 函数返回值的所有权转移给 s1
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);  // s2 的所有权转移到函数，再转移给 s3
}

fn gives_ownership() -> String {
    String::from("yours")  // 返回的 String 移动给调用者
}

fn takes_and_gives_back(mut s: String) -> String {
    s.push_str(", world");
    s  // 返回修改后的 String，所有权转移给调用者
}
```

### 引用与借用 (References & Borrowing)

使用引用（&）可以在不获取所有权的情况下访问值：

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);  // 借用 s1
    println!("'{}' 的长度是 {}", s1, len);  // s1 仍然有效
}

fn calculate_length(s: &String) -> usize {
    s.len()
}  // s 超出作用域，但因为它不拥有值，所以不会丢弃
```

**借用规则**（非常重要）：

1. **同一时间，只能有一个可变引用**
2. **或者可以有多个不可变引用**
3. **引用必须始终有效**

```rust
let mut s = String::from("hello");

// 可变引用
let r1 = &mut s;
r1.push_str(", world");
// println!("{}", r1);  // 如果这里使用 r1...

// 不可变引用
let len = calculate_length(&s);  // 错误！不能在可变引用作用域内创建不可变引用

// 正确的做法：
let mut s = String::from("hello");
{
    let r1 = &mut s;
    r1.push_str(", world");
}  // r1 作用域结束

let len = calculate_length(&s);  // 现在可以了
```

### 可变引用

使用 `&mut T` 创建可变引用：

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);  // 传入可变引用
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

### 悬垂引用（Dangling References）

Rust 编译器会防止悬垂引用（引用了已释放的内存）：

```rust
// ❌ 错误示例
fn dangle() -> &String {  // 返回字符串引用
    let s = String::from("hello");
    &s  // 返回 s 的引用
}  // s 被丢弃，内存释放！返回了悬垂引用

// ✅ 正确做法：直接返回 String
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // s 的所有权转移给调用者
}
```

### 切片 (Slices)

切片是对序列（数组、字符串等）的引用，不拥有数据。

**字符串切片**：

```rust
let s = String::from("hello world");
let hello = &s[0..5];   // "hello"
let world = &s[6..11];  // "world"
let whole = &s[0..];    // "hello world" (从 0 开始可省略起始)
let last = &s[..];      // 整个字符串 (可省略结束)
```

**字符串字面量就是切片**：

```rust
let s: &str = "Hello, world!";  // s 是 &str 类型
```

**数组切片**：

```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];  // &[i32] 类型，包含 [2, 3]
```

## 代码示例

本模块的代码示例位于 `examples/` 目录：

- `ownership_basics.rs` - 所有权基本规则演示
- `move_semantics.rs` - 移动语义演示
- `borrowing.rs` - 借用规则演示
- `references.rs` - 引用使用演示
- `slices.rs` - 切片类型演示

运行示例：

```bash
# 编译并运行特定示例
cargo run -p module-02-ownership --example ownership_basics

# 或使用 cargo 的简化命令
cargo run -p module-02-ownership --example move_semantics
```

## 练习题

查看 [exercises.md](exercises.md) 完成练习题。

## 综合练习

查看 [综合练习.md](综合练习.md) 进行综合实践。

## 自检清单

完成学习后，使用 [自检清单.md](自检清单.md) 检查掌握情况。

## 常见问题

### Q: 为什么 Rust 不使用垃圾回收（GC）？

A: GC 需要运行时追踪和管理内存，会带来性能开销和不可预测的暂停。Rust 通过所有权在编译时管理内存，无需运行时开销，性能可预测。

### Q: 什么时候用移动，什么时候用引用？

A:
- **移动**：当需要转移数据所有权时（如函数需要持有数据）
- **引用**：当只需要读取或临时修改数据时（如函数计算长度）

### Q: 为什么需要借用规则？

A: 借用规则防止了数据竞争（data race），这是并发编程中常见且难以调试的 bug。在编译时捕获这些问题比运行时崩溃好得多。

### Q: `String` 和 `&str` 有什么区别？

A:
- `String`：拥有的、可增长的、堆分配的字符串
- `&str`：字符串切片，不可变，引用其他地方存储的字符串数据

### Q: 什么时候会发生悬垂引用？

A: 当引用的生命周期超过了它指向的数据的生命周期。Rust 编译器通过借用检查器在编译时防止这种情况。

## 所有权系统的优势

1. **内存安全**：无需手动管理内存，也不会有 double free
2. **无 GC 开销**：编译时管理，零运行时成本
3. **并发安全**：借用规则防止数据竞争
4. **高效**：只在必要时拷贝数据

## 下一步

完成本模块后，继续学习：

- [module-03-structs-enums](../module-03-structs-enums/) - 使用结构体组织相关数据
