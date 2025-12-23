# Rust 学习路径

一个为中文学习者设计的循序渐进 Rust 学习项目。

## 项目简介

本项目通过 10 个精心设计的模块，从基础到实战，帮助你系统掌握 Rust 编程语言。每个模块包含：

- 📖 详细的中文概念讲解
- 💻 可运行的代码示例
- ✏️ 练习题及参考答案
- ✔️ 学习效果验证机制

## 学习路径

```
module-01-basics    → 变量、数据类型、函数
module-02-ownership  → Rust 核心概念
module-03-structs-enums → 自定义数据类型
module-04-patterns   → 强大的控制流工具
module-05-error-handling → Result 和 Option
module-06-collections → Vec、HashMap 等
module-07-generics   → 代码抽象
module-08-lifetimes  → 引用的有效性
module-09-concurrency → 线程与消息传递
module-10-project    → 综合应用
```

## 快速开始

### 前置要求

- 安装 [Rust](https://www.rust-lang.org/tools/install) (1.75 或更高版本)
- 熟悉基本的命令行操作

### 构建项目

```bash
# 构建所有模块
cargo build

# 构建特定模块
cargo build -p module-01-basics
```

### 运行示例

```bash
# 运行某个示例
cargo run -p module-01-basics --bin variables
```

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定模块的测试
cargo test -p module-01-basics
```

## 学习进度

查看 [进度.md](进度.md) 追踪你的学习进展。

## 贡献指南

欢迎贡献新的学习内容！请查看 [CONTRIBUTING.md](CONTRIBUTING.md) 了解详情。

## 许可证

MIT License
