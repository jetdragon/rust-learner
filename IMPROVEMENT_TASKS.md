# Rust 学习路径项目 - 改进任务计划

基于专家分析，本项目需要进行以下改进以提供更完整的 Rust 学习体验。

## 任务优先级

### P0 - 关键修改（必须完成）

#### Task 1: 调整模块顺序 - 生命周期提前
**目标**: 将 module-08 (Lifetimes) 移到 module-03 之后

**修改内容**:
- 将 module-08 重命名为 module-04b-lifetimes
- 调整后续模块编号
- 更新所有 README 中的引用链接
- 更新工作区成员列表

**验证**:
- 所有模块的交叉引用链接正确
- cargo build 成功

---

#### Task 2: 移除 module-07 中的生命周期内容
**目标**: 清理 module-07，只保留 Trait 和泛型内容

**修改内容**:
- 从 module-07/README.md 移除生命周期部分（第5节）
- 检查示例文件中的生命周期示例并移除
- 更新学习目标，移除生命周期相关内容

**验证**:
- module-07 只包含 Trait 和泛型
- 代码示例不包含生命周期注解
- README 内容连贯

---

#### Task 3: 扩充 module-06 集合类型
**目标**: 添加缺失的集合类型（BTreeMap、VecDeque）

**修改内容**:
- 在 README.md 中添加 BTreeMap 和 BTreeSet 说明
- 添加 VecDeque 说明和示例
- 更新性能对比表
- 创建新的示例文件：
  - examples/btreemap.rs
  - examples/vecdeque.rs

**验证**:
- 新示例代码可运行
- README 内容完整
- 性能表包含所有主要集合类型

---

#### Task 4: 扩充 module-09 并发编程
**目标**: 添加 async/await 和更多并发原语

**修改内容**:
- 添加 async/await 基础章节
- 添加 RwLock、Condvar 说明
- 添加线程池实现示例
- 添加 Future trait 说明
- 在 Cargo.toml 添加 tokio 依赖
- 创建新示例：
  - examples/async_basics.rs
  - examples/thread_pool.rs
  - examples/rwlock.rs

**验证**:
- async 示例可运行
- 线程池示例正确
- README 内容全面

---

#### Task 5: 新增 module-11 智能指针
**目标**: 创建智能指针模块（Rust 核心概念）

**修改内容**:
- 创建 module-11-smart-pointers 目录
- 创建 Cargo.toml
- 创建 README.md 包含：
  - Box<T> - 堆分配、递归类型、Trait Objects
  - Rc<T> - 引用计数、多所有权
  - Arc<T> - 原子引用计数
  - Cell<T>/RefCell<T> - 内部可变性
- 创建示例文件：
  - examples/box_basics.rs
  - examples/rc_example.rs
  - examples/refcell.rs
  - examples/cons_list.rs
- 创建 exercises.md
- 创建 综合练习.md
- 创建 自检清单.md
- 更新主 README 模块列表
- 更新 workspace Cargo.toml 成员列表

**验证**:
- 新模块构建成功
- 所有示例可运行
- 与其他模块引用正确

---

#### Task 6: 新增 module-12 迭代器
**目标**: 创建迭代器模块（Rust 零成本抽象）

**修改内容**:
- 创建 module-12-iterators 目录
- 创建 Cargo.toml
- 创建 README.md 包含：
  - Iterator trait
  - 消费适配器 (collect, sum, fold, etc.)
  - 迭代器适配器 (map, filter, zip, etc.)
  - 自定义迭代器
  - 性能考虑
- 创建示例文件：
  - examples/iterator_basics.rs
  - examples/consumer_adapters.rs
  - examples/iterator_adapters.rs
  - examples/custom_iterator.rs
  - examples/fibonacci.rs
- 创建 exercises.md
- 创建 综合练习.md
- 创建 自检清单.md
- 更新主 README 模块列表
- 更新 workspace Cargo.toml 成员列表

**验证**:
- 新模块构建成功
- 所有示例可运行
- 迭代器性能对比正确

---

### P1 - 重要修改（建议完成）

