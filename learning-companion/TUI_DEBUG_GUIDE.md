# TUI 调试指南

## 问题 1: 按键无响应

### 可能原因
1. 按键未被 `handle_key()` 捕获
2. 状态不匹配（在错误的状态处理按键）
3. 焦点区域错误（如在 ModuleFocus::Action 时按上下键）

### 调试步骤

#### 1. 添加按键日志
在 `tui.rs` 的 `handle_key()` 函数开头添加：

```rust
pub fn handle_key(&mut self, key: KeyCode) -> Result<()> {
    // 🔍 调试：记录所有按键
    eprintln!("[DEBUG] 按键: {:?}, 当前状态: {:?}", key, self.state);

    // 清除之前的消息（除了某些特定按键）
    if !matches!(key, KeyCode::Char('o') | KeyCode::Char('O')) {
        self.message = None;
        self.message_deadline = None;
    }

    match self.state {
        // ... 现有代码
    }
}
```

#### 2. 添加状态转换日志
在每个状态处理函数开头添加：

```rust
fn handle_dashboard_key(&mut self, key: KeyCode) {
    eprintln!("[DEBUG] Dashboard 处理按键: {:?}", key);

    // ... 现有代码
}
```

#### 3. 检查未处理的按键
在 `match` 语句末尾添加：

```rust
fn handle_dashboard_key(&mut self, key: KeyCode) {
    match key {
        KeyCode::Up => { /* ... */ }
        KeyCode::Down => { /* ... */ }
        // ... 其他 case
        _ => {
            eprintln!("[DEBUG] Dashboard 未处理的按键: {:?}", key);
        }
    }
}
```

#### 4. 验证事件读取
在 `run_tui()` 的事件循环中添加：

```rust
if event::poll(Duration::from_millis(100))? {
    if let Event::Key(key) = event::read()? {
        eprintln!("[DEBUG] 原始事件: {:?}", key);
        if key.kind == KeyEventKind::Press {
            app.handle_key(key.code)?;
        }
    }
}
```

### 快速测试脚本
创建 `test_keys.sh` 或手动测试：

```bash
# 启动 TUI
cargo run -- --path ..

# 测试所有按键并查看日志输出
# 记录哪些按键有响应，哪些没有
```

---

## 问题 2: CLI 格式消息混入

### 可能原因
1. 业务逻辑层使用了 `println!`、`eprintln!`
2. `show_encouragement()` 等函数直接打印
3. `progress.rs` 中的状态更新函数打印到 stdout

### 调试步骤

#### 1. 搜索所有 println
```bash
# 在 learning-companion 目录下搜索
grep -r "println!" src/
grep -r "eprintln!" src/
```

#### 2. 修复方法

**在 tui.rs 中替换所有 println**：

```rust
// ❌ 错误：直接打印会破坏 TUI
println!("✅ 已更新进度");

// ✅ 正确：使用消息系统
self.show_message("✅ 已更新进度".to_string());
```

**在 progress.rs 中移除打印**：

```rust
// ❌ 错误
pub fn update_task_status(repo: &LearningRepo, module_id: &str, task_str: &str) -> Result<()> {
    println!("✓ 标记 {} 的 {} 为已完成", module_id, task_type.as_str());
    // ...
}

// ✅ 正确：只返回结果
pub fn update_task_status(repo: &LearningRepo, module_id: &str, task_str: &str) -> Result<String> {
    let message = format!("✓ 标记 {} 的 {} 为已完成", module_id, task_type.as_str());
    // ...
    Ok(message)
}
```

#### 3. 修改 TUI 调用

```rust
// tui.rs 中的 UpdateProgressConfirm 状态处理
if *confirmed {
    if let Some(repo) = &self.repo {
        if let Some(module) = repo.modules.get(*selected_module) {
            let task_names = ["concept", "examples", "exercises", "project", "checklist"];
            let task = task_names.get(*selected_task).unwrap_or(&"concept");

            // 使用返回的消息
            let msg = crate::progress::update_task_status(repo, &module.id, task)?;
            self.show_message(msg);
        }
    }
}
```

