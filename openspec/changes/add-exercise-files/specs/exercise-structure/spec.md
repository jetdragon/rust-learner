## MODIFIED Requirements

### Requirement: 标准模块目录结构
每个学习模块 SHALL 包含完整的练习文件结构，包括练习模板和解答文件。

#### Scenario: 练习文件存在性
- **GIVEN** 任意学习模块
- **WHEN** 查看 `exercises/` 目录
- **THEN** SHALL 包含至少 3 个练习文件（exercise1.rs, exercise2.rs, 等）
- **AND** 每个练习文件 SHALL 对应 `exercises.md` 中的一道题目
- **AND** SHALL 包含 `solutions/` 子目录，内含对应的解答文件

#### Scenario: 练习文件模板格式
- **GIVEN** 一个练习文件（如 `exercise1.rs`）
- **WHEN** 学习者打开文件
- **THEN** SHALL 包含以下结构：
  ```rust
  //! 练习 1: [练习标题]
  //!
  //! # 目标
  //! [描述学习目标]
  //!
  //! # 任务
  //! TODO: [具体需要实现的功能]

  // 提供的骨架代码
  pub fn function_name() -> ReturnType {
      // TODO: 实现这个函数
      unimplemented!()
  }

  #[cfg(test)]
  mod tests {
      use super::*;

      #[test]
      fn test_function_name() {
          // 测试代码
      }
  }
  ```

#### Scenario: 解答文件格式
- **GIVEN** `solutions/exercise1.rs` 解答文件
- **WHEN** 学习者查看解答
- **THEN** SHALL 包含完整的函数实现
- **AND** SHALL 包含详细的注释解释实现思路
- **AND** SHALL 在文件顶部添加注释：
  ```rust
  //! # 练习 1 解答
  //!
  //! **提示**：请先尝试自己实现，再参考此解答！
  //!
  //! # 实现思路
  //! [解释解题思路和关键点]
  ```

#### Scenario: 练习与文档关联
- **GIVEN** `exercises.md` 文件
- **WHEN** 描述练习题目
- **THEN** 每题 SHALL 包含对应的文件引用：
  ```markdown
  ## 练习 1: [标题]

  **练习文件**: `exercises/exercise1.rs`
  **解答文件**: `exercises/solutions/exercise1.rs`

  [题目描述]
  ```

#### Scenario: 练习可编译性
- **GIVEN** 一个练习模板文件
- **WHEN** 运行 `cargo build --bin exercise1`
- **THEN** SHOULD 能够编译（使用 `unimplemented!()` 宏作为占位符）
- **AND** 当运行测试时 SHOULD 失败（提示需要实现）

#### Scenario: 解答正确性
- **GIVEN** `solutions/exercise1.rs` 解答文件
- **WHEN** 运行 `cargo test --test exercise1_solution`
- **THEN** MUST 通过所有测试

## ADDED Requirements

### Requirement: 练习文件验证

每个模块的练习文件 MUST 有对应的测试验证实现正确性。

#### Scenario: 测试覆盖
- **GIVEN** 任意练习文件
- **WHEN** 查看文件内容
- **THEN** MUST 包含至少一个测试函数
- **AND** 测试 SHOULD 覆盖正常边界情况

#### Scenario: 集成测试
- **GIVEN** 模块的 `tests/` 目录
- **WHEN** 查看 `exercises_tests.rs`
- **THEN** MUST 包含对所有练习的集成测试
- **AND** 每个练习 MUST 有对应的测试模块

### Requirement: 练习文件元数据

练习文件 MUST 包含足够的元数据信息帮助学习者理解练习目标。

#### Scenario: 难度标记
- **GIVEN** 练习文件
- **WHEN** 查看文件顶部的文档注释
- **THEN** MUST 标注难度级别（简单/中等/困难）
- **AND** MUST 包含预计完成时间

#### Scenario: 前置知识
- **GIVEN** 练习文件
- **WHEN** 查看文档注释
- **THEN** MUST 列出完成练习所需的前置知识
- **AND** SHOULD 包含相关概念的文档链接或示例引用
