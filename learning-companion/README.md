# Rust 学习伴侣

一个帮助你追踪 Rust 学习进度、提供练习和激励的命令行工具。

## 功能

- 🖥️ **交互式 TUI** - 友好的终端用户界面，键盘导航
- 📊 **学习仪表板** - 可视化展示学习进度和统计
- ✅ **进度追踪** - 记录学习任务完成情况
- 📝 **智能练习** - 自动生成练习题并分析掌握程度
- ⏰ **定时提醒** - 系统通知提醒你学习
- 🏆 **成就系统** - 解锁成就保持学习动力
- 💾 **数据持久化** - 本地 SQLite 存储学习记录

## 安装

```bash
cd learning-companion
cargo build --release
```

## 使用

### 交互式 TUI 模式（推荐）

直接运行程序进入交互式终端界面：

```bash
# 默认启动 TUI 模式
cargo run

# 或使用 --tui 标志
cargo run -- --tui

# 或使用 -i 简写
cargo run -- -i
```

**TUI 按键操作：**
- `↑↓` - 在列表中移动光标
- `Enter` - 确认选择
- `Tab` - 在字段间切换
- `Esc` - 返回上级
- `q` - 退出程序

### CLI 命令模式

如果你喜欢传统命令行方式，仍然可以使用：

```bash
# 查看学习仪表板
cargo run -- dashboard

# 更新学习进度
cargo run -- update -m module-01-basics -t concept

# 开始练习测试
cargo run -- practice -m module-01-basics -c 5

# 设置学习提醒 (20:00)
cargo run -- remind -H 20 -M 0

# 查看成就
cargo run -- achievements

# 导出学习数据
cargo run -- export
```

### 支持的任务类型

- `concept` / 概念 - 概念学习
- `examples` / 示例 - 代码示例
- `exercises` / 练习 - 练习题
- `project` / 综合 - 综合练习
- `checklist` / 自检 - 自检通过

## 数据存储

学习数据存储在 `~/.learning-companion/data.db`（SQLite 格式）

## 进度计算

掌握程度计算公式：
```
总得分 = (练习完成率 × 30%) + (测试通过率 × 30%) + (自检打分 × 20%) + (综合练习 × 20%)
```

- **≥95%**：解锁下一模块
- **80-94%**：建议复习
- **<80%**：需要加强学习

## 开发

```bash
# 运行测试
cargo test

# 代码格式化
cargo fmt

# 代码检查
cargo clippy
```

## 未来计划

- [x] 交互式 TUI 模式
- [ ] Tauri 桌面应用版本
- [ ] 更多模块的练习题
- [ ] 学习统计图表
- [ ] AI 生成练习题
- [ ] 多仓库支持

## 许可证

MIT License
