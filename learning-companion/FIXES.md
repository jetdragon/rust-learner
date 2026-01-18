# TUI 问题修复方案

## 问题 1: 移除所有 println!（避免污染 TUI）

### 需要修改的文件

#### src/progress.rs - update_task_status 函数

**当前代码**:
```rust
pub fn update_task_status(repo: &LearningRepo, module_id: &str, task_str: &str) -> Result<()> {
    let task = TaskType::from_str(task_str);

    if let Some(task_type) = task {
        println!("✓ 标记 {} 的 {} 为已完成", module_id, task_type.as_str());
        // ...
        println!("📊 当前掌握程度：{:.1}%", new_score);

        if new_score >= 95.0 {
            println!("🎉 恭喜！你已掌握该模块，可以进入下一阶段学习！");
        } else if new_score >= 80.0 {
            println!("💪 做得不错！继续加油！");
        } else {
            println!("📚 继续学习，你可以的！");
        }

        return Ok(());
    }

    // ...
    println!("❌ 未知的任务类型：{}", task_str);
    println!("💡 支持的任务类型：概念、示例、练习、综合、自检");
    Err(anyhow::anyhow!("未知任务类型"))
}
```

**修复后代码**:
```rust
pub fn update_task_status(repo: &LearningRepo, module_id: &str, task_str: &str) -> Result<String> {
    let task = TaskType::from_str(task_str);

    if let Some(task_type) = task {
        // 更新数据库中的模块进度
        let increase = match task_type {
            TaskType::Concept => 15.0,
            TaskType::Examples => 15.0,
            TaskType::Exercises => 30.0,
            TaskType::Project => 30.0,
            TaskType::Checklist => 10.0,
        };

        // 获取当前进度并更新
        let current_score = crate::db::get_module_mastery(module_id).unwrap_or(0.0);
        let new_score = (current_score + increase).min(100.0);

        crate::db::update_module_progress(module_id, new_score)?;

        // 构建返回消息而不是直接打印
        let message = if new_score >= 95.0 {
            format!("✅ {} - 掌握 {:.1}%，已掌握该模块！🎉", task_type.as_str(), new_score)
        } else if new_score >= 80.0 {
            format!("✅ {} - 掌握 {:.1}%，做得不错！💪", task_type.as_str(), new_score)
        } else {
            format!("✅ {} - 掌握 {:.1}%，继续加油！📚", task_type.as_str(), new_score)
        };

        return Ok(message);
    }

    // 模糊匹配
    let task_lower = task_str.to_lowercase();
    if task_lower.contains("概念") || task_lower.contains("concept") {
        return update_task_status(repo, module_id, "concept");
    } else if task_lower.contains("示例") || task_lower.contains("example") {
        return update_task_status(repo, module_id, "examples");
    } else if task_lower.contains("练习") || task_lower.contains("exercise") {
        return update_task_status(repo, module_id, "exercises");
    } else if task_lower.contains("综合") || task_lower.contains("project") {
        return update_task_status(repo, module_id, "project");
    } else if task_lower.contains("自检") || task_lower.contains("checklist") {
        return update_task_status(repo, module_id, "checklist");
    }

    Err(anyhow::anyhow!("未知任务类型：{}", task_str))
}
```

#### src/tui.rs - UpdateProgressConfirm 处理

**当前代码**:
```rust
KeyCode::Enter => {
    if *confirmed {
        // 确认保存
        if let Some(repo) = &self.repo {
            if let Some(module) = repo.modules.get(*selected_module) {
                let task_names = ["concept", "examples", "exercises", "project", "checklist"];
                let task = task_names.get(*selected_task).unwrap_or(&"concept");
                let _ = crate::progress::update_task_status(repo, &module.id, task);
                let module_name = module.name.clone();
                let task_idx = *selected_task;
                self.show_message(format!("✅ 已更新 {} 的 {} 任务", module_name,
                    ["概念学习", "代码示例", "练习题", "综合练习", "自检"].get(task_idx).unwrap_or(&"")));
            }
        }
    }
    // 无论确认还是取消，都返回上级状态
    self.pop_state();
    self.update_help_text();
}
```

