# 集合类型 (Collections)

## 模块简介

Rust 标准库提供了多种强大的集合类型，用于存储和操作多个值。本模块将深入探讨 Vec、HashMap、HashSet 等常用集合类型的使用方法和最佳实践。

## 学习目标

完成本模块后，你将能够：

- 掌握 Vec 的创建、操作和迭代
- 理解字符串集合 (String 和 &str)
- 掌握 HashMap 的使用和常见操作
- 了解 HashSet 的用途和特性
- 理解迭代器的使用
- 掌握集合的性能特性

## 前置知识

在学习本模块之前，你应该掌握：

- [module-01-basics](../module-01-basics/) - 变量、函数基础
- [module-02-ownership](../module-02-ownership/) - 所有权和借用
- [module-03-structs-enums](../module-03-structs-enums/) - 结构体和枚举

## 核心概念

### Vec (动态数组)

Vec 是最常用的集合类型，提供动态大小的数组：

```rust
// 创建 Vec
let mut vec = Vec::new();
vec.push(1);

// 使用宏创建
let vec = vec![1, 2, 3];

// 访问元素
let first = vec[0];
let maybe_first = vec.get(0);  // 返回 Option<&T>

// 迭代
for item in &vec {
    println!("{}", item);
}
```

### HashMap

HashMap 存储键值对，提供 O(1) 平均查找：

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Alice"), 10);
scores.insert(String::from("Bob"), 20);

// 访问值
let alice_score = scores.get("Alice");
```

### HashSet

HashSet 存储唯一值，提供 O(1) 查找和插入：

```rust
use std::collections::HashSet;

let mut set = HashSet::new();
set.insert(1);
set.insert(2);
set.insert(1);  // 重复值会被忽略
```

### 字符串集合

```rust
// Vec<String>
let strings = vec![
    String::from("hello"),
    String::from("world"),
];

// Vec<&str>
let string_slices: Vec<&str> = vec!["hello", "world"];
```

## 代码示例

本模块的代码示例位于 `examples/` 目录：

- `vec_basics.rs` - Vec 基础演示
- `hashmap.rs` - HashMap 演示
- `hashset.rs` - HashSet 演示
- `iterators.rs` - 迭代器演示

运行示例：

```bash
cargo run -p module-06-collections --example vec_basics
```

## 练习题

查看 [exercises.md](exercises.md) 完成练习题。

## 综合练习

查看 [综合练习.md](综合练习.md) 进行综合实践。

## 自检清单

完成学习后，使用 [自检清单.md](自检清单.md) 检查掌握情况。

## 常见问题

### Q: Vec vs 数组

A: Vec 是动态大小，可以增长；数组大小固定。优先使用 Vec。

### Q: HashMap vs BTreeMap

A: HashMap 无序，O(1) 平均；BTreeMap 有序，O(log n)。大多数情况用 HashMap。

### Q: 什么时候使用 HashSet 而不是 Vec？

A: 当需要快速查找、去重、或集合成员测试时使用 HashSet。

### Q: 如何选择合适的集合类型？

A:
- 需要索引访问 → Vec
- 需要键值查找 → HashMap
- 需要唯一值和快速查找 → HashSet
- 需要保持顺序 → Vec 或 BTreeMap/TreeSet

### Q: String vs &str

A: String 拥有数据，可变；&str 是借用，不可变。函数参数常用 &str。

## 性能考虑

### Vec

- 尾部添加元素：O(1) 分摊
- 中间插入/删除：O(n)
- 随机访问：O(1)

### HashMap

- 插入：O(1) 分摊
- 查找：O(1)
- 删除：O(1)
- 迭代：O(capacity)

### HashSet

- 插入：O(1) 分摊
- 包含检查：O(1)
- 删除：O(1)

## 最佳实践

1. **使用宏创建已知数据**：`vec![1, 2, 3]`
2. **使用 with_capacity 预分配**：`Vec::with_capacity(100)`
3. **优先使用借用迭代器**：`&vec` 而不是 `vec.iter()`
4. **使用 get 处理越界**：返回 Option
5. **使用 Entry API 处理 HashMap**

## 下一步

完成本模块后，继续学习：

- [module-07-traits-generics](../module-07-traits-generics/) - Trait 与泛型
- [module-08-lifetimes](../module-08-lifetimes/) - 生命周期