#### Task 7: 为 module-05 补充错误处理练习
**目标**: 添加更多错误处理实战练习

**修改内容**:
- 在 exercises.md 添加练习：
  - Result 基础操作
  - 自定义错误类型
  - From trait 实现
  - 错误链 (anyhow::Context)
- 更新示例文件（如果需要）

**验证**:
- 新练习题有意义
- 参考答案正确

---

#### Task 8: 为 module-07 补充 Trait 高级特性练习
**目标**: 添加 Trait Objects、关联类型等练习

**修改内容**:
- 在 exercises.md 添加练习：
  - Trait Objects 使用
  - Associated types 实现
  - Supertraits
  - Newtype pattern
- 创建相关示例文件

**验证**:
- 新练习覆盖高级特性
- 代码可运行

---

#### Task 9: 为 module-08 (原 module-04) 补充生命周期练习
**目标**: 添加更多生命周期实战练习

**修改内容**:
- 在 exercises.md 添加练习：
  - 结构体生命周期
  - 多生命周期参数
  - 生命周期与 Trait Objects
- 创建相关示例文件

**验证**:
- 练习题有实际意义
- 难度适中

---

#### Task 10: 为 module-09 补充并发练习
**目标**: 添加线程池、async 等实战练习

**修改内容**:
- 在 exercises.md 添加练习：
  - 线程池实现
  - RwLock 使用
  - async/await 实战
  - Channel 生产者-消费者
- 更新综合练习.md

**验证**:
- 练习题实用
- 参考答案完整

---

#### Task 11: 为 module-06 补充集合练习
**目标**: 添加 BTreeMap、VecDeque 等练习

**修改内容**:
- 在 exercises.md 添加练习：
  - BTreeMap 有序操作
  - VecDeque 双端队列
  - 综合数据结构设计

**验证**:
- 新练习有意义
- 覆盖新内容

---

### P2 - 优化修改（可选）

#### Task 12: 更新进度追踪文档
**目标**: 反映模块变化，更新进度追踪

**修改内容**:
- 更新 进度.md 模块列表
- 更新 学习指南.md 学习路径
- 更新各模块的自检清单

**验证**:
- 文档一致性
- 链接正确

---

## 执行顺序

建议按以下顺序执行：

1. Task 1 - 调整模块顺序（影响最大，先做）
2. Task 2 - 移除 module-07 生命周期内容
3. Task 5 - 新增 module-11 智能指针
4. Task 6 - 新增 module-12 迭代器
5. Task 3 - 扩充 module-06 集合类型
6. Task 4 - 扩充 module-09 并发编程
7. Task 7-11 - 补充练习题
8. Task 12 - 更新进度文档

---

## 验证标准

每个任务完成后，需要验证：

1. **编译通过**: `cargo build` 成功
2. **测试通过**: `cargo test` 成功
3. **示例可运行**: 所有 example 可执行
4. **文档一致**: README、exercise、综合练习引用正确
5. **链接有效**: 模块间交叉引用链接有效

---

## 提交规范

每个任务完成后提交：

```bash
git add .
git commit -m "Task X: 简短描述"
```

提交消息格式：
- P0 任务: `[P0] Task X: 描述`
- P1 任务: `[P1] Task X: 描述`
- P2 任务: `[P2] Task X: 描述`

---

## 状态追踪

- [ ] Task 1: 调整模块顺序
- [ ] Task 2: 移除 module-07 生命周期内容
- [ ] Task 3: 扩充 module-06 集合类型
- [ ] Task 4: 扩充 module-09 并发编程
- [ ] Task 5: 新增 module-11 智能指针
- [ ] Task 6: 新增 module-12 迭代器
- [ ] Task 7: 补充 module-05 错误处理练习
- [ ] Task 8: 补充 module-07 Trait 练习
- [ ] Task 9: 补充 module-08 生命周期练习
- [ ] Task 10: 补充 module-09 并发练习
- [ ] Task 11: 补充 module-06 集合练习
- [ ] Task 12: 更新进度追踪文档

---

**创建日期**: 2025-01-17
**预计完成**: 2025-01-XX
