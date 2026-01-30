# 结构体与枚举 (Structs & Enums)

## 模块简介

结构体（struct）和枚举（enum）是 Rust 中创建自定义类型的主要方式。结构体让你可以将相关的数据组合在一起，而枚举则让你定义一个值可以是若干个不同变体之一。本模块将深入探讨这两种重要的类型系统。

## 学习目标

完成本模块后，你将能够：

- 定义和使用结构体
- 理解结构体字段的访问和修改
- 编写结构体方法（Method）
- 定义和使用枚举
- 使用 Option 和 Result 枚举处理值的存在性和错误
- 理解模式匹配与枚举的结合使用

## 前置知识

在学习本模块之前，你应该掌握：

- [module-01-basics](../module-01-basics/) - 变量、函数基础
- [module-02-ownership](../module-02-ownership/) - 所有权系统

## 核心概念

### 结构体 (Structs)

结构体让你可以创建自定义的数据类型，将相关联的数据组合在一起。

#### 字段结构体

最常用的结构体类型：

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

// 访问字段
println!("Username: {}", user1.username);

// 修改字段（结构体必须是可变的）
let mut user2 = User {
    email: String::from("another@example.com"),
    username: String::from("another"),
    active: true,
    sign_in_count: 1,
};
user2.email = String::from("another@example.com");
```

#### 元组结构体

没有字段名，只有类型：

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0);

// 访问字段使用 .0、.1 索引
println!("Point x: {}", origin.0);
```

#### 单元结构体

没有任何字段：

```rust
struct AlwaysEqual;

let subject = AlwaysEqual;
```

### 结构体方法

方法是与结构体关联的函数：

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 方法（有 &self）
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 关联函数（没有 self）
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

let rect = Rectangle { width: 30, height: 50 };
println!("Area: {}", rect.area());

let square = Rectangle::square(20);
```

### 枚举 (Enums)

枚举允许你定义一个类型，其值可以是几个变体之一：

```rust
enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

#### 带数据的枚举变体

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
```

更复杂的变体：

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

#### 枚举方法

```rust
impl Message {
    fn call(&self) {
        // 方法体
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

### Option 枚举

Option 是 Rust 中最常用的枚举之一，用于表示值可能存在或不存在：

```rust
enum Option<T> {
    None,
    Some(T),
}

// 使用示例
let some_number: Option<i32> = Some(5);
let absent_number: Option<i32> = None;
```

**注意**：Option 被 prelude 包含，不需要显式导入。

### Result 枚举

Result 用于可能返回错误的操作：

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// 使用示例
use std::fs::File;

let f: Result<File, std::io::Error> = File::open("hello.txt");
```

### match 表达式

match 是一个非常强大的控制流运算符：

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

#### 匹配 Option

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

#### 匹配具名变量

```rust
match x {
    Some(50) => println!("Got 50"),
    Some(y) => println!("Got {}", y),
    None => println!("Got nothing"),
}
```

#### 匹配范围

```rust
let x = 5;

match x {
    1..=5 => println!("one through five"),
    _ => println!("something else"),
}
```

### if let 简化匹配

```rust
let some_value = Some(7);

// 完整的 match
match some_value {
    Some(i) => println!("Got {}", i),
    None => println!("Got nothing"),
}

// 简化的 if let
if let Some(i) = some_value {
    println!("Got {}", i);
}
```

## 代码示例

本模块的代码示例位于 `examples/` 目录：

- `structs.rs` - 结构体定义和使用演示
- `enums.rs` - 枚举定义和匹配演示
- `methods.rs` - 方法语法演示
- `option.rs` - Option 类型演示
- `result_enum.rs` - Result 类型演示

运行示例：

```bash
cargo run -p module-03-structs-enums --example structs
```

## 练习题

查看 [exercises.md](exercises.md) 完成练习题。

## 综合练习

查看 [综合练习.md](综合练习.md) 进行综合实践。

## 自检清单

完成学习后，使用 [自检清单.md](自检清单.md) 检查掌握情况。

## 常见问题

### Q: 结构体和元组有什么区别？

A: 结构体有命名的字段，而元组通过索引访问。结构体提供更好的可读性和类型安全。

### Q: 什么时候使用枚举而不是结构体？

A: 当一个值只能是几个特定变体之一时使用枚举（如 IP 地址的 V4 或 V6）。当需要组合多个相关数据时使用结构体。

### Q: Option 为什么能帮助避免空指针错误？

A: Option<T> 强制你处理 None 的情况，编译器会确保你检查了值是否存在，从而避免空指针解引用。

### Q: 为什么 Rust 需要模式匹配是穷尽的？

A: 这确保了所有可能的情况都被处理，防止运行时因未预料的情况而崩溃。

### Q: 方法和函数有什么区别？

A: 方法是在结构体/枚举的上下文中定义的，第一个参数总是 `self`。函数是独立的，不属于任何类型。

## 下一步

完成本模块后，继续学习：

- [module-04-lifetimes](../module-04-lifetimes/) - 理解生命周期，深入掌握所有权系统
