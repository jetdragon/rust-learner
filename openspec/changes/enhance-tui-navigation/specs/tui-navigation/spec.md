# TUI 导航增强

## MODIFIED Requirements

### Requirement: 仪表板模块选择光标应精确移动

**Requirement:** 在仪表板界面使用方向键选择模块时，每按一次键光标 SHALL 精确移动一行，而非跳行。

#### Scenario: 上下键精确移动光标

**Given** 用户在仪表板界面
- 选中第 N 个模块

**When** 用户按下向下方向键

**Then** 光标 SHALL 移动到第 N+1 个模块
- 不应跳过任何模块

#### Scenario: 边界情况处理

**Given** 用户在仪表板界面
- 选中第 0 个模块

**When** 用户按下向上方向键

**Then** 光标应保持在第 0 个模块
- 不应移动到负数索引

---

### Requirement: 模块详情界面应显示模块信息和操作选项

**Requirement:** 从仪表板进入模块后，界面 SHALL 显示模块详细信息、文件列表和可用操作。

#### Scenario: 显示模块文件列表

**Given** 用户选择了模块 "02-所有权系统"

**When** 进入模块详情界面

**Then** 界面 SHALL 显示:
- 模块名称
- 模块目录路径
- 可用文件列表 (README.md, exercises.md, tests/, 自检清单.md)
- 每个任务的当前状态

#### Scenario: 使用 VSCode 打开模块文件

**Given** 用户在模块详情界面

**When** 用户选择"打开 README"操作

**Then** 系统 SHALL 使用 `code <path>` 命令在 VSCode 中打开文件

---

### Requirement: 任务操作应支持打开和标记完成两种操作

**Requirement:** 每个任务类型 SHALL 支持两种操作：打开相关文件和标记任务完成。

#### Scenario: 打开任务对应的文件

**Given** 用户在模块详情界面
- 当前任务是"概念学习"
- 状态为未完成

**When** 用户选择"打开"操作

**Then** 系统 SHALL 打开 README.md 文件供学习

#### Scenario: 标记任务完成

**Given** 用户在模块详情界面
- 用户已完成当前任务的学习

**When** 用户选择"标记完成"操作

**Then** 系统 SHALL 更新进度文件
- 任务状态变为完成
- 显示成功消息

---

### Requirement: 进度为 0 的模块应优先显示打开操作

**Requirement:** 当模块所有任务均为未完成状态时，界面 SHALL 优先提示用户打开学习。

#### Scenario: 新模块显示打开提示

**Given** 用户选择了进度为 0 的模块

**When** 进入模块详情界面

**Then** 界面 SHALL:
- 显示"开始学习"提示
- 第一个任务默认选中"打开"操作
- 提示用户先阅读再标记完成

---

## ADDED Requirements

### Requirement: 新增模块详情状态

**Requirement:** 系统 SHALL 新增 `AppState::ModuleDetail` 状态用于显示模块详情。

**State Structure:**
```
ModuleDetail {
    selected_module: usize,
    selected_task: usize,
    focus_area: ModuleFocus,
}
```

**Where ModuleFocus:**
```
enum ModuleFocus {
    TaskList,    // 任务列表焦点
    Action,      // 操作按钮焦点
}
```

#### Scenario: 进入模块详情状态

**Given** 用户在仪表板界面选中模块 2

**When** 按下 Enter 键

**Then** 应用状态变为 `ModuleDetail { selected_module: 2, selected_task: 0, focus_area: TaskList }`

---

### Requirement: 新增模块详情界面

**Requirement:** 系统 SHALL 实现 `draw_module_detail` 函数渲染模块详情界面。

**UI Elements:**
- 模块标题区域
- 文件列表区域（可点击）
- 任务操作区域（每行显示：任务名 [打开] [完成]）
- 底部操作提示

**Shortcuts:**
- `o` / `O`: 打开当前任务文件
- `Space`: 标记当前任务完成
- `Tab`: 切换焦点（任务列表/操作）
- `Enter`: 确认操作
- `Esc`: 返回仪表板

#### Scenario: 渲染模块详情界面

**Given** 应用状态为 ModuleDetail

**When** 调用 draw_module_detail 渲染

**Then** 界面 SHALL 显示:
- 模块标题 "模块名称"
- 文件路径列表
- 5 个任务行，每行包含任务名和 [打开] [完成] 按钮
