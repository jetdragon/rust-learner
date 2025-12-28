# fix-tui-rendering Tasks

## 任务清单

1. [x] 验证启动流程
   - [x] 检查 App::new() 初始状态
   - [x] 确保默认进入 MainMenu

2. [x] 修复方向键跳行
   - [x] 移除所有界面的 highlight_symbol
   - [x] 使用手动前缀方式 (">> ")
   - [x] 同样修复主菜单、仪表板、更新进度、模块详情界面

3. [x] 修复 Tab 焦点切换
   - [x] 使用手动前缀方式正确显示焦点
   - [x] 验证 focus_area 状态切换
   - [x] 确保 UI 正确响应焦点变化

4. [x] 修复界面刷新残留
   - [x] 使用 Paragraph 完整覆盖区域
   - [x] 确保无残留边框

5. [x] 运行测试验证
   - [x] cargo test 确保无回归 (15/15 通过)

## 验证标准

- [x] 启动显示主菜单
- [x] 方向键精确移动一行
- [x] Tab 键可以切换焦点
- [x] 无界面残留
- [x] 测试通过

## 变更摘要

- 移除所有 List/HighlightSymbol/ListState 使用
- 统一使用 Paragraph + 手动前缀 (">> ") 方式
- 简化 widgets import
