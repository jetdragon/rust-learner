# Module 02: 所有权与借用 - 练习

欢迎来到模块 02 的练习部分！本模块聚焦于 Rust 的核心概念：所有权（Ownership）和借用（Borrowing）。

## 📚 练习列表

| 练习 | 主题 | 难度 | 预计时间 |
|------|------|------|----------|
| [Exercise 1](exercise1_ownership_transfer.rs) | 所有权转移 | 入门 | 10 分钟 |
| [Exercise 2](exercise2_function_ownership.rs) | 函数与所有权 | 入门 | 15 分钟 |
| [Exercise 3](exercise3_references.rs) | 引用与借用 | 入门 | 15 分钟 |
| [Exercise 4](exercise4_mutable_reference.rs) | 可变引用 | 中等 | 15 分钟 |
| [Exercise 5](exercise5_borrow_rules.rs) | 借用规则挑战 | 困难 | 20 分钟 |
| [Exercise 6](exercise6_string_slice.rs) | 字符串切片 | 中等 | 20 分钟 |

## 🚀 如何开始

### 运行单个练习

```bash
# 进入练习目录
cd module-02-ownership/exercises

# 运行特定练习
cargo run --bin exercise1_ownership_transfer
cargo run --bin exercise4_mutable_reference
```

### 运行测试

```bash
# 运行所有练习的测试
cargo test --package module-02-ownership

# 运行特定练习的测试
cargo test --package module-02-ownership exercise1_ownership_transfer
cargo test --package module-02-ownership exercise4_mutable_reference

# 显示测试输出
cargo test --package module-02-ownership -- --nocapture
```

### 查看解答

如果你遇到困难，可以参考解答文件：

```bash
# 查看特定练习的解答
cat solutions/exercise1_ownership_transfer.rs
cat solutions/exercise4_mutable_reference.rs
```

## 💡 学习建议

### 练习顺序

建议按顺序完成练习，每个练习都建立在前一个的基础上：

1. **Exercise 1**: 理解基本的所有权转移
2. **Exercise 2**: 学习函数如何影响所有权
3. **Exercise 3**: 掌握引用和借用基础
4. **Exercise 4**: 深入可变引用
5. **Exercise 5**: 综合运用借用规则
6. **Exercise 6**: 学习字符串切片

### 常见问题

**Q: 为什么我的代码不能编译？**
A: Rust 编译器的错误信息非常详细。仔细阅读错误信息，它通常会告诉你违反了什么规则。

**Q: 我应该什么时候使用引用？**
A: 当你只需要读取数据而不需要获取所有权时，使用引用。这是更高效的方式。

**Q: 什么时候使用可变引用？**
A: 当你需要修改借用数据时。记住，同一时间只能有一个可变引用。

**Q: 什么是字符串切片？**
A: 字符串切片（`&str`）是对字符串部分的引用。它不获取所有权，只借用部分数据。

## 🎯 练习提示

### Exercise 1: 所有权转移

- 关注 `String` 类型的移动语义
- 对比整型的 `Copy` 行为
- 理解为什么被移动后的变量不能再使用

### Exercise 2: 函数与所有权

- 传递参数时的所有权转移
- 使用返回值恢复所有权
- 理解引用的优势

### Exercise 3: 引用与借用

- 使用 `&` 创建引用
- 理解借用不获取所有权
- 掌握基本的引用操作

### Exercise 4: 可变引用

- 使用 `&mut` 创建可变引用
- 理解可变引用的唯一性限制
- 学会通过引用修改数据

### Exercise 5: 借用规则

- 理解借用规则的重要性
- 学会使用作用域解决借用冲突
- 掌握何时使用可变/不可变引用

### Exercise 6: 字符串切片

- 理解 `&str` 类型
- 使用切片语法 `[start..end]`
- 学会使用字符串字面量

## 📖 核心概念

### 所有权规则

1. Rust 中每个值都有一个所有者
2. 值在同一时间只能有一个所有者
3. 当所有者离开作用域，值被丢弃

### 借用规则

1. 同一时间，要么有一个可变引用，要么有多个不可变引用
2. 引用必须始终有效

### 引用类型

- `&T`: 不可变引用（共享引用）
- `&mut T`: 可变引用（独占引用）
- `&str`: 字符串切片

## 🔧 实用命令

```bash
# 检查代码（不运行）
cargo check --package module-02-ownership

# 格式化代码
cargo fmt --package module-02-ownership

# 运行 clippy（代码检查工具）
cargo clippy --package module-02-ownership
```

## 📚 进阶学习

完成这些练习后，你可以：

- 阅读 [The Rust Book](https://doc.rust-lang.org/book/) 第 4 章
- 探索智能指针（Box, Rc, Arc）
- 学习生命周期注解
- 理解借用检查器的工作原理

## 🤝 获取帮助

如果遇到问题：

1. 仔细阅读编译器错误信息
2. 查看解答文件中的实现思路
3. 参考 [Rust 官方文档](https://doc.rust-lang.org/std/)
4. 在 [Rust 社区](https://www.rust-lang.org/community) 寻求帮助

祝学习愉快！🦀
