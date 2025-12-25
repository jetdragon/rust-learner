# Change: 添加交互式终端界面 (TUI)

## Why

当前学习伴侣使用传统 CLI 命令行参数方式（如 `cargo run -- update -m module-01-basics -t concept`），操作繁琐且需要记忆命令。用户需要更直观的交互方式，通过键盘导航即可完成所有操作。

## What Changes

- **添加 TUI 模式**: 使用 ratatui 实现交互式终端界面
- **双模式支持**: 保留原有 CLI 模式，新增 `--tui` 标志启动 TUI 模式
- **默认行为**: 无参数时默认进入 TUI 模式
- **键盘导航**: 支持方向键、Tab、Enter、Esc、q 等标准按键操作
- **界面组件**: 主菜单、仪表板、进度更新、练习、成就、提醒设置等 TUI 视图
- **错误处理**: TUI 内的优雅错误弹窗，不崩溃

## Impact

### Affected specs
- **新增**: `tui-interface` - TUI 交互界面规范

### Affected code
- **learning-companion/src/main.rs**: 添加 TUI 启动分支和 `--tui` 标志
- **learning-companion/src/ui.rs**: 可能需要重构或补充
- **新增 learning-companion/src/tui.rs**: TUI 状态管理和事件处理

### Dependencies
项目已包含 `ratatui` 和 `crossterm` 依赖，无需额外安装

### Compatibility
- **CLI 模式**: 完全保留，所有现有命令继续工作
- **Breaking Changes**: 无
