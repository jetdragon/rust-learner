# 08-Trait 与泛型

## 学习目标

- 理解 Trait 的定义和用途
- 掌握泛型编程基础
- 学会使用 Trait Bound
- 掌握 Trait 高级特性（关联类型、默认参数等）

## 核心概念

### 1. Trait (特质)

Trait 定义了共享的行为。类似于其他语言中的接口，但更强大。

```rust
trait Summary {
    fn summarize(&self) -> String;

    fn default_summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

### 2. 实现 Trait

```rust
struct Article {
    title: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.title, self.content)
    }
}
```

### 3. 泛型 (Generics)

泛型允许编写适用于多种类型的代码。

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

### 4. Trait Bound

```rust
// 语法糖形式
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// where 子句形式 (更清晰)
fn some_function<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // ...
}
```

## 标准库常用 Trait

| Trait | 说明 | 方法 |
|-------|------|------|
| `Display` | 格式化输出 | `to_string()` |
| `Debug` | 调试输出 | `{:?}` |
| `Clone` | 显式克隆 | `.clone()` |
| `Copy` | 隐式复制 | 自动实现 |
| `PartialEq` | 部分相等 | `==`, `!=` |
| `Eq` | 完全相等 | 继承 PartialEq |
| `PartialOrd` | 部分比较 | `<`, `>`, `<=`, `>=` |
| `Ord` | 完全比较 | 继承 PartialOrd + Eq |
| `Hash` | 可哈希 | 用于 HashMap/HashSet |
| `Iterator` | 迭代器 | `next()`, `map()`, `filter()` |
| `Into` | 转换所有权 | `into()` |
| `From` | 从类型创建 | `from()` |
| `Default` | 默认值 | `default()` |
| `Drop` | 析构函数 | `drop()` |

## Trait 的高级特性

### 1. 关联类型

```rust
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

### 2. 默认泛型参数

```rust
trait TraitExample<T = i32> {
    fn method(&self, x: T) -> T;
}
```

### 3. 返回实现了 Trait 的类型

```rust
fn returns_summarizable() -> impl Summary {
    Article {
        title: String::from("Headline"),
        content: String::from("Content"),
    }
}
```

### 4. Trait Bound 条件

```rust
use std::fmt::Display;

fn with_limits<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Display + Clone,
{
    // T 和 U 都实现了 Display 和 Clone
}
```

## 实践建议

1. **优先使用 Trait 而非具体类型**: 提高代码灵活性
2. **合理使用泛型**: 避免过度抽象
3. **利用标准库 Trait**: 为类型实现常用 Trait
4. **Trait Objects vs 泛型**: 理解两者的使用场景和性能差异

## 相关资源

- [Rust Book - Trait Bound](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [Rust Book - Advanced Traits](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html)
- [API 文档 - std::marker](https://doc.rust-lang.org/std/marker/index.html)
