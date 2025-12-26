# enhance-tui-navigation Tasks

## 任务清单

### Phase 1: 修复光标跳行问题

1. [x] 修复 draw_dashboard 使用 highlight_symbol 机制
   - [x] 移除手动添加的 "> > " 前缀
   - [x] 配置 List widget 的 highlight_symbol
   - [x] 验证光标移动精确到一行

2. [x] 测试仪表板导航
   - [x] 上下键移动测试
   - [x] 边界情况测试（首尾）

### Phase 2: 新增模块详情界面

3. [x] 新增 ModuleDetail 状态和 ModuleFocus 枚举
   - [x] 在 AppState 中添加 ModuleDetail 变体
   - [x] 定义 ModuleFocus 枚举 (TaskList, Action)

4. [x] 实现模块详情按键处理
   - [x] handle_module_detail_key 函数
   - [x] 支持 o 键打开文件
   - [x] 支持空格键标记完成
   - [x] 支持 Tab 切换焦点
   - [x] 支持 Esc 返回

5. [x] 实现 draw_module_detail 界面
   - [x] 显示模块信息
   - [x] 显示文件列表
   - [x] 显示任务状态和操作选项

6. [x] 实现 VSCode 打开功能
   - [x] 添加 open_in_vscode 函数
   - [x] 使用 `cmd::Command::new("code")` 打开文件

### Phase 3: 集成和测试

7. [x] 更新仪表板进入模块详情
   - [x] 修改 handle_dashboard_key 支持 O 进入详情
   - [x] 更新帮助文本

8. [x] 运行所有测试
   - [x] cargo test 确保无回归 (15/15 通过)
   - [x] 验证新功能正常工作

## 依赖关系

- Phase 1 必须先完成
- Phase 2 可以并行执行（任务 3, 4, 5, 6 顺序执行）
- Phase 3 在 Phase 2 完成后执行

## 验证标准

- [x] 光标移动精确到一行
- [x] 模块详情界面显示正确
- [x] VSCode 可以打开模块文件
- [x] 所有现有测试通过
