## ADDED Requirements

### Requirement: 标准模块目录结构
每个学习模块 SHALL 遵循统一的目录结构，便于学习者导航和查找内容。

#### Scenario: 模块目录布局
- **GIVEN** 一个新的学习模块被创建
- **WHEN** 查看模块目录
- **THEN** SHALL 包含以下结构：
  ```
  01-模块名称/
  ├── README.md           # 概念说明
  ├── examples/           # 代码示例
  │   ├── basic.rs
  │   └── advanced.rs
  ├── exercises/          # 练习题模板
  │   ├── exercise1.rs
  │   └── solutions/      # 参考答案
  │       └── exercise1.rs
  ├── tests/              # 验证测试
  │   └── mod.rs
  ├── exercises.md        # 练习题列表
  ├── 综合练习.md         # 综合项目
  └── 自检清单.md         # 学习检查点
  ```

#### Scenario: 可选模块扩展
- **GIVEN** 一个模块需要额外资源
- **WHEN** 添加额外文件
- **THEN** SHALL 可以添加 `resources/` 目录存放图片、数据文件等
- **AND** SHALL 可以添加 `延伸阅读.md` 列出推荐资源

### Requirement: Cargo 工作空间配置
项目 MUST 使用 Cargo 工作空间管理所有学习模块。

#### Scenario: 工作空间结构
- **GIVEN** 项目根目录
- **WHEN** 查看 `Cargo.toml`
- **THEN** MUST 配置为工作空间模式
- **AND** 每个模块 MUST 可作为独立的 binary 或 library

#### Scenario: 独立编译模块
- **GIVEN** 任意学习模块
- **WHEN** 运行 `cargo build -p <模块名>`
- **THEN** MUST 只编译该模块及其依赖
- **AND** 编译时间应相对较短

### Requirement: 命名规范
项目中的文件和代码 SHALL 遵循一致的中文和英文混合命名规范。

#### Scenario: 目录命名
- **GIVEN** 创建新的学习目录
- **WHEN** 命名目录
- **THEN** SHALL 使用 `<序号>-<主题>` 格式（如 `01-基础入门`）
- **AND** 序号 SHALL 使用两位数字以支持10个以上的模块

#### Scenario: 代码文件命名
- **GIVEN** 创建 Rust 源文件
- **WHEN** 命名文件
- **THEN** SHALL 使用 snake_case 英文命名（如 `variable_types.rs`）
- **AND** 名称 SHOULD 清晰表达文件内容的主题

### Requirement: 文档模板规范
所有模块文档 SHALL 遵循统一的格式和结构。

#### Scenario: README 模板
- **GIVEN** 一个新模块的 `README.md`
- **WHEN** 学习者阅读文档
- **THEN** SHALL 包含以下章节：
  - 模块简介
  - 学习目标
  - 前置知识
  - 核心概念
  - 代码示例
  - 常见问题
  - 下一步

#### Scenario: 练习题模板
- **GIVEN** `exercises.md` 文件
- **WHEN** 学习者查看练习
- **THEN** 每题 SHALL 包含：
  - 题目描述
  - 难度标记（简单/中等/困难）
  - 提示（可选，可折叠）
  - 验证命令
