# Proposal: 添加练习文件模板和解答

## 问题分析

当前项目中，每个学习模块下存在结构不一致的问题：

1. **exercises/ 文件夹为空**：根据 `module-structure` spec，`exercises/` 应该包含练习题模板文件（如 `exercise1.rs`）和 `solutions/` 子目录，但目前所有模块的 `exercises/` 和 `solutions/` 都是空的。

2. **exercises.md 文件位置**：练习题描述在模块根目录的 `exercises.md` 文件中，而不是与练习文件关联。

3. **缺失可运行的练习代码**：学习者无法直接在 `exercises/` 目录下找到可编辑的练习模板和可对照的解答代码。

这种不一致导致：
- 学习者不知道在哪里编写练习代码
- 无法提供"练习模板"让学习者填空
- 无法提供独立的解答文件供参考

## 提议变更

为每个模块添加：
1. **练习模板文件** (`exercises/exercise_N.rs`)：包含 TODO 注释的骨架代码，让学习者填写实现
2. **解答文件** (`exercises/solutions/exercise_N.rs`)：完整的参考解答
3. **验证测试**：在模块的 `tests/` 目录下添加对应的集成测试

## 涉及模块

所有 10 个学习模块都需要补充练习文件：
- module-01-basics
- module-02-ownership
- module-03-structs-enums
- module-04-patterns
- module-05-error-handling
- module-06-collections
- module-07-generics
- module-08-lifetimes
- module-09-concurrency
- module-10-project

## 设计考虑

### 练习文件格式

每个练习文件应包含：
```rust
//! 练习 1: [练习名称]
//!
//! TODO: 在此实现 [功能描述]

pub fn function_name() -> ReturnType {
    // TODO: 实现这个函数
}
```

### 解答文件格式

解答文件包含完整实现，学习者可以参考但不能直接复制（通过文件名和注释提示）。

### 与现有内容的关联

- `exercises.md` 中描述的练习题需要在 `exercises/` 目录下有对应的文件
- 练习编号应与 `exercises.md` 中的题目对应

## 实施策略

1. **保持向后兼容**：不删除现有的 `exercises.md`，它继续作为练习题的详细描述
2. **渐进式添加**：从 module-01 开始，逐个模块添加练习文件
3. **自动化验证**：为每个练习添加测试，确保解答正确

## 依赖关系

- 依赖于 `complete-module-content` 变更中的模块结构
- 与 TUI 学习伴侣的"练习"功能集成
