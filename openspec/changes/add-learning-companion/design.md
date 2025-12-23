# 学习伴侣应用设计文档

## Context

为中文 Rust 学习项目创建一个桌面伴侣应用，解决学习追踪和激励问题。目标用户是自学 Rust 的中文开发者。

## Goals / Non-Goals

### Goals
- 提供友好的学习进度可视化
- 自动或半自动追踪学习进度
- 智能练习和评估系统
- 及时提醒和激励机制
- 支持本项目的仓库结构

### Non-Goals
- 不托管学习内容（内容在主仓库）
- 不提供在线社交功能（第一阶段）
- 不支持其他编程语言（专注于 Rust）

## Decisions

### 1. 技术栈：Tauri + React

**选择理由**：
- Tauri 使用 Rust 后端，可复用主项目代码
- 包体积小（相比 Electron）
- 原生系统通知支持
- 跨平台（Windows/macOS/Linux）

**替代方案**：
- Electron：太大，与 Rust 项目风格不符
- 纯 CLI：缺乏可视化界面
- VS Code 插件：局限于编辑器内

### 2. 进度追踪方式：半自动 + 手动补充

**实现**：
- 自动解析：`进度.md`、模块目录结构、测试通过情况
- 手动补充：用户标记练习完成状态、自检打分
- 持久化：本地 SQLite 数据库存储历史

**理由**：完全自动追踪难以准确判断学习质量，手动补充保证准确性

### 3. 掌握程度计算：多维度加权

**计算公式**：
```
总得分 = (练习完成率 × 30%) + (测试通过率 × 30%) + (自检打分 × 20%) + (综合练习 × 20%)
```

**阈值设定**：
- ≥95%：解锁下一模块
- 80-94%：建议复习
- <80%：强制复习

### 4. 练习系统：题库 + AI 生成

**阶段一（MVP）**：预定义题库
- 从每个模块的 `exercises.md` 和 `tests/` 提取题目
- 支持代码填空、选择题、判断题

**阶段二**：AI 辅助生成
- 分析模块内容生成新题
- 根据薄弱点针对性出题

### 5. 激励机制：成就 + 连续学习

**成就系统**：
- 首次完成模块
- 连续学习 7/30/100 天
- 完美通过测试（100%）
- 代码质量大师（clippy 零警告）

**连续学习追踪**：
- 每天有学习活动即 +1 天
- 中断则从 0 重新计算
- 显示当前连续天数和历史最高

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Tauri Desktop App                     │
├─────────────────────────────────────────────────────────┤
│  Frontend (React)                    Backend (Rust)     │
│  ┌─────────────┐                    ┌──────────────┐    │
│  │  Progress   │◄──────────────────►│  Repository  │    │
│  │   View      │    JSON/Events     │   Scanner    │    │
│  └─────────────┘                    └──────────────┘    │
│  ┌─────────────┐                    ┌──────────────┐    │
│  │   Practice  │◄──────────────────►│   Exercise   │    │
│  │   Module    │    Questions/       │   Generator  │    │
│  └─────────────┘    Answers         └──────────────┘    │
│  ┌─────────────┐                    ┌──────────────┐    │
│  │  Dashboard  │◄──────────────────►│    Stats     │    │
│  └─────────────┘    Analytics        │   Engine     │    │
│  ┌─────────────┐                    ┌──────────────┐    │
│  │  Settings   │◄──────────────────►│    Local     │    │
│  └─────────────┘                    │    Storage   │    │
│                                       │   (SQLite)   │    │
│  ┌─────────────┐                    └──────────────┘    │
│  │  Notifier   │◄──────┐                                   │
│  └─────────────┘       │ System Notify                   │
└────────────────────────┴─────────────────────────────────┘
                              │
                              ▼
                        ┌───────────┐
                        │Learn Rust │
                        │  Repo     │
                        └───────────┘
```

## Data Models

### Module Progress
```rust
struct ModuleProgress {
    id: String,              // "module-01-basics"
    name: String,            // "基础入门"
    status: CompletionStatus,
    started_at: Option<DateTime>,
    completed_at: Option<DateTime>,
    mastery_score: f32,      // 0.0 - 1.0
    tasks: Vec<TaskStatus>,
    practice_results: Vec<PracticeResult>,
}
```

### Practice Result
```rust
struct PracticeResult {
    id: String,
    module_id: String,
    timestamp: DateTime,
    questions_total: u32,
    questions_correct: u32,
    score: f32,
    weak_topics: Vec<String>,
}
```

### Study Session
```rust
struct StudySession {
    date: NaiveDate,
    duration_minutes: u32,
    modules_studied: Vec<String>,
    practices_completed: u32,
    notes: String,
}
```

## File Structure

```
learning-companion/
├── src-tauri/           # Rust backend
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── repo.rs       # 仓库扫描和解析
│   │   ├── progress.rs   # 进度计算
│   │   ├── exercise.rs   # 练习题生成
│   │   ├── storage.rs    # SQLite 持久化
│   │   └── notify.rs     # 系统通知
├── src/                 # React frontend
│   ├── pages/
│   │   ├── Dashboard.tsx
│   │   ├── Progress.tsx
│   │   ├── Practice.tsx
│   │   └── Settings.tsx
│   ├── components/
│   └── lib/
├── package.json
└── tauri.conf.json
```

## Risks / Trade-offs

### Risk 1：进度解析准确性

**风险**：不同仓库结构可能无法正确解析

**缓解措施**：
- 定义标准仓库结构规范
- 提供手动配置映射规则
- 允许手动修正解析结果

### Risk 2：用户隐私

**风险**：本地存储学习数据可能泄露

**缓解措施**：
- 数据仅存储本地，不上传
- 提供数据导出/删除功能
- 开源代码可审计

### Trade-off 1：提醒频率

**选项**：
- 固定时间提醒：简单但不灵活
- 智能提醒：复杂但更人性

**决策**：支持两者，默认固定时间，可选智能模式（基于学习历史）

## Migration Plan

无需迁移现有数据。应用首次启动时：
1. 扫描当前仓库结构
2. 读取 `进度.md` 现有数据
3. 初始化本地数据库
4. 用户可手动补充历史数据

## Open Questions

1. 是否需要支持多个学习仓库？
   - 暂不支持（MVP），后续可扩展

2. 练习题型是否需要代码运行验证？
   - MVP 仅支持选择题/判断题
   - 后续集成 Cargo 运行验证

3. 是否需要云端同步？
   - 暂不需要，保持本地
   - 后续可提供可选云同步

4. 多语言支持？
   - MVP 仅中文
   - 架构支持后续 i18n
