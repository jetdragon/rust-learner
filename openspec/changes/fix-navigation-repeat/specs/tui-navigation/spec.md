# 修复导航跳行问题

## MODIFIED Requirements

### Requirement: 仪表板布局应正确显示所有模块

**Requirement:** 仪表板界面 SHALL 正确显示所有学习模块，且每个模块占用一行。

**Current Problem:**
- 顶部区域固定 10 行高度
- 进度条 Gauge 占用额外空间
- 模块列表区域高度不足

**Solution:**
- 调整顶部区域高度为 9 行
- 使用文本进度条代替 Gauge widget
- 确保模块列表区域高度足够

#### Scenario: 显示所有模块

**Given** 有 10 个学习模块

**When** 打开仪表板界面

**Then** 界面 SHALL 显示:
- 所有 10 个模块
- 每个模块一行
- 光标可以逐行移动

---

### Requirement: 方向键应精确移动一行

**Requirement:** 在任意列表界面使用方向键时， SHALL 精确移动一行。

**Current Problem:**
- 按一次向下键移动两行
- 光标位置与实际选中项不一致

**Solution:**
- 移除可能导致渲染重叠的 Gauge widget
- 使用文本进度条显示完成度

#### Scenario: 精确移动光标

**Given** 用户在仪表板界面
- 选中第 N 个模块

**When** 按下向下方向键一次

**Then** 光标 SHALL 移动到第 N+1 个模块
- 不应跳过任何模块

---

### Requirement: 进度条应使用文本显示

**Requirement:** 仪表板 SHALL 使用文本进度条显示完成度，而非 Gauge widget。

#### Scenario: 文本进度条显示

**Given** 总体完成度为 60%

**When** 渲染仪表板统计区域

**Then** 界面 SHALL 显示:
```
"[######....] 60%"
```

---

### Requirement: 列表区域高度应自动调整

**Requirement:** 模块列表区域 SHALL 使用 `Constraint::Min(0)` 自动填充剩余空间。

#### Scenario: 列表区域自动填充

**Given** 终端高度为 24 行

**When** 渲染仪表板

**Then** 模块列表区域 SHALL 占据:
- 24 - 9 (头部) - 3 (底部) = 12 行

**Layout:**
```
Header: Constraint::Length(9)
List:  Constraint::Min(0)
Footer: Constraint::Length(3)
```
