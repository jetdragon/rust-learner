# TUI 交互修复

## ADDED Requirements

### Requirement: 空格键支持焦点切换

**Requirement:** TUI 界面在更新进度界面中，Tab 键和空格键 SHALL 能够切换焦点区域（模块列表/任务列表）。

#### Scenario: 使用空格键切换焦点

**Given** 用户在更新进度界面
- 当前焦点在模块列表上

**When** 用户按下空格键

**Then** 焦点应该切换到任务列表

---

### Requirement: 空格键支持确认操作

**Requirement:** TUI 界面在确认界面中，Tab 键、空格键和右方向键 SHALL 能够选择确认选项。

#### Scenario: 使用空格键确认选择

**Given** 用户在更新进度确认界面
- 确认选项默认为"否"

**When** 用户按下空格键

**Then** 确认选项应该切换为"是"

#### Scenario: 使用空格键确认并保存

**Given** 用户已通过空格键选择"是"

**When** 用户按下 Enter 键

**Then** 进度应该被保存
- 界面应该返回仪表板
- 应该显示成功消息

---

### Requirement: 更新进度界面显示优化

**Requirement:** 更新进度界面 SHALL 清晰显示当前选中的模块名称。

#### Scenario: 显示当前模块名称

**Given** 用户选择了某个模块

**When** 进入更新进度界面

**Then** 任务列表区域应该显示"当前模块: [模块名称]"
