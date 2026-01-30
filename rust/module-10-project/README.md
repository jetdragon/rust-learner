# 10-综合项目

这是一个综合项目模块，将运用前面学习的所有 Rust 知识来构建一个完整的命令行待办事项应用。

## 项目概述

我们将构建一个功能完整的 Todo 应用，它将包含：

- **数据结构**：使用结构体和枚举组织数据
- **集合操作**：使用 Vec 和 HashMap 管理任务
- **错误处理**：使用 Result 和 Option 处理错误情况
- **所有权系统**：正确管理数据的生命周期
- **特征和泛型**：使用特征定义行为
- **文件 I/O**：持久化存储任务数据
- **模块系统**：组织代码结构
- **迭代器**：处理和过滤任务列表
- **生命周期**：管理引用的有效性
- **并发**：使用多线程处理任务

## 核心数据结构

```rust
pub struct Todo {
    id: u32,
    title: String,
    description: Option<String>,
    completed: bool,
    priority: Priority,
    tags: Vec<String>,
    created_at: DateTime<Utc>,
}

pub enum Priority {
    Low,
    Medium,
    High,
}
```

## 主要功能

### 1. 任务管理
- 创建新任务
- 编辑任务
- 删除任务
- 标记完成/未完成

### 2. 任务查询
- 列出所有任务
- 按状态过滤
- 按优先级排序
- 按标签搜索
- 按关键词搜索

### 3. 数据持久化
- 保存到 JSON 文件
- 从文件加载
- 自动备份

### 4. 命令行界面
- 添加交互式菜单
- 支持命令行参数
- 彩色输出
- 进度显示

## 项目结构

```
module-10-project/
├── src/
│   ├── lib.rs          # 库入口，导出公共API
│   ├── todo.rs         # Todo 核心结构
│   ├── priority.rs     # 优先级枚举
│   ├── store.rs        # 数据存储
│   ├── filter.rs       # 过滤器
│   └── cli.rs          # 命令行界面
├── examples/
│   ├── basic.rs        # 基本用法
│   ├── filtering.rs    # 过滤示例
│   ├── persistence.rs  # 持久化示例
│   └── interactive.rs  # 交互式应用
├── tests/
│   └── integration.rs  # 集成测试
└── Cargo.toml
```

## 学习目标

通过完成这个项目，你将掌握：

1. **综合运用 Rust 知识**
   - 将所有模块的知识整合到一个实际项目中
   - 理解如何选择合适的数据结构和算法

2. **项目架构设计**
   - 模块化设计
   - 代码组织
   - API 设计

3. **错误处理策略**
   - 定义自定义错误类型
   - 错误传播
   - 用户友好的错误信息

4. **测试策略**
   - 单元测试
   - 集成测试
   - 测试覆盖率

5. **文档编写**
   - API 文档
   - 使用示例
   - README 编写

## 快速开始

```rust
use module_10_project::{Todo, TodoList, Priority};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建新的任务列表
    let mut todo_list = TodoList::new();

    // 添加任务
    todo_list.add(
        String::from("学习 Rust"),
        Some(String::from("完成所有练习")),
        Priority::High,
        vec![String::from("学习"), String::from("Rust")],
    );

    // 标记完成
    todo_list.complete(1)?;

    // 保存到文件
    todo_list.save("todos.json")?;

    Ok(())
}
```

## 运行示例

```bash
# 基本用法
cargo run --example basic

# 过滤任务
cargo run --example filtering

# 持久化存储
cargo run --example persistence

# 交互式应用
cargo run --example interactive
```

## 运行测试

```bash
# 运行所有测试
cargo test

# 运行集成测试
cargo test --test integration

# 显示测试输出
cargo test -- --nocapture
```

## 挑战任务

1. **扩展功能**
   - 添加任务截止日期
   - 支持子任务
   - 添加任务提醒

2. **性能优化**
   - 使用索引加速搜索
   - 实现懒加载
   - 优化大量任务的性能

3. **用户体验**
   - 添加进度条
   - 实现自动完成
   - 支持配置文件

4. **高级特性**
   - 使用 async/await
   - 实现 Web API
   - 添加数据库后端
