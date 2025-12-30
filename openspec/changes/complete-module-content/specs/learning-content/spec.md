## ADDED Requirements

### Requirement: 模块文档完整性
每个学习模块 MUST 包含四个学习文档文件：README.md、exercises.md、综合练习.md、自检清单.md。

#### Scenario: 概念讲解文档
- **GIVEN** 一个学习模块存在
- **WHEN** 查看其 README.md 文件
- **THEN** MUST 包含模块简介和学习目标
- **AND** MUST 详细讲解模块的核心概念
- **AND** MUST 包含代码示例和运行说明
- **AND** MUST 包含常见问题解答

#### Scenario: 练习题文档
- **GIVEN** 一个学习模块
- **WHEN** 查看 exercises.md 文件
- **THEN** MUST 包含至少 5 道练习题
- **AND** 每题 MUST 有难度标记（简单/中等/困难）
- **AND** MUST 包含练习提示和参考答案

#### Scenario: 综合练习文档
- **GIVEN** 一个学习模块
- **WHEN** 查看 综合练习.md 文件
- **THEN** MUST 描述一个综合运用模块知识的项目
- **AND** MUST 有明确的项目需求和验收标准
- **AND** MUST 包含实现指导

#### Scenario: 自检清单文档
- **GIVEN** 一个学习模块
- **WHEN** 查看 自检清单.md 文件
- **THEN** MUST 列出模块应掌握的关键知识点
- **AND** MUST 以可勾选的清单形式呈现
- **AND** MUST 帮助学习者评估掌握程度

### Requirement: 模块代码完整性
每个学习模块 MUST 包含可编译运行的 lib.rs、示例代码和测试用例。

#### Scenario: 库文件实现
- **GIVEN** 一个学习模块存在
- **WHEN** 查看其 src/lib.rs 文件
- **THEN** MUST 包含模块的公共接口导出
- **AND** MUST 有详细的文档注释
- **AND** MUST 能够通过 `cargo build` 编译

#### Scenario: 示例代码可用性
- **GIVEN** 一个学习模块
- **WHEN** 查看 examples/ 目录
- **THEN** MUST 至少有 3 个可运行的 .rs 示例文件
- **AND** 每个示例 MUST 能够通过 `cargo run --example <name>` 运行
- **AND** 示例代码 MUST 包含详细的中文注释
- **AND** 示例 MUST 展示模块的核心概念

#### Scenario: 测试用例覆盖
- **GIVEN** 一个学习模块
- **WHEN** 查看 tests/ 目录
- **THEN** MUST 包含 mod.rs 测试文件
- **AND** MUST 有至少 5 个测试函数
- **AND** 所有测试 MUST 能够通过 `cargo test` 通过
- **AND** 测试 MUST 覆盖模块的关键概念

### Requirement: 所有权模块内容 (Module-02)
Module-02 MUST 包含展示所有权系统的完整代码和测试。

#### Scenario: 所有权基本规则
- **WHEN** 查看 examples/ownership_basics.rs
- **THEN** MUST 展示 Rust 所有权三条基本规则
- **AND** MUST 包含变量作用域示例
- **AND** MUST 包含移动发生的情况

#### Scenario: 借用和引用
- **WHEN** 查看 examples/borrowing.rs
- **THEN** MUST 展示不可变借用
- **AND** MUST 展示可变借用规则
- **AND** MUST 展示借用检查器的行为

#### Scenario: 切片类型
- **WHEN** 查看 examples/slices.rs
- **THEN** MUST 展示字符串切片 &str
- **AND** MUST 展示数组切片
- **AND** MUST 包含切片边界处理的示例

#### Scenario: 所有权测试
- **WHEN** 运行 `cargo test -p module-02-ownership`
- **THEN** 所有测试 MUST 通过
- **AND** 测试 MUST 验证所有权、借用、引用等概念

### Requirement: 结构体与枚举模块内容 (Module-03)
Module-03 MUST 包含展示结构体和枚举的完整代码和测试。

#### Scenario: 结构体定义
- **WHEN** 查看 examples/structs.rs
- **THEN** MUST 展示字段结构体
- **AND** MUST 展示元组结构体
- **AND** MUST 展示单元结构体
- **AND** MUST 包含结构体实例化和字段访问示例

#### Scenario: 枚举和匹配
- **WHEN** 查看 examples/enums.rs
- **THEN** MUST 展示枚举定义
- **AND** MUST 展示带数据的枚举变体
- **AND** MUST 展示 match 表达式匹配枚举

#### Scenario: Option 和 Result 类型
- **WHEN** 查看 examples/option.rs 和 examples/result_enum.rs
- **THEN** MUST 展示 Option 类型的 Some 和 None
- **AND** MUST 展示 Result 类型的 Ok 和 Err
- **AND** MUST 包含处理这些类型的模式匹配

### Requirement: 模式匹配模块内容 (Module-04)
Module-04 MUST 包含展示模式匹配的完整代码和测试。

#### Scenario: Match 表达式
- **WHEN** 查看 examples/match_basics.rs
- **THEN** MUST 展示 match 基本语法
- **AND** MUST 展示穷尽性要求
- **AND** MUST 包含通配符模式 _

#### Scenario: 高级模式
- **WHEN** 查看 examples/patterns.rs
- **THEN** MUST 展示字面值模式
- **AND** MUST 展示范围模式
- **AND** MUST 展示或模式
- **AND** MUST 展示守卫