**修复后代码**:
```rust
KeyCode::Enter => {
    if *confirmed {
        // 确认保存
        if let Some(repo) = &self.repo {
            if let Some(module) = repo.modules.get(*selected_module) {
                let task_names = ["concept", "examples", "exercises", "project", "checklist"];
                let task = task_names.get(*selected_task).unwrap_or(&"concept");

                // 使用返回的消息
                match crate::progress::update_task_status(repo, &module.id, task) {
                    Ok(msg) => {
                        self.show_message(msg);
                    }
                    Err(e) => {
                        self.show_message(format!("❌ 更新失败: {}", e));
                    }
                }
            }
        }
    }
    // 无论确认还是取消，都返回上级状态
    self.pop_state();
    self.update_help_text();
}
```

---

## 问题 2: 修复返回上级菜单逻辑

### 需要修改的状态处理函数

#### UpdateProgress 返回逻辑

**当前代码** (可能的问题):
```rust
fn handle_update_progress_key(&mut self, key: KeyCode) {
    if let AppState::UpdateProgress { ref mut selected_module, ref mut selected_task, ref mut focus_area } = self.state {
        match key {
            KeyCode::Esc | KeyCode::Char('q') => {
                // 返回模块详情界面
                let module = *selected_module;
                let task = *selected_task;
                self.state = AppState::ModuleDetail {
                    selected_module: module,
                    selected_task: task,
                    focus_area: ModuleFocus::TaskList,
                };
                self.state_stack.clear(); // 清空状态栈，避免累积
                self.update_help_text();
            }
            // ...
        }
    }
}
```

**问题分析**:
- 使用了 `self.state =` 直接赋值
- `self.state_stack.clear()` 可能导致无法返回更上层

**修复后代码**:
```rust
fn handle_update_progress_key(&mut self, key: KeyCode) {
    if let AppState::UpdateProgress { ref mut selected_module, ref mut selected_task, ref mut focus_area } = self.state {
        match key {
            KeyCode::Esc | KeyCode::Char('q') => {
                // 使用 pop_state 返回上级
                self.pop_state();
                self.update_help_text();
            }
            // ... 其他按键处理保持不变
        }
    }
}
```

#### Dashboard 返回逻辑

**当前代码**:
```rust
KeyCode::Esc | KeyCode::Char('q') => {
    // 返回主菜单
    self.state = AppState::MainMenu;
    self.state_stack.clear();
    self.update_help_text();
}
```

**修复后代码**:
```rust
KeyCode::Esc | KeyCode::Char('q') => {
    // 返回主菜单
    if self.state_stack.is_empty() {
        self.state = AppState::MainMenu;
    } else {
        self.pop_state();
    }
    self.update_help_text();
}
```

---

## 问题 3: 确保消息超时清理

### 验证主循环逻辑

**当前代码应该已经是正确的**，检查确认：

```rust
pub fn run_tui(project_path: &str) -> Result<()> {
    // ... 初始化代码 ...

    // 主循环
    loop {
        // 🔍 关键：每次循环都检查消息超时
        if let Some(deadline) = app.message_deadline {
            if Instant::now() >= deadline {
                app.message = None;
                app.message_deadline = None;
            }
        }

        // 绘制界面
        terminal.draw(|f| ui(f, &mut app))?;

        // 检查是否应该退出
        if app.should_quit {
            break;
        }

        // 读取事件（超时 100ms）
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    app.handle_key(key.code)?;
                }
            }
        }
    }

    // ... 清理代码 ...
}
```

**如果代码不完整，添加超时检查**:
```rust
loop {
    // 检查消息超时并自动清除
    if let Some(deadline) = app.message_deadline {
        if Instant::now() >= deadline {
            app.message = None;
            app.message_deadline = None;
        }
    }

    // 绘制界面
    terminal.draw(|f| ui(f, &mut app))?;

    // ... 其余代码
}
```

---

## 问题 4: 按键无响应

### 常见原因和修复

#### 原因 1: 焦点区域错误

**ModuleDetail 状态的焦点切换**:

