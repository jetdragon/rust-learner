# fix-tui-rendering: 修复 TUI 渲染和导航问题

## 问题描述

1. **方向键移动跳行** - 按一次方向键移动两行
2. **启动直接进入仪表板** - 应该先显示主菜单
3. **Tab 键无法切换焦点** - 左右两栏界面中，Tab 键无法切换焦点
4. **菜单刷新残留** - 切换界面后，旧界面的边框和分割线残留

## 问题分析

### 问题 1: 方向键跳行

**可能原因**:
- `ListState` 和 `highlight_symbol` 机制冲突
- 事件处理中可能有多次渲染触发

### 问题 2: 启动直接进入仪表板

**原因**:
- `main.rs` 中 `cli.command.is_none()` 判断逻辑问题
- 当不传参数时，`command` 是 `None`，但这应该触发主菜单

### 问题 3: Tab 键无法切换焦点

**原因**:
- `handle_update_progress_key` 中的 Tab 处理可能不正确
- FocusArea 状态切换后，UI 没有正确响应

### 问题 4: 菜单刷新残留

**原因**:
- ratatui 需要在绘制前清除区域
- 布局约束可能没有覆盖整个区域

## 解决方案

### 1. 修复启动流程

- 修改 `main.rs`，确保启动时进入 `MainMenu` 状态
- 默认使用 `--tui` 模式直接显示主菜单

### 2. 修复方向键跳行

- 移除 `highlight_symbol`，使用手动前缀方式
- 确保 `ListState` 只在单一位置使用

### 3. 修复 Tab 焦点切换

- 检查 `handle_update_progress_key` 中的 Tab 处理逻辑
- 确保 UI 根据 `focus_area` 正确渲染

### 4. 修复刷新残留

- 在 `ui` 函数中使用 `f.buffer.reset()` 清除区域
- 确保每个界面完整覆盖其区域

## 影响范围

- `learning-companion/src/main.rs`
- `learning-companion/src/tui.rs`
