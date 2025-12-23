## ADDED Requirements

### Requirement: 中文学习路径结构
项目 SHALL 按难度递进的顺序组织学习模块，每个模块对应一个 Rust 主题。

#### Scenario: 学习路径导航
- **GIVEN** 一个学习者打开项目
- **WHEN** 他们查看目录结构
- **THEN** SHALL 看到从 01 到 10 按顺序编号的学习目录
- **AND** 目录名 SHALL 使用中文描述主题（如 `01-基础入门`）

#### Scenario: 难度递进
- **GIVEN** 学习路径包含多个模块
- **WHEN** 模块按编号排序
- **THEN** 较小编号的模块 SHALL 涵盖基础概念
- **AND** 较大编号的模块 SHALL 逐步引入更高级的主题

### Requirement: 模块内容完整性
每个学习模块 MUST 包含四种类型的文件：概念说明文档、代码示例、练习题、验证测试。

#### Scenario: 概念说明
- **GIVEN** 一个学习模块存在
- **WHEN** 学习者打开 `README.md`
- **THEN** MUST 看到详细的概念解释（中文）
- **AND** 解释 MUST 包含语法、用法、常见陷阱

#### Scenario: 代码示例
- **GIVEN** 一个学习模块
- **WHEN** 学习者查看 `examples/` 目录
- **THEN** MUST 至少有一个可运行的 `.rs` 文件
- **AND** 代码 MUST 包含详细的中文注释

#### Scenario: 练习题
- **GIVEN** 一个学习模块
- **WHEN** 学习者打开 `exercises.md`
- **THEN** MUST 看到该主题相关的练习题
- **AND** 每题 MUST 有难度标记（简单/中等/困难）

### Requirement: 学习效果验证
每个模块 SHALL 提供机制让学习者检验自己的理解程度。

#### Scenario: 自动化测试
- **GIVEN** 一个学习模块包含练习
- **WHEN** 学习者运行 `cargo test`
- **THEN** 测试 SHALL 验证练习答案的正确性
- **AND** 失败的测试 SHALL 给出有用的错误提示

#### Scenario: 进度追踪
- **GIVEN** 学习者完成一个模块
- **WHEN** 他们在 `进度.md` 中标记
- **THEN** SHALL 可以记录完成日期和心得
- **AND** 后续 SHALL 可以回顾学习历程

### Requirement: 代码规范
所有代码和文档 SHALL 使用统一的中文规范。

#### Scenario: 中文注释
- **GIVEN** 任何 `.rs` 源文件
- **WHEN** 代码包含注释
- **THEN** 注释 SHALL 使用中文
- **AND** SHALL 解释代码的作用和原因

#### Scenario: 中文错误消息
- **GIVEN** 测试或示例代码
- **WHEN** 需要显示错误或提示信息
- **THEN** 消息 SHALL 使用中文
- **AND** SHALL 清晰指导学习者如何修复问题

### Requirement: 可独立运行示例
每个示例代码 MUST 能够独立编译和运行。

#### Scenario: 编译验证
- **GIVEN** 任意示例文件
- **WHEN** 运行 `cargo build`
- **THEN** 示例 MUST 成功编译
- **AND** 不应有警告（除非是教学目的）

#### Scenario: 运行输出
- **GIVEN** 一个可执行的示例
- **WHEN** 运行 `cargo run --bin <示例名>`
- **THEN** 程序 MUST 输出预期的结果
- **AND** 输出 MUST 包含中文说明

### Requirement: 学习检查点
每个模块 SHALL 在结束时有一个综合练习检验学习效果。

#### Scenario: 综合练习
- **GIVEN** 学习者完成模块的所有内容
- **WHEN** 他们查看 `综合练习.md`
- **THEN** MUST 看到一个需要综合运用模块知识的项目
- **AND** 项目 MUST 有明确的验收标准

#### Scenario: 自检清单
- **GIVEN** 一个学习模块
- **WHEN** 学习者查看 `自检清单.md`
- **THEN** MUST 列出该模块应掌握的关键知识点
- **AND** 学习者 MUST 可以勾选已掌握的内容
