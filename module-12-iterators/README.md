# 12-迭代器 (Iterators)

## 模块简介

迭代器是 Rust 中处理序列的强大工具，提供了零成本抽象。迭代器模式允许你逐步处理序列中的元素，同时保持代码的简洁和高效。本模块将深入探讨迭代器 trait、消费者适配器、迭代器适配器以及自定义迭代器。

## 学习目标

完成本模块后，你将能够：

- 理解 Iterator trait 和相关概念
- 掌握消费者适配器（sum、collect、fold 等）
- 掌握迭代器适配器（map、filter、zip 等）
- 学会创建自定义迭代器
- 理解迭代器的性能优势
- 掌握迭代器与 for 循环的对比

## 前置知识

在学习本模块之前，你应该掌握：

- [module-01-basics](../module-01-basics/) - 变量、函数基础
- [module-03-structs-enums](../module-03-structs-enums/) - 结构体和枚举
- [module-08-traits-generics](../module-08-traits-generics/) - Trait 和泛型

## 核心概念

### Iterator trait

Iterator trait 定义了序列迭代的接口：

```rust
pub trait Iterator {
    type Item;  // 关联类型

    fn next(&mut self) -> Option<Self::Item>;

    // 提供默认实现的方法
    fn map<B, F>(self, f: F) -> Map<Self, F>
    where
        F: FnMut(Self::Item) -> B,
    { ... }

    fn filter<P>(self, predicate: P) -> Filter<Self, P>
    where
        P: FnMut(&Self::Item) -> bool,
    { ... }

    // ... 更多方法
}
```

**关键概念**：
- `Item`：迭代器产生值的类型（关联类型）
- `next()`：返回下一个值，返回 `Option<Item>`
- **惰性求值**：只有在需要时才计算值

### 消费者适配器

消费者适配器会**消耗**迭代器，返回最终结果：

#### 1. `collect()`

将迭代器收集到集合中：

```rust
let v = vec
![1, 2, 3, 4, 5];

// 收集到 Vec
let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();

// 收集到 HashMap
use std::collections::HashMap;
let map: HashMap<_, _> = v.iter().enumerate().collect();
```

#### 2. `sum()`, `product()`

计算总和和乘积：

```rust
let v = vec
![1, 2, 3, 4, 5];
let sum: i32 = v.iter().sum();
let product: i32 = v.iter().product();
```

#### 3. `min()`, `max()`, `min_by()`, `max_by()`

找到最小值或最大值：

```rust
let v = vec
![3, 1, 4, 1, 5];
let min = v.iter().min();
let max = v.iter().max();
```

#### 4. `find()`, `position()`, `rposition()`

查找元素或位置：

```rust
let v = vec
![1, 2, 3, 4, 5];

let found = v.iter().find(|&&x| x == 3);  // Some(&3)
let pos = v.iter().position(|&x| x == 3);  // Some(2)
```

#### 5. `any()`, `all()`

检查是否满足条件：

```rust
let v = vec
![1, 2, 3, 4, 5];
let any_even = v.iter().any(|&x| x % 2 == 0);  // true
let all_positive = v.iter().all(|&x| x > 0);    // true
```

#### 6. `fold()`, `reduce()`

累积计算：

```rust
let v = vec
![1, 2, 3, 4, 5];

// fold: 提供初始值
let sum = v.iter().fold(0, |acc, &x| acc + x);  // 15

// reduce: 使用第一个元素作为初始值
let sum = v.iter().reduce(|acc, &x| acc + x);  // Some(15)
```

#### 7. `count()`

计算元素数量：

```rust
let v = vec
![1, 2, 3, 4, 5];
let count = v.iter().count();  // 5
```

#### 8. `last()`, `nth()`

获取特定位置的元素：

```rust
let v = vec
![1, 2, 3, 4, 5];
let last = v.iter().last();    // Some(&5)
let third = v.iter().nth(2);  // Some(&3)
```

### 迭代器适配器

迭代器适配器**不消耗**迭代器，而是返回新的迭代器：

#### 1. `map()`

转换每个元素：

```rust
let v = vec
![1, 2, 3, 4, 5];
let squares: Vec<_> = v.iter().map(|x| x * x).collect();
// [1, 4, 9, 16, 25]
```

#### 2. `filter()`, `filter_map()`

过滤元素：

```rust
let v = vec
![1, 2, 3, 4, 5];
let evens: Vec<_> = v.iter().filter(|&&x| x % 2 == 0).collect();
// [2, 4]

// filter_map: 可以返回 Option
let filtered: Vec<_> = v.iter().filter_map(|&x| {
    if x % 2 == 0 { Some(x * x) } else { None }
}).collect();
// [4, 16]
```

#### 3. `enumerate()`

添加索引：

```rust
let v = vec
!["a", "b", "c"];
for (i, item) in v.iter().enumerate() {
    println!("{}: {}", i, item);
}
// 0: a
// 1: b
// 2: c
```

#### 4. `zip()`, `zip_with()`

组合两个迭代器：

```rust
let v1 = vec
![1, 2, 3];
let v2 = vec
!["a", "b", "c"];
let pairs: Vec<_> = v1.iter().zip(v2.iter()).collect();
// [(&1, "a"), (&2, "b"), (&3, "c")]
```

#### 5. `chain()`

连接两个迭代器：

```rust
let v1 = vec
![1, 2, 3];
let v2 = vec
![4, 5, 6];
let chained: Vec<_> = v1.iter().chain(v2.iter()).collect();
// [1, 2, 3, 4, 5, 6]
```

#### 6. `take()`, `skip()`, `take_while()`, `skip_while()`

跳过或取元素：

