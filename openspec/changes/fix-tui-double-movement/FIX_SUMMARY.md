# TUI 导航双步移动问题修复总结

## 问题根源
在 Windows 平台上，crossterm 库会为每个按键生成两个事件：
1. `KeyEventKind::Press` - 按键按下事件
2. `KeyEventKind::Release` - 按键释放事件

原代码没有过滤事件类型，导致每个按键都被处理了两次，造成方向键和 Tab 键的双步移动问题。

## 解决方案
在事件处理循环中添加事件类型过滤，只处理按键按下事件：

```rust
// 只处理按键按下事件，忽略按键释放事件（Windows 会报告两种事件）
if key.kind == KeyEventKind::Press {
    app.handle_key(key.code)?;
}
```

## 修改的文件
- `learning-companion/src/tui.rs`
  - 添加 `KeyEventKind` 导入
  - 在主循环中添加事件类型检查

## 影响
- 修复了 Windows 上的双步移动问题
- 不影响其他平台（macOS、Linux）的正常运行
- 保持了原有功能和性能

## 测试建议
运行应用并测试以下功能：
1. 主菜单的方向键导航
2. 仪表板的模块选择
3. 更新进度界面的 Tab 切换
4. 模块详情的任务选择和 Tab 切换

所有导航操作现在应该只移动一步，而不是两步。