**检查代码**:
```rust
fn handle_module_detail_key(&mut self, key: KeyCode) {
    if let AppState::ModuleDetail { ref mut selected_module, ref mut selected_task, ref mut focus_area } = self.state {
        match key {
            KeyCode::Up => {
                if let Some(repo) = &self.repo {
                    match focus_area {
                        ModuleFocus::TaskList => {
                            if *selected_task > 0 {
                                *selected_task -= 1;
                            }
                        }
                        ModuleFocus::Action => {
                            if *selected_task > 0 {
                                *selected_task -= 1;
                            }
                        }
                    }
                }
            }
            // ... 其他按键
        }
    }
}
```

**问题**: `ModuleFocus::Action` 区域没有上下键响应（因为没有列表）

**修复**: 为 Action 区域添加适当的行为，或者移除焦点区域支持

```rust
match focus_area {
    ModuleFocus::TaskList => {
        if *selected_task > 0 {
            *selected_task -= 1;
        }
    }
    ModuleFocus::Action => {
        // Action 区域不需要上下键，不处理
        // 或者保持与 TaskList 同步
    }
}
```

#### 原因 2: 未处理的按键

**在所有状态处理函数末尾添加默认分支**:

```rust
fn handle_dashboard_key(&mut self, key: KeyCode) {
    if let AppState::Dashboard { ref mut selected_module } = self.state {
        match key {
            KeyCode::Esc | KeyCode::Char('q') => { /* ... */ }
            KeyCode::Up => { /* ... */ }
            KeyCode::Down => { /* ... */ }
            // ... 其他已处理的按键
            _ => {
                // 🔍 添加默认分支处理未识别的按键
                eprintln!("[DEBUG] Dashboard 未处理的按键: {:?}", key);
            }
        }
    }
}
```

#### 原因 3: 状态转换问题

**确保状态正确切换**:

在 `push_state()` 和 `pop_state()` 中添加验证：

```rust
fn push_state(&mut self, new_state: AppState) {
    eprintln!("[DEBUG] 压入状态: {:?} <- 当前: {:?}", new_state, self.state);
    self.state_stack.push(self.state.clone());
    self.state = new_state;
    self.message = None;
    self.message_deadline = None;
    self.update_help_text();
}

fn pop_state(&mut self) {
    if let Some(prev_state) = self.state_stack.pop() {
        eprintln!("[DEBUG] 弹出状态: {:?} -> 恢复: {:?}", self.state, prev_state);
        self.state = prev_state;
        self.message = None;
        self.message_deadline = None;
        self.update_help_text();
    } else {
        eprintln!("[DEBUG] 状态栈为空，返回主菜单");
        self.state = AppState::MainMenu;
        self.update_help_text();
    }
}
```

---

## 快速应用修复

### 方法 1: 手动修复（推荐用于学习）

按照上述修改逐个文件修复，理解每个问题。

### 方法 2: 使用补丁文件

创建 `fixes.patch` 并应用：

```bash
git apply fixes.patch
```

### 方法 3: 我可以帮你直接修复

如果你希望我直接修复这些问题，我需要确认：
1. 是否保留调试日志（`eprintln!`）？
2. 是否保留原始代码的注释？
3. 是否添加单元测试？

---

## 验证修复

修复后，使用以下步骤验证：

```bash
# 1. 重新编译
cd learning-companion
cargo build --release

# 2. 测试 TUI
cargo run --release -- --path ..

# 3. 测试以下场景：
#    - 主菜单导航（上下键）
#    - 进入 Dashboard 并返回
#    - 进入模块详情并返回
#    - 更新进度（检查消息是否显示并消失）
#    - 确认对话框（左右键切换选项）
#    - 进入子菜单并逐级返回

# 4. 检查日志（如果添加了调试输出）
#    正常运行应该没有控制台输出
#    只有调试时才看到 eprintln! 的内容
```

---

## 修复后的代码统计

预期修改:
- `src/progress.rs`: 约 20 行修改
- `src/tui.rs`: 约 50 行修改（包括调试日志）

预期改进:
- ✅ 消除所有 `println!` 对 TUI 的污染
- ✅ 修复返回上级菜单的逻辑错误
- ✅ 确保消息正确显示和自动消失
- ✅ 添加调试日志便于后续问题定位

---

需要我直接应用这些修复吗？
