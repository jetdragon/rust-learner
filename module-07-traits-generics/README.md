# 07-Trait 与泛型

## 学习目标

- 理解 Trait 的定义和用途
- 掌握泛型编程基础
- 学会使用 Trait Bound
- 了解生命周期参数

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

### 5. 生命周期

生命周期确保引用有效。

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
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

## 生命周期规则

1. **省略规则**:
   - 每个引用参数都有自己的生命周期
   - 如果只有一个输入生命周期，赋给所有输出生命周期
   - 如果有多个输入生命周期，但其中一个是 `&self` 或 `&mut self`，赋给 `self` 的生命周期

2. **静态生命周期**:
   ```rust
   let s: &'static str = "I have a static lifetime.";
   ```

3. **结构体中的生命周期**:
   ```rust
   struct ImportantExcerpt<'a> {
       part: &'a str,
   }
   ```

## 实践建议

1. **优先使用 Trait 而非具体类型**: 提高代码灵活性
2. **合理使用泛型**: 避免过度抽象
3. **理解生命周期规则**: 大多数情况编译器能推断
4. **利用标准库 Trait**: 为类型实现常用 Trait

## 相关资源

- [Rust Book - Trait Bound](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [Rust Book - 生命周期](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
- [API 文档 - std::marker](https://doc.rust-lang.org/std/marker/index.html)