---

## 问题 3: 消息不擦除

### 可能原因
1. `message_deadline` 未生效
2. `draw()` 函数中的超时检查逻辑错误
3. 消息绘制在错误位置

### 调试步骤

#### 1. 检查超时逻辑
在 `run_tui()` 的主循环中验证：

```rust
loop {
    // 🔍 调试：检查消息状态
    if let Some(msg) = &app.message {
        if let Some(deadline) = app.message_deadline {
            let remaining = deadline.saturating_duration_since(Instant::now());
            eprintln!("[DEBUG] 消息剩余时间: {:?}", remaining);
        }
    }

    // 检查消息超时并自动清除
    if let Some(deadline) = app.message_deadline {
        if Instant::now() >= deadline {
            eprintln!("[DEBUG] 消息超时，清除");
            app.message = None;
            app.message_deadline = None;
        }
    }

    // 绘制界面
    terminal.draw(|f| ui(f, &mut app))?;

    // ... 其余代码
}
```

#### 2. 检查绘制逻辑
在 `ui()` 函数中：

```rust
// 🔍 调试：记录绘制状态
fn ui(f: &mut Frame, app: &mut App) {
    let size = f.size();
    eprintln!("[DEBUG] 绘制界面，状态: {:?}, 消息: {:?}", app.state, app.message);

    // ... 绘制代码
}
```

#### 3. 修复超时检查
确保超时检查在每次绘制前执行：

```rust
// 在 run_tui() 中
loop {
    // 每次循环都检查超时
    if let Some(deadline) = app.message_deadline {
        if Instant::now() >= deadline {
            app.message = None;
            app.message_deadline = None;
        }
    }

    terminal.draw(|f| ui(f, &mut app))?;
    // ...
}
```

---

## 问题 4: 返回上级逻辑错误

### 可能原因
1. 状态栈（`state_stack`）管理混乱
2. 某些状态没有正确压入栈
3. 直接设置 state 而不使用栈

### 调试步骤

#### 1. 添加状态栈日志
在所有状态切换的地方添加：

```rust
impl App {
    fn push_state(&mut self, new_state: AppState) {
        eprintln!("[DEBUG] 压入状态: {:?}, 栈深度: {}", new_state, self.state_stack.len());
        self.state_stack.push(self.state.clone());
        self.state = new_state;
        self.message = None;
        self.message_deadline = None;
        self.update_help_text();
    }

    fn pop_state(&mut self) {
        eprintln!("[DEBUG] 弹出状态，栈深度: {}", self.state_stack.len());
        if let Some(prev_state) = self.state_stack.pop() {
            eprintln!("[DEBUG] 恢复状态: {:?}", prev_state);
            self.state = prev_state;
            self.message = None;
            self.message_deadline = None;
            self.update_help_text();
        } else {
            eprintln!("[DEBUG] 状态栈为空！");
        }
    }
}
```

#### 2. 检查所有状态切换
搜索所有 `self.state = ...` 的地方：

```bash
grep -n "self.state = " src/tui.rs
```

#### 3. 统一使用状态栈

**问题代码示例**：

```rust
// ❌ 直接设置状态，无法返回
KeyCode::Esc => {
    self.state = AppState::Dashboard { selected_module: 0 };
}
```

**修复后**：

```rust
// ✅ 使用状态栈
KeyCode::Esc => {
    if let Some(prev) = self.state_stack.pop() {
        self.state = prev;
    } else {
        self.state = AppState::MainMenu;
    }
    self.update_help_text();
}
```

#### 4. 状态迁移图
绘制当前的状态迁移图：

