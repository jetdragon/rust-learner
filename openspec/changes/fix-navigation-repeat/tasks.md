# fix-navigation-repeat Tasks

## 任务清单

1. [x] 调整仪表板布局约束
   - [x] 将顶部区域改为 `Constraint::Length(9)`
   - [x] 移除硬编码的 gauge_area 位置

2. [x] 移除 Gauge widget，改为文本进度条
   - [x] 在 stats_lines 中添加文本进度条
   - [x] 移除 Gauge 渲染代码

3. [x] 验证模块列表显示
   - [x] 确保所有模块可以显示
   - [x] 确保光标可以逐行移动

4. [x] 运行测试
   - [x] cargo test 确保无回归 (15/15 通过)

## 验证标准

- [x] 光标移动精确一行
- [x] 所有模块可见
- [x] 测试通过

## 变更摘要

- 顶部区域高度从 10 行减少到 9 行
- 移除 Gauge widget，使用文本进度条 `[███░░░░░░] 50%`
- 移除 Gauge import
