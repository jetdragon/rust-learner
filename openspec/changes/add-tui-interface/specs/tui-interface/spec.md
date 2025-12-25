# Spec Delta: TUI Interface

## ADDED Requirements

### Requirement: TUI 模式启动
The application SHALL provide a `--tui` or `--interactive` flag to launch the interactive Terminal User Interface mode.

#### Scenario: 使用 tui 标志启动
```bash
cargo run -- --tui
```
**期望结果**: 程序进入交互式 TUI 模式，显示主菜单

#### Scenario: 使用简写标志启动
```bash
cargo run -- -i
```
**期望结果**: 程序进入交互式 TUI 模式

#### Scenario: 无参数时默认 TUI
```bash
cargo run
```
**期望结果**: 程序默认进入 TUI 模式（或询问用户选择模式）

---

### Requirement: 键盘导航
The application SHALL support standard keyboard navigation for all TUI operations.

#### Scenario: 方向键选择菜单项
**当** 用户在主菜单界面
**且** 按下 `↓` 键
**则** 光标移动到下一个选项

#### Scenario: Enter 确认选择
**当** 用户选中一个菜单项
**且** 按下 `Enter` 键
**则** 进入对应的功能界面

#### Scenario: Esc 返回上级
**当** 用户在子界面
**且** 按下 `Esc` 键
**则** 返回上一级界面

#### Scenario: q 键退出
**当** 用户按下 `q` 键
**则** 程序退出

---

### Requirement: 主菜单界面
The TUI SHALL display a clear main menu listing all available functions.

#### Scenario: 显示主菜单选项
**当** 用户进入 TUI 模式
**则** 应显示以下选项：
- 查看学习仪表板
- 更新学习进度
- 开始练习测试
- 查看成就
- 设置学习提醒
- 导出学习数据
- 退出程序

#### Scenario: 高亮当前选中项
**当** 用户在主菜单中导航
**则** 当前选中项应有明显视觉高亮

---

### Requirement: 进度更新界面
The application SHALL allow users to update learning progress by selecting modules and tasks in the TUI.

#### Scenario: 模块列表选择
**当** 用户进入"更新学习进度"界面
**则** 应显示所有可用学习模块列表

#### Scenario: 任务复选框
**当** 用户选择一个模块后
**则** 应显示该模块的任务复选框（概念、示例、练习、综合、自检）

#### Scenario: 空格键切换复选框
**当** 用户聚焦在一个任务复选框
**且** 按下空格键
**则** 该任务状态被切换

#### Scenario: Tab 在字段间切换
**当** 用户在进度更新界面
**且** 按下 `Tab` 键
**则** 焦点在模块选择器和任务复选框组之间切换

---

### Requirement: 练习界面
The application SHALL allow users to complete practice tests within the TUI.

#### Scenario: 模块和题目数量选择
**当** 用户进入"开始练习测试"界面
**则** 应能选择模块和题目数量

#### Scenario: 题目显示
**当** 练习开始
**则** 应逐题显示，包括：
- 题目编号和进度
- 题目内容
- 选项列表

#### Scenario: 选项选择
**当** 用户在选项列表中
**且** 使用方向键 + Enter 选择
**则** 答案被记录，进入下一题

#### Scenario: 实时反馈
**当** 用户提交答案
**则** 立即显示正确/错误反馈和解析

---

### Requirement: 仪表板界面
The TUI SHALL display learning progress statistics in a dashboard view.

#### Scenario: 总体进度显示
**当** 用户进入"学习仪表板"界面
**则** 应显示：
- 总体完成百分比
- 进度条可视化
- 连续学习天数

#### Scenario: 模块状态列表
**当** 用户在仪表板中
**则** 应显示所有模块的：
- 完成状态图标
- 名称和 ID
- 任务完成情况
- 掌握程度

---

### Requirement: 响应式布局
The TUI SHALL adapt to different terminal sizes.

#### Scenario: 最小尺寸要求
**当** 终端尺寸小于 80x24
**则** 程序应显示警告并等待终端调整

#### Scenario: 动态内容调整
**当** 终端尺寸改变
**则** 界面应自动调整布局以适应新尺寸

---

### Requirement: 兼容性保留
The application MUST maintain full compatibility with the original CLI mode.

#### Scenario: CLI 命令仍然工作
**当** 用户使用原有 CLI 命令
```bash
cargo run -- dashboard
cargo run -- update -m module-01-basics -t concept
```
**则** 程序应以原有方式正常工作

---

### Requirement: 错误处理
The TUI SHALL gracefully handle error conditions without crashing.

#### Scenario: 数据库错误弹窗
**当** 发生数据库错误
**则** 应在 TUI 中显示错误弹窗，而不是崩溃

#### Scenario: 输入验证
**当** 用户输入无效数据（如无效时间）
**则** 应显示内联错误提示

---

### Requirement: 帮助信息
The interface SHALL display operation hints at the bottom of the screen.

#### Scenario: 底部提示栏
**当** 用户在任何界面
**则** 底部应显示当前可用的按键操作提示

---

## MODIFIED Requirements

### Requirement: 应用启动方式
The application SHALL support launching in TUI mode without arguments, while preserving CLI subcommand compatibility. Users may use `--tui` or `--interactive` flags to explicitly launch TUI mode.

#### Scenario: 无参数启动进入 TUI
**当** 用户运行 `cargo run` 不带任何参数
**则** 程序应进入 TUI 模式

#### Scenario: 子命令进入 CLI 模式
**当** 用户运行 `cargo run -- dashboard` 等子命令
**则** 程序应保持原有的 CLI 行为

#### Scenario: 显式 TUI 标志
**当** 用户运行 `cargo run -- --tui`
**则** 程序应进入 TUI 模式
