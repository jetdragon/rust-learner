# TUI 渲染修复

## MODIFIED Requirements

### Requirement: 启动时应进入主菜单

**Requirement:** 当用户运行程序不带参数时， SHALL 显示主菜单而非仪表板。

**Current Problem:**
- `cli.command.is_none()` 时直接进入 TUI，但初始状态可能不是 MainMenu

**Solution:**
- 确保 `App::new()` 初始状态为 `MainMenu`
- 主菜单选项第一个应为"查看学习仪表板"

#### Scenario: 不带参数启动

**Given** 用户运行 `learning-companion`

**When** 不提供任何参数

**Then** 界面 SHALL 显示主菜单
- 显示 7 个菜单选项
- 第一个选项高亮

---

### Requirement: 方向键应精确移动一行

**Requirement:** 在任意列表界面使用方向键时， SHALL 精确移动一行。

**Current Problem:**
- 按一次向下键移动两行
- `highlight_symbol` 与手动前缀冲突

**Solution:**
- 统一使用手动前缀方式，移除 `highlight_symbol`
- 确保 `ListState` 正确管理选中状态

#### Scenario: 仪表板模块选择

**Given** 在仪表板界面
- 当前选中第 N 个模块

**When** 按一次向下键

**Then** 光标 SHALL 移动到第 N+1 个模块
- 不应跳过任何模块

---

### Requirement: Tab 键应能切换焦点

**Requirement:** 在更新进度等左右两栏界面中，Tab 键 SHALL 能在模块列表和任务列表之间切换焦点。

**Current Problem:**
- Tab 键按下后焦点立即跳回

**Solution:**
- 检查 `handle_update_progress_key` 中的 Tab 处理
- 确保 `focus_area` 正确更新并持久化

#### Scenario: Tab 切换焦点

**Given** 在更新进度界面
- 当前焦点在模块列表

**When** 按 Tab 键

**Then** 焦点 SHALL 切换到任务列表
- 任务列表边框变为黄色高亮
- 按 Tab 再次切换回模块列表

---

### Requirement: 界面切换时不应有残留

**Requirement:** 切换界面后，旧界面的边框和分割线 SHALL 不残留。

**Current Problem:**
- 切换到新界面后，旧界面的边框还在

**Solution:**
- 在 `ui` 函数开始时清除内容区域
- 确保每个界面完整覆盖其布局区域

#### Scenario: 界面切换

**Given** 在仪表板界面

**When** 按 O 进入模块详情界面

**Then** 仪表板的边框 SHALL 完全消失
- 模块详情界面正确显示
- 无残留边框或分割线

---

## ADDED Requirements

### Requirement: 启动流程验证

**Requirement:** 应用启动时 SHALL 验证初始状态为主菜单。

#### Scenario: 验证初始状态

**Given** 创建新的 App 实例

**When** 调用 `App::new(path)`

**Then** 应用的初始状态 SHALL 为 `AppState::MainMenu`
- `state_stack` 应为空
- `main_menu_selected` 应为 0

**Verification:**
```rust
let app = App::new(path.to_string());
assert_eq!(app.state, AppState::MainMenu);
```