```rust
let v = vec
![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

// take: 取前 N 个
let taken: Vec<_> = v.iter().take(5).collect();
// [1, 2, 3, 4, 5]

// skip: 跳过前 N 个
let skipped: Vec<_> = v.iter().skip(5).collect();
// [6, 7, 8, 9, 10]

// take_while: 取满足条件的元素
let taken_while: Vec<_> = v.iter().take_while(|&&x| x <= 5).collect();
// [1, 2, 3, 4, 5]

// skip_while: 跳过满足条件的元素
let skipped_while: Vec<_> = v.iter().skip_while(|&&x| x <= 5).collect();
// [6, 7, 8, 9, 10]
```

#### 7. `flatten()`, `flat_map()`

扁平化嵌套结构：

```rust
let nested = vec
![vec
![1, 2], vec
![3, 4]];
let flattened: Vec<_> = nested.iter().flatten().cloned().collect();
// [1, 2, 3, 4]

// flat_map: 先 map 后 flatten
let flat_mapped: Vec<_> = (0..3).flat_map(|i| 0..=i).collect();
// [0, 0, 1, 0, 1, 2]
```

#### 8. `rev()`

反转迭代器：

```rust
let v = vec
![1, 2, 3, 4, 5];
let reversed: Vec<_> = v.iter().rev().cloned().collect();
// [5, 4, 3, 2, 1]
```

#### 9. `cloned()`, `copied()`

复制引用中的值：

```rust
let v = vec
![1, 2, 3, 4, 5];

// cloned: 实现 Clone 的类型
let cloned: Vec<_> = v.iter().cloned().collect();

// copied: 实现 Copy 的类型
let copied: Vec<_> = v.iter().copied().collect();
```

### 自定义迭代器

实现 Iterator trait 创建自定义迭代器：

```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// 使用自定义迭代器
let mut counter = Counter::new();
while let Some(val) = counter.next() {
    println!("{}", val);
}

// 可以使用所有适配器
let sum: u32 = Counter::new().sum();  // 1+2+3+4+5 = 15
```

### 迭代器性能

#### 零成本抽象

迭代器在编译时被优化，性能与手写循环相同：

```rust
// 迭代器版本
let sum: i32 = (1..=100).sum();

// 手写循环版本
let mut sum = 0;
for i in 1..=100 {
    sum += i;
}

// 两者性能相同，但迭代器版本更简洁
```

#### 惰性求值优势

迭代器的惰性求值可以避免不必要的计算：

```rust
// 只会生成前 5 个元素
let first_five: Vec<_> = (1..).take(5).collect();
// [1, 2, 3, 4, 5]

// 不会生成所有 100 个元素
let result = (1..100).find(|&x| x * x > 10);
// Some(4)  // 4 * 4 = 16 > 10
```

## 代码示例

本模块的代码示例位于 `examples/` 目录：

- `iterator_basics.rs` - Iterator trait 基础
- `consumer_adapters.rs` - 消费者适配器演示
- `iterator_adapters.rs` - 迭代器适配器演示
- `custom_iterator.rs` - 自定义迭代器演示
- `fibonacci.rs` - 斐波那契数列迭代器
- `performance.rs` - 性能对比演示

## 练习题

查看 [exercises.md](exercises.md) 完成练习题。

## 综合练习

查看 [综合练习.md](综合练习.md) 进行综合实践。

## 自检清单

完成学习后，使用 [自检清单.md](自检清单.md) 检查掌握情况。

## 常见问题

### Q: 迭代器和 for 循环有什么区别？

A:
- **性能相同**：编译器会将 for 循环优化成迭代器
- **迭代器更灵活**：可以链式调用多个操作
- **for 循环更简洁**：简单情况更直观

### Q: `iter()`, `iter_mut()`, `into_iter()` 有什么区别？

A:
- `iter()`：迭代不可变引用 `&T`
- `iter_mut()`：迭代可变引用 `&mut T`
- `into_iter()`：获取所有权，迭代 `T`

### Q: 为什么用 `.cloned()` 或 `.copied()`？

A:
- 迭代器产生引用时，需要转换成拥有的值
- `cloned()`：调用 `clone()` 方法（适用于 `Clone` 类型）
- `copied()`：复制值（仅适用于 `Copy` 类型，更高效）

### Q: 迭代器会占用更多内存吗？

A:
- **不会**：迭代器是惰性的，只在需要时计算
- **节省内存**：`take(5)` 只会生成 5 个元素，而不是全部

### Q: 如何选择合适的迭代器方法？

A:
- **简单遍历**：for 循环
- **需要索引**：`enumerate()`
- **过滤**：`filter()`
- **转换**：`map()`
- **查找**：`find()`, `position()`
- **汇总**：`sum()`, `fold()`
- **收集结果**：`collect()`

## 最佳实践

1. **优先使用迭代器**：代码更简洁、表达力更强
2. **链式调用**：充分利用适配器的组合能力
3. **理解惰性**：避免不必要的计算
4. **选择合适的迭代方法**：`iter()`, `iter_mut()`, `into_iter()`
5. **注意借用规则**：迭代时不要修改原数据
6. **性能无关时**：优先可读性，编译器会优化

## 下一步

完成本模块后，继续学习：

- [module-09-concurrency](../module-09-concurrency/) - 并发编程
- [module-11-smart-pointers](../module-11-smart-pointers/) - 智能指针

## 相关资源

- [Rust Book - Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [Rust Book - Iterator Trait](https://doc.rust-lang.org/book/ch13-04-performance.html)
- [API 文档 - std::iter](https://doc.rust-lang.org/std/iter/)
- [Rust by Example - Iterators](https://doc.rust-lang.org/rust-by-example/std/iter.html)
