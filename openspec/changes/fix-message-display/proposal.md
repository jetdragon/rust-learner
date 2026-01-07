# Proposal: Fix Message Display Issues in TUI

## Status
**Proposed** | 2025-01-07

## Context
在"确认更新进度"对话框中选择"是"后，显示的成功消息存在以下问题：

1. **文字溢出边框**：消息文字过长时（如"✅ 已更新 模块01-基础入门 的 概念学习 任务"）会超出底部消息框的边界
2. **消息不会自动消失**：消息显示后会一直保留，需要用户按任意键才能清除，影响后续操作
3. **换行问题**：`draw_message` 函数使用 `Line::from()` 不支持文字自动换行

## Current Implementation

消息显示在 `learning-companion/src/tui.rs` 中：

```rust
// 行 532-533: 设置消息
self.message = Some(format!("✅ 已更新 {} 的 {} 任务", module.name, task_name));

// 行 966-968: 绘制消息
if let Some(msg) = &app.message {
    draw_message(f, chunks[2], msg);
}

// 行 991-998: draw_message 函数
fn draw_message(f: &mut Frame, area: Rect, message: &str) {
    let msg = Paragraph::new(Line::from(message.to_string()))  // ❌ 不会换行
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Green))
        .block(Block::default().borders(Borders::ALL).title("提示"));
    f.render_widget(msg, area);
}
```

## Problems Analysis

### 1. 文字溢出
- `Line::from()` 将整个消息作为单行文本处理
- 没有设置 `wrap` 参数导致文字不换行
- 底部区域 (`chunks[2]`) 高度只有 3 行，可能不够容纳长消息

### 2. 消息持久显示
- 消息设置在 `self.message` 中后会一直存在
- 只有在下次按键时才会在 `handle_key` 开头清除
- 这意味着用户需要按一次任意键才能让消息消失

### 3. 用户体验问题
- 用户看到消息后，想继续操作时需要额外按键
- 长消息会破坏界面布局
- 没有超时自动清除机制

## Proposed Solution

### 方案 1: 自动消失的临时消息（推荐）
1. 为 `App` 结构添加 `message_timeout` 字段
2. 消息显示时设置超时时间（如 3 秒）
3. 在主循环中检查超时，自动清除消息
4. 修复 `draw_message` 支持文字换行

### 方案 2: 按键清除（当前行为改进）
1. 保持当前按键清除机制
2. 修复 `draw_message` 支持文字换行
3. 在帮助文本中提示"按任意键清除消息"

### 方案 3: 简化消息格式
1. 缩短消息文字，避免溢出
2. 使用代码而非全名显示模块

## Recommended Approach
**方案 1** - 自动消失的临时消息，提供最佳用户体验：
- 消息显示 2-3 秒后自动消失
- 支持文字自动换行
- 用户无需额外操作

## Scope
- **Files**: `learning-companion/src/tui.rs`
- **Changes**:
  1. 添加 `message_deadline: Option<Instant>` 字段到 `App` 结构
  2. 修改 `draw_message` 支持换行
  3. 在主循环中检查并清除超时消息
  4. 缩短消息格式或使用 `Wrap` 处理长文本

## Alternatives Considered
1. **使用 toast 弹窗**：实现复杂，需要额外的 UI 状态管理
2. **增加底部区域高度**：会压缩主内容区域，影响整体布局
3. **完全不显示消息**：用户不知道操作是否成功

## Related Changes
None - this is an independent improvement to the TUI UX.

## Acceptance Criteria
1. 长消息文字能正确换行，不溢出边框
2. 消息在 2-3 秒后自动消失
3. 用户可以立即继续操作，无需额外按键
4. 短消息（如"数据导出完成"）也正常显示