```
MainMenu
    ├─ Enter → Dashboard
    │         ├─ Esc → MainMenu ✓
    │         ├─ Enter/U → UpdateProgress
    │         │             ├─ Esc → ModuleDetail ❓ (应该是 Dashboard)
    │         │             └─ Enter → UpdateProgressConfirm
    │         │                         └─ Esc → UpdateProgress
    │         └─ O → ModuleDetail
    │                 └─ Esc → Dashboard ✓
    ...
```

---

## 通用调试技巧

### 1. 使用条件编译
```rust
#[cfg(debug_assertions)]
const DEBUG: bool = true;

#[cfg(not(debug_assertions))]
const DEBUG: bool = false;

if DEBUG {
    eprintln!("[DEBUG] ...");
}
```

### 2. 创建调试宏
```rust
macro_rules! debug_log {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            eprintln!("[DEBUG] {}", format!($($arg)*));
        }
    }
}

// 使用
debug_log!("当前状态: {:?}", self.state);
```

### 3. 可视化状态
在 TUI 界面显示当前状态（仅调试模式）：

```rust
fn draw_debug_info(f: &mut Frame, app: &App) {
    if cfg!(debug_assertions) {
        let debug_text = vec![
            Line::from(format!("State: {:?}", app.state)),
            Line::from(format!("Stack depth: {}", app.state_stack.len())),
            Line::from(format!("Message: {:?}", app.message)),
        ];

        let paragraph = Paragraph::new(debug_text)
            .block(Block::default().borders(Borders::ALL).title("DEBUG"));

        let area = Rect {
            x: size.width - 30,
            y: 0,
            width: 30,
            height: 5,
        };
        f.render_widget(paragraph, area);
    }
}
```

### 4. 最小化测试
创建简单的测试 TUI：

```rust
// 在 tui.rs 末尾添加
#[cfg(test)]
mod test_ui {
    use super::*;

    #[test]
    fn test_state_stack() {
        let mut app = App::new(".".to_string());
        app.push_state(AppState::Dashboard { selected_module: 0 });
        assert_eq!(app.state_stack.len(), 1);
        app.pop_state();
        assert_eq!(app.state, AppState::MainMenu);
    }

    #[test]
    fn test_message_timeout() {
        let mut app = App::new(".".to_string());
        app.show_message("test".to_string());
        assert!(app.message.is_some());
        // 等待 4 秒后
        std::thread::sleep(std::time::Duration::from_secs(4));
        // 模拟主循环检查
        // ...
    }
}
```

---

## 自动化检测脚本

创建 `check_tui.sh`:

```bash
#!/bin/bash

echo "=== TUI 问题检测 ==="

# 1. 检查 println! 使用
echo "1. 检查 println! 使用（应该在 TUI 中避免）"
grep -n "println!" src/tui.rs | grep -v "//" && echo "⚠️  发现 println!"
echo ""

# 2. 检查直接状态赋值
echo "2. 检查直接状态赋值（可能需要改用状态栈）"
grep -n "self.state = AppState::" src/tui.rs | head -20
echo ""

# 3. 检查未处理按键
echo "3. 检查状态处理函数是否有默认分支"
for func in handle_main_menu_key handle_dashboard_key handle_module_detail_key; do
    grep -A 5 "fn $func" src/tui.rs | grep "_ =>" || echo "✅ $func 有默认处理"
done
echo ""

# 4. 统计状态数量
echo "4. 状态转换复杂度"
echo "AppState 变体数量: $(grep -c "AppState::" src/tui.rs)"
echo ""

echo "=== 检测完成 ==="
```

运行：
```bash
chmod +x check_tui.sh
./check_tui.sh
```

---

## 下一步

1. **添加日志** - 按照 1-4 的步骤添加调试日志
2. **运行测试** - 记录问题触发时的日志
3. **定位问题** - 根据日志确定具体原因
4. **修复代码** - 按照修复方法修改
5. **验证修复** - 确认问题解决且不引入新问题

需要我帮你直接修复这些问题吗？
