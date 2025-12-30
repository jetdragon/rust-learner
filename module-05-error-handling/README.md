# 错误处理 (Error Handling)

## 模块简介

Rust 的错误处理机制是其最强大的特性之一。本模块将深入探讨 Rust 中的错误处理模式，包括 Result 类型、Option 类型、错误传播、自定义错误类型以及最佳实践。

## 学习目标

完成本模块后，你将能够：

- 理解 Rust 错误处理的设计哲学
- 掌握 Result 和 Option 类型的使用
- 使用 `?` 操作符简化错误传播
- 创建自定义错误类型
- 使用 thiserror 和 anyhow 库
- 理解 panic! 和 recoverable 错误的区别
- 掌握错误处理的最佳实践

## 前置知识

在学习本模块之前，你应该掌握：

- [module-01-basics](../module-01-basics/) - 变量、函数基础
- [module-02-ownership](../module-02-ownership/) - 所有权系统
- [module-03-structs-enums](../module-03-structs-enums/) - 结构体和枚举
- [module-04-patterns](../module-04-patterns/) - 模式匹配

## 核心概念

### 错误处理哲学

Rust 将错误分为两类：

1. **可恢复错误 (Recoverable)**：使用 `Result<T, E>` 类型
2. **不可恢复错误 (Unrecoverable)**：使用 `panic!` 宏

```rust
// 可恢复错误
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("除数不能为零".to_string())
    } else {
        Ok(a / b)
    }
}

// 不可恢复错误
fn critical_error() {
    panic!("发生严重错误，程序无法继续");
}
```

### Result 类型

`Result<T, E>` 是一个枚举，有两个变体：

```rust
enum Result<T, E> {
    Ok(T),    // 成功，包含值 T
    Err(E),   // 失败，包含错误 E
}
```

### 处理 Result

#### 1. 使用 match

```rust
match divide(10.0, 2.0) {
    Ok(result) => println!("结果: {}", result),
    Err(e) => println!("错误: {}", e),
}
```

#### 2. 使用 unwrap 和 expect

```rust
// unwrap: 成功时返回值，失败时 panic
let result = divide(10.0, 2.0).unwrap();

// expect: 类似 unwrap，但可以指定错误消息
let result = divide(10.0, 0.0).expect("除法失败");
```

#### 3. 使用 ? 操作符

```rust
fn parse_and_divide(a_str: &str, b_str: &str) -> Result<f64, ParseIntError> {
    let a: f64 = a_str.parse()?;  // 如果失败，提前返回 Err
    let b: f64 = b_str.parse()?;
    divide(a, b)
}
```

### Option 类型

`Option<T>` 表示值可能存在或不存在：

```rust
enum Option<T> {
    Some(T),
    None,
}
```

### 错误传播

#### 传播错误的几种方式

```rust
// 方式 1: 显式 match
fn method1() -> Result<(), Error> {
    match risky_operation() {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

// 方式 2: 使用 ? (推荐)
fn method2() -> Result<(), Error> {
    risky_operation()?;
    Ok(())
}
```

### 自定义错误类型

```rust
#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    Custom(String),
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}
```

### 使用 thiserror

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("解析错误: {0}")]
    Parse(#[from] std::num::ParseIntError),

    #[error("自定义错误: {0}")]
    Custom(String),
}
```

### 错误转换

```rust
// 使用 ? 自动转换错误
fn read_file() -> Result<String, MyError> {
    let content = std::fs::read_to_string("file.txt")?;  // std::io::Error -> MyError
    Ok(content)
}
```

### panic! 和不可恢复错误

```rust
// 显式 panic
panic!("发生严重错误");

// assert! 失败时也会 panic
assert!(x > 0, "x 必须大于 0");

// unwrap 和 expect 失败时 panic
let value = Some(5).unwrap();
let value = None.expect("值不应该为 None");
```

## 代码示例

本模块的代码示例位于 `examples/` 目录：

- `result_basics.rs` - Result 基础演示
- `error_propagation.rs` - 错误传播演示
- `custom_errors.rs` - 自定义错误类型
- `panic.rs` - panic 演示

运行示例：

```bash
cargo run -p module-05-error-handling --example result_basics
```

## 练习题

查看 [exercises.md](exercises.md) 完成练习题。

## 综合练习

查看 [综合练习.md](综合练习.md) 进行综合实践。

## 自检清单

完成学习后，使用 [自检清单.md](自检清单.md) 检查掌握情况。

## 常见问题

### Q: 什么时候使用 Result，什么时候使用 Option？

A: 使用 `Result` 当错误需要信息，使用 `Option` 当错误只是一个缺失值。例如：
- `parse::<usize>()` 返回 `Result`，因为解析失败需要知道原因
- `get()` 返回 `Option`，因为索引越界只是"没有这个元素"

### Q: 什么时候应该 panic!？

A: 只有在以下情况使用 panic!：
- 示例代码中
- 测试中
- 当错误不应该发生时（不可能的情况）
- 当你的代码无法安全恢复时

### Q: ? 操作符有什么限制？

A: `?` 操作符只能在返回 `Result` 或 `Option` 的函数中使用。它会：
- 成功时解包值
- 失败时提前返回错误

### Q: 如何处理多种错误类型？

A: 有几种方式：
1. 创建自定义错误枚举，为每种错误实现 `From`
2. 使用 `Box<dyn Error + Send + Sync>` 作为错误类型
3. 使用 `anyhow` 库简化错误处理

### Q: unwrap 和 expect 哪个更好？

A: `expect` 更好，因为它允许你提供错误消息，便于调试。只在测试或示例中使用 `unwrap`。

## 最佳实践

1. **优先使用 Result 而不是 panic**
   - 让调用者决定如何处理错误
   - 只在真正无法恢复时 panic

2. **使用 ? 而不是显式 match**
   - 代码更简洁
   - 错误传播更清晰

3. **自定义错误类型**
   - 为公共 API 创建有意义的错误类型
   - 实现 `Error` trait 和 `Display`

4. **提供上下文信息**
   - 使用 `map_err()` 添加错误上下文
   - 或者使用 `anyhow` 的 `.context()`

5. **考虑使用 thiserror/anyhow**
   - `thiserror`：用于定义库的错误类型
   - `anyhow`：用于应用程序的错误处理

## 下一步

完成本模块后，继续学习：

- [module-06-collections](../module-06-collections/) - 集合类型
- [module-07-traits](../module-07-traits/) - 泛型和 Trait
