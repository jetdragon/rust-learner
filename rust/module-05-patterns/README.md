# 模式匹配 (Pattern Matching)

## 模块简介

模式匹配是 Rust 中强大且常用的控制流工具。它让你可以基于值的不同形态执行不同的代码。本模块将深入探讨模式匹配的各种用法和技巧。

## 学习目标

完成本模块后，你将能够：

- 理解模式匹配的基本语法
- 掌握各种模式类型（字面值、变量、通配符等）
- 理解 match 表达式的穷尽性要求
- 使用 if let 和 while let 简化代码
- 掌握解构结构体和枚举
- 理解模式匹配中的高级特性

## 前置知识

在学习本模块之前，你应该掌握：

- [module-01-basics](../module-01-basics/) - 变量、函数基础
- [module-02-ownership](../module-02-ownership/) - 所有权系统
- [module-03-structs-enums](../module-03-structs-enums/) - 结构体和枚举

## 核心概念

### match 表达式基础

match 表达式由模式和代码组成：

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

**关键特性**：
- match 必须穷尽所有可能
- 每个分支返回相同类型的值
- 模式按顺序匹配，第一个匹配的分支会被执行

### 模式类型

#### 1. 字面值模式

```rust
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

#### 2. 变量模式

```rust
let x = 5;

match x {
    n => println!("x is {}", n),
}
```

#### 3. 通配符模式

```rust
let x = 3;

match x {
    1 => println!("one"),
    2 => println!("two"),
    _ => println!("anything else"),
}
```

#### 4. 多个模式

```rust
let x = 1;

match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

#### 5. 范围模式

```rust
let x = 5;

match x {
    1..=5 => println!("one through five"),
    _ => println!("something else"),
}
```

### 匹配 Option

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
```

### 匹配枚举

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn process_message(msg: &Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to {}, {}", x, y),
        Message::Write(text) => println!("Text: {}", text),
        Message::ChangeColor(r, g, b) => println!("Color: {}, {}, {}", r, g, b),
    }
}
```

### 解构结构体

```rust
struct Point {
    x: i32,
    y: i32,
}

fn print_coordinates(p: &Point) {
    match p {
        Point { x: 0, y: 0 } => println!("Origin"),
        Point { x, y: 0 } => println!("On x-axis at {}", x),
        Point { x: 0, y } => println!("On y-axis at {}", y),
        Point { x, y } => println!("At ({}, {})", x, y),
    }
}
```

### 解构枚举

```rust
enum Color {
    RGB(i32, i32, i32),
    HSV(i32, i32, i32),
}

fn describe_color(color: &Color) {
    match color {
        Color::RGB(0, 0, 0) => println!("Black"),
        Color::RGB(r, g, b) => println!("RGB({}, {}, {})", r, g, b),
        Color::HSV(h, s, v) => println!("HSV({}, {}, {})", h, s, v),
    }
}
```

### 忽略值

使用 `_` 忽略不需要的值：

```rust
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, _, third, _, fifth) => {
        println!("Some numbers: {}, {}, {}", first, third, fifth)
    }
}
```

使用 `..` 忽略剩余部分：

```rust
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
    Point { x, .. } => println!("x is {}", x),
}
```

### 守卫

在模式上添加额外的条件：

```rust
let num = Some(4);

match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => println!("none"),
}
```

### @ 绑定

使用 `@` 同时绑定值和测试：

```rust
enum Message {
    Hello { id: i32 },
}

fn msg_id(msg: &Message) -> i32 {
    match msg {
        Message::Hello { id: id @ 3..=7 } => {
            println!("Found id in range: {}", id);
            id
        }
        Message::Hello { id: new_id @ 10 | 12 } => {
            println!("Found id 10 or 12: {}", new_id);
            new_id
        }
        Message::Hello { id } => id,
    }
}
```

### if let 简化

当只关心一个模式时，使用 if let：

```rust
let some_value = Some(7);

// 完整的 match
match some_value {
    Some(i) => println!("value is {}", i),
    None => println!("no value"),
}

// 简化的 if let
if let Some(i) = some_value {
    println!("value is {}", i);
}
```

### while let 循环

只要模式匹配就继续循环：

```rust
let mut optional = Some(0);

while let Some(i) = optional {
    if i < 5 {
        optional = Some(i + 1);
    } else {
        optional = None;
    }
}
```

## 代码示例

本模块的代码示例位于 `examples/` 目录：

- `match_basics.rs` - match 基础演示
- `patterns.rs` - 各种模式演示
- `if_let.rs` - if let 和 while let 演示
- `destructuring.rs` - 解构演示

运行示例：

```bash
cargo run -p module-04-patterns --example match_basics
```

## 练习题

查看 [exercises.md](exercises.md) 完成练习题。

## 综合练习

查看 [综合练习.md](综合练习.md) 进行综合实践。

## 自检清单

完成学习后，使用 [自检清单.md](自检清单.md) 检查掌握情况。

## 常见问题

### Q: 为什么 match 必须穷尽？

A: Rust 的设计哲学是确保安全。穷尽性要求确保所有可能的情况都被处理，防止运行时因未预料的情况而崩溃。

### Q: _ 通配符和变量模式有什么区别？

A: 变量模式会绑定值并可以使用，通配符 `_` 会忽略值且不能使用。

### Q: 什么时候使用 if let 而不是 match？

A: 当只关心一个模式，其他情况不需要处理时使用 if let。如果需要处理多种情况，使用 match。

### Q: 模式匹配的顺序重要吗？

A: 是的，模式按顺序匹配。更具体的模式应该放在更通用的模式之前。

### Q: 如何在模式中使用范围？

A: 使用 `..=` 包含结束值，使用 `..` 不包含结束值。例如 `1..=5` 匹配 1 到 5，`1..5` 只匹配 1 到 4。

## 下一步

完成本模块后，继续学习：

- [module-06-error-handling](../module-06-error-handling/) - 错误处理与 Result 类型