#### Scenario: if let 和 while let
- **WHEN** 查看 examples/if_let.rs
- **THEN** MUST 展示 if let 简化模式匹配
- **AND** MUST 展示 while let 重复模式匹配
- **AND** MUST 包含何时使用这些糖语法

### Requirement: 错误处理模块内容 (Module-05)
Module-05 MUST 包含展示错误处理的完整代码和测试。

#### Scenario: Panic 和可恢复错误
- **WHEN** 查看 examples/panic.rs
- **THEN** MUST 展示 panic! 宏的使用
- **AND** MUST 展示 panic 时的栈展开
- **AND** MUST 包含何时使用 panic 的指导

#### Scenario: Result 类型
- **WHEN** 查看 examples/result.rs
- **THEN** MUST 展示 Result 的 Ok 和 Err
- **AND** MUST 展示 ? 操作符
- **AND** MUST 展示组合算子如 map、and_then

#### Scenario: 错误传播
- **WHEN** 查看 examples/error_propagation.rs
- **THEN** MUST 展示错误传播模式
- **AND** MUST 展示使用 ? 简化传播
- **AND** MUST 包含自定义错误消息

### Requirement: 集合模块内容 (Module-06)
Module-06 MUST 包含展示集合类型的完整代码和测试。

#### Scenario: 向量操作
- **WHEN** 查看 examples/vectors.rs
- **THEN** MUST 展示 Vec 创建
- **AND** MUST 展示元素添加和删除
- **AND** MUST 展示向量迭代

#### Scenario: 字符串处理
- **WHEN** 查看 examples/strings.rs
- **THEN** MUST 展示 String 和 &str 区别
- **AND** MUST 展示字符串拼接
- **AND** MUST 展示字符串切片的 UTF-8 边界

#### Scenario: HashMap 和迭代器
- **WHEN** 查看 examples/hashmap.rs 和 examples/iterators.rs
- **THEN** MUST 展示 HashMap 创建和查询
- **AND** MUST 展示迭代器适配器如 map、filter
- **AND** MUST 展示消费器如 collect、fold

### Requirement: 泛型与 Trait 模块内容 (Module-07)
Module-07 MUST 包含展示泛型和 Trait 的完整代码和测试。

#### Scenario: 泛型函数和类型
- **WHEN** 查看 examples/generics.rs
- **THEN** MUST 展示泛型函数
- **AND** MUST 展示泛型结构体
- **AND** MUST 展示枚举中的泛型

#### Scenario: Trait 定义和实现
- **WHEN** 查看 examples/traits.rs
- **THEN** MUST 展示 trait 定义
- **AND** MUST 展示为类型实现 trait
- **AND** MUST 展示 trait 作为参数
- **AND** MUST 展示 trait bound

### Requirement: 生命周期模块内容 (Module-08)
Module-08 MUST 包含展示生命周期的完整代码和测试。

#### Scenario: 生命周期注解
- **WHEN** 查看 examples/lifetime_basics.rs
- **THEN** MUST 展示函数中的生命周期注解
- **AND** MUST 解释为什么需要生命周期
- **AND** MUST 包含多生命周期参数示例

#### Scenario: 结构体生命周期
- **WHEN** 查看 examples/struct_lifetimes.rs
- **THEN** MUST 展示持有引用的结构体
- **AND** MUST 展示生命周期省略规则
- **AND** MUST 包含 'static 生命周期示例

### Requirement: 并发模块内容 (Module-09)
Module-09 MUST 包含展示并发的完整代码和测试。

#### Scenario: 线程创建
- **WHEN** 查看 examples/threads.rs
- **THEN** MUST 展示 thread::spawn 创建线程
- **AND** MUST 展示 move 闭包的使用
- **AND** MUST 展示 join 等待线程完成

#### Scenario: 消息传递和共享状态
- **WHEN** 查看 examples/channels.rs 和 examples/mutex.rs
- **THEN** MUST 展示 channel 创建和使用
- **AND** MUST 展示 Mutex<T> 同步
- **AND** MUST 展示 Arc<T> 多线程共享

### Requirement: 实战项目模块内容 (Module-10)
Module-10 MUST 包含一个综合运用 Rust 知识的实战项目。

#### Scenario: 项目完整性
- **WHEN** 查看 module-10-project
- **THEN** src/lib.rs MUST 包含项目的核心功能
- **AND** examples/ 目录 MUST 包含使用示例
- **AND** tests/ 目录 MUST 包含测试用例
- **AND** 项目 MUST 综合运用前面模块的知识

### Requirement: Git 提交规范
每个模块的完成变更 MUST 通过独立的 git commit 记录。

#### Scenario: 模块提交
- **GIVEN** 一个模块的所有内容已完成
- **WHEN** 变更被提交到 git
- **THEN** commit 消息 MUST 清楚描述模块名称和变更内容
- **AND** commit MUST 仅包含该模块的文件
- **AND** commit 消息 MUST 遵循项目的约定

#### Scenario: 最终验证提交
- **GIVEN** 所有模块都已完成
- **WHEN** 最终验证通过
- **THEN** MUST 创建一个总结性的 commit
- **AND** commit 消息 MUST 列出所有完成的模块
