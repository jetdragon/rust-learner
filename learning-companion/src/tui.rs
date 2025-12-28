//! TUI 模块
//!
//! 交互式终端用户界面实现

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};
use std::io;
use std::time::Duration;

// 导入项目模块
use crate::repo::{LearningRepo, ModuleProgress};

/// 应用状态
#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    MainMenu,
    Dashboard { selected_module: usize },
    ModuleDetail { selected_module: usize, selected_task: usize, focus_area: ModuleFocus },
    UpdateProgress { selected_module: usize, selected_task: usize, focus_area: FocusArea },
    UpdateProgressConfirm { selected_module: usize, selected_task: usize, confirmed: bool },
    Practice { selected_module: usize, question_count: usize, focus_field: PracticeField },
    PracticeSession { session: PracticeSession },
    Achievements,
    RemindSetup { hour: u8, minute: u8, focus_field: TimeField },
    Export,
    FileViewer {
        file_path: String,
        content: String,
        scroll_offset: usize,
        return_state: Box<AppState>, // 保存返回的状态
    },
}

/// 焦点区域（用于 UpdateProgress）
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FocusArea {
    ModuleList,
    TaskList,
}

/// 时间字段焦点（用于 RemindSetup）
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimeField {
    Hour,
    Minute,
}

/// 练习界面字段焦点
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PracticeField {
    Module,
    Count,
}

/// 模块详情焦点区域
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModuleFocus {
    TaskList,
    Action,
}

/// 练习会话状态
#[derive(Debug, Clone, PartialEq)]
pub struct PracticeSession {
    pub questions: Vec<crate::exercise::Question>,
    pub current_index: usize,
    pub answers: Vec<Option<usize>>,
    pub show_result: bool,
}

/// 主应用结构
pub struct App {
    /// 当前状态
    pub state: AppState,
    /// 状态栈，用于返回上级
    pub state_stack: Vec<AppState>,
    /// 主菜单选中项
    pub main_menu_selected: usize,
    /// 主菜单选项
    pub main_menu_items: Vec<String>,
    /// 标题
    pub title: String,
    /// 帮助提示
    pub help_text: String,
    /// 仓库数据（延迟加载）
    pub repo: Option<LearningRepo>,
    /// 消息提示
    pub message: Option<String>,
    /// 是否应该退出
    pub should_quit: bool,
    /// 项目路径
    pub project_path: String,
}

impl App {
    /// 创建新应用
    pub fn new(project_path: String) -> Self {
        let main_menu_items = vec![
            "📊 查看学习仪表板".to_string(),
            "📚 更新学习进度".to_string(),
            "✏️  开始练习测试".to_string(),
            "🏆 查看成就".to_string(),
            "⏰ 设置学习提醒".to_string(),
            "📤 导出学习数据".to_string(),
            "退出程序".to_string(),
        ];

        Self {
            state: AppState::MainMenu,
            state_stack: Vec::new(),
            main_menu_selected: 0,
            main_menu_items,
            title: "🦀 Rust 学习伴侣".to_string(),
            help_text: "↑↓ 移动 | Enter 确认 | q 退出".to_string(),
            repo: None,
            message: None,
            should_quit: false,
            project_path,
        }
    }

    /// 确保仓库已加载
    fn ensure_repo(&mut self) -> Result<()> {
        if self.repo.is_none() {
            self.repo = Some(LearningRepo::new(&self.project_path)?);
        }
        Ok(())
    }

    /// 处理按键事件
    pub fn handle_key(&mut self, key: KeyCode) -> Result<()> {
        // 清除之前的消息（除了某些特定按键）
        if !matches!(key, KeyCode::Char('o') | KeyCode::Char('O')) {
            self.message = None;
        }

        match self.state {
            AppState::MainMenu => self.handle_main_menu_key(key),
            AppState::Dashboard { .. } => self.handle_dashboard_key(key),
            AppState::ModuleDetail { .. } => self.handle_module_detail_key(key),
            AppState::UpdateProgress { .. } => self.handle_update_progress_key(key),
            AppState::UpdateProgressConfirm { .. } => self.handle_update_progress_confirm_key(key),
            AppState::Practice { .. } => self.handle_practice_key(key),
            AppState::Achievements => self.handle_achievements_key(key),
            AppState::RemindSetup { .. } => self.handle_remind_setup_key(key),
            AppState::Export => self.handle_export_key(key),
            AppState::PracticeSession { .. } => self.handle_practice_session_key(key),
            AppState::FileViewer { .. } => self.handle_file_viewer_key(key),
        }
        Ok(())
    }

    /// 主菜单按键处理
    fn handle_main_menu_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up => {
                if self.main_menu_selected > 0 {
                    self.main_menu_selected -= 1;
                }
            }
            KeyCode::Down => {
                if self.main_menu_selected < self.main_menu_items.len() - 1 {
                    self.main_menu_selected += 1;
                }
            }
            KeyCode::Enter => {
                if let Err(e) = self.enter_main_menu_selection() {
                    self.message = Some(format!("错误: {}", e));
                }
            }
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    /// 进入主菜单选中的项
    fn enter_main_menu_selection(&mut self) -> Result<()> {
        match self.main_menu_selected {
            0 => {
                self.ensure_repo()?;
                self.push_state(AppState::Dashboard { selected_module: 0 });
            }
            1 => {
                self.ensure_repo()?;
                self.push_state(AppState::UpdateProgress {
                    selected_module: 0,
                    selected_task: 0,
                    focus_area: FocusArea::ModuleList,
                });
            }
            2 => {
                self.ensure_repo()?;
                self.push_state(AppState::Practice {
                    selected_module: 0,
                    question_count: 5,
                    focus_field: PracticeField::Module,
                });
            }
            3 => {
                self.push_state(AppState::Achievements);
            }
            4 => {
                self.push_state(AppState::RemindSetup {
                    hour: 20,
                    minute: 0,
                    focus_field: TimeField::Hour,
                });
            }
            5 => {
                self.push_state(AppState::Export);
            }
            6 => self.should_quit = true,
            _ => {}
        }
        Ok(())
    }

    /// 仪表板按键处理
    fn handle_dashboard_key(&mut self, key: KeyCode) {
        if let AppState::Dashboard { ref mut selected_module } = self.state {
            match key {
                KeyCode::Esc | KeyCode::Char('q') => {
                    // 返回主菜单
                    self.state = AppState::MainMenu;
                    self.state_stack.clear();
                    self.update_help_text();
                }
                KeyCode::Up => {
                    if *selected_module > 0 {
                        *selected_module -= 1;
                    }
                }
                KeyCode::Down => {
                    if let Some(repo) = &self.repo {
                        if *selected_module < repo.modules.len().saturating_sub(1) {
                            *selected_module += 1;
                        }
                    }
                }
                KeyCode::Enter | KeyCode::Char('u') | KeyCode::Char('U') => {
                    // 进入更新进度界面，选中的模块
                    let module = *selected_module;
                    self.state = AppState::UpdateProgress {
                        selected_module: module,
                        selected_task: 0,
                        focus_area: FocusArea::ModuleList,
                    };
                    self.update_help_text();
                }
                KeyCode::Char('p') | KeyCode::Char('P') => {
                    // 进入练习界面
                    let module = *selected_module;
                    self.state = AppState::Practice {
                        selected_module: module,
                        question_count: 5,
                        focus_field: PracticeField::Module,
                    };
                    self.update_help_text();
                }
                KeyCode::Char('a') | KeyCode::Char('A') => {
                    // 查看成就
                    self.push_state(AppState::Achievements);
                }
                KeyCode::Char('o') | KeyCode::Char('O') => {
                    // 进入模块详情界面
                    let module = *selected_module;
                    self.state = AppState::ModuleDetail {
                        selected_module: module,
                        selected_task: 0,
                        focus_area: ModuleFocus::TaskList,
                    };
                    self.update_help_text();
                }
                _ => {}
            }
        }
    }

    /// 模块详情按键处理
    fn handle_module_detail_key(&mut self, key: KeyCode) {
        if let AppState::ModuleDetail { ref mut selected_module, ref mut selected_task, ref mut focus_area } = self.state {
            match key {
                KeyCode::Esc | KeyCode::Char('q') => {
                    // 返回仪表板
                    if let Some(repo) = &self.repo {
                        let module = *selected_module;
                        self.state = AppState::Dashboard { selected_module: module };
                    } else {
                        self.state = AppState::MainMenu;
                    }
                    self.update_help_text();
                }
                KeyCode::Tab => {
                    *focus_area = match focus_area {
                        ModuleFocus::TaskList => ModuleFocus::Action,
                        ModuleFocus::Action => ModuleFocus::TaskList,
                    };
                }
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
                KeyCode::Down => {
                    if let Some(repo) = &self.repo {
                        match focus_area {
                            ModuleFocus::TaskList => {
                                if *selected_task < 4 {
                                    *selected_task += 1;
                                }
                            }
                            ModuleFocus::Action => {
                                if *selected_task < 4 {
                                    *selected_task += 1;
                                }
                            }
                        }
                    }
                }
                KeyCode::Char('o') | KeyCode::Char('O') => {
                    // 在TUI内打开当前任务对应的文件
                    if let Some(repo) = &self.repo {
                        if let Some(module) = repo.modules.get(*selected_module) {
                            let task_files = ["README.md", "examples", "exercises.md", "tests", "自检清单.md"];
                            if let Some(file) = task_files.get(*selected_task) {
                                let path = module.directory.join(file);

                                // 保存当前状态以便返回
                                let current_state = self.state.clone();

                                // 尝试读取文件内容
                                match std::fs::read_to_string(&path) {
                                    Ok(content) => {
                                        // 切换到文件查看器状态
                                        self.state = AppState::FileViewer {
                                            file_path: file.to_string(),
                                            content,
                                            scroll_offset: 0,
                                            return_state: Box::new(current_state),
                                        };
                                        self.update_help_text();
                                    }
                                    Err(_) => {
                                        // 如果是目录，显示目录内容
                                        if path.is_dir() {
                                            let mut dir_content = String::new();
                                            dir_content.push_str(&format!("目录: {}\n\n", path.display()));

                                            if let Ok(entries) = std::fs::read_dir(&path) {
                                                for entry in entries.flatten() {
                                                    let name = entry.file_name().to_string_lossy().to_string();
                                                    let file_type = if entry.path().is_dir() { "[DIR]" } else { "[FILE]" };
                                                    dir_content.push_str(&format!("{} {}\n", file_type, name));
                                                }
                                            }

                                            self.state = AppState::FileViewer {
                                                file_path: format!("{}/ (目录)", file),
                                                content: dir_content,
                                                scroll_offset: 0,
                                                return_state: Box::new(current_state),
                                            };
                                            self.update_help_text();
                                        } else {
                                            self.message = Some(format!("❌ 无法读取文件: {}", file));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                KeyCode::Char(' ') | KeyCode::Enter => {
                    // 进入确认状态，而不是直接标记完成
                    if let Some(repo) = &self.repo {
                        if let Some(_module) = repo.modules.get(*selected_module) {
                            // 提取当前状态值，避免借用冲突
                            let module_idx = *selected_module;
                            let task_idx = *selected_task;

                            // 保存当前状态并切换到确认状态
                            let current_state = self.state.clone();
                            self.state = AppState::UpdateProgressConfirm {
                                selected_module: module_idx,
                                selected_task: task_idx,
                                confirmed: false,
                            };
                            self.state_stack.push(current_state);
                            self.update_help_text();
                        }
                    }
                }
                _ => {}
            }
        }
    }

    /// 使用 VSCode 打开文件或目录
    fn open_in_vscode(&self, path: &std::path::Path) -> Result<()> {
        let path_str = path.to_string_lossy();
        std::process::Command::new("code")
            .arg("-r")
            .arg(&*path_str)
            .spawn()
            .map(|_| ())
            .map_err(|e| anyhow::anyhow!("Failed to open VSCode: {}", e))
    }

    /// 更新进度按键处理
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
                KeyCode::Tab | KeyCode::Char(' ') => {
                    *focus_area = match focus_area {
                        FocusArea::ModuleList => FocusArea::TaskList,
                        FocusArea::TaskList => FocusArea::ModuleList,
                    };
                }
                KeyCode::Up => {
                    if let Some(repo) = &self.repo {
                        match focus_area {
                            FocusArea::ModuleList => {
                                if *selected_module > 0 {
                                    *selected_module -= 1;
                                    *selected_task = 0;
                                }
                            }
                            FocusArea::TaskList => {
                                if *selected_task > 0 {
                                    *selected_task -= 1;
                                }
                            }
                        }
                    }
                }
                KeyCode::Down => {
                    if let Some(repo) = &self.repo {
                        match focus_area {
                            FocusArea::ModuleList => {
                                if *selected_module < repo.modules.len().saturating_sub(1) {
                                    *selected_module += 1;
                                    *selected_task = 0;
                                }
                            }
                            FocusArea::TaskList => {
                                if *selected_task < 4 {
                                    *selected_task += 1;
                                }
                            }
                        }
                    }
                }
                KeyCode::Enter => {
                    // 进入确认界面
                    let module = *selected_module;
                    let task = *selected_task;
                    self.state = AppState::UpdateProgressConfirm {
                        selected_module: module,
                        selected_task: task,
                        confirmed: false,
                    };
                    self.update_help_text();
                }
                _ => {}
            }
        }
    }

    /// 更新进度确认按键处理
    fn handle_update_progress_confirm_key(&mut self, key: KeyCode) {
        if let AppState::UpdateProgressConfirm { ref mut selected_module, ref mut selected_task, ref mut confirmed } = self.state {
            match key {
                KeyCode::Esc | KeyCode::Char('q') => {
                    // 返回上级状态（UpdateProgress）
                    self.pop_state();
                    self.update_help_text();
                }
                KeyCode::Left => {
                    *confirmed = false;
                }
                KeyCode::Right | KeyCode::Tab | KeyCode::Char(' ') => {
                    *confirmed = true;
                }
                KeyCode::Enter => {
                    if *confirmed {
                        // 确认保存
                        if let Some(repo) = &self.repo {
                            if let Some(module) = repo.modules.get(*selected_module) {
                                let task_names = ["concept", "examples", "exercises", "project", "checklist"];
                                let task = task_names.get(*selected_task).unwrap_or(&"concept");
                                let _ = crate::progress::update_task_status(repo, &module.id, task);
                                self.message = Some(format!("✅ 已更新 {} 的 {} 任务", module.name,
                                    ["概念学习", "代码示例", "练习题", "综合练习", "自检"].get(*selected_task).unwrap_or(&"")));
                            }
                        }
                    }
                    // 无论确认还是取消，都返回上级状态
                    self.pop_state();
                    self.update_help_text();
                }
                _ => {}
            }
        }
    }

    /// 练习按键处理
    fn handle_practice_key(&mut self, key: KeyCode) {
        if let AppState::Practice { ref mut selected_module, ref mut question_count, ref mut focus_field } = self.state {
            match key {
                KeyCode::Esc | KeyCode::Char('q') => {
                    // 返回仪表板
                    if let Some(repo) = &self.repo {
                        let module = *selected_module;
                        self.state = AppState::Dashboard { selected_module: module };
                    } else {
                        self.state = AppState::MainMenu;
                    }
                    self.update_help_text();
                }
                KeyCode::Tab => {
                    *focus_field = match focus_field {
                        PracticeField::Module => PracticeField::Count,
                        PracticeField::Count => PracticeField::Module,
                    };
                }
                KeyCode::Up => {
                    if let Some(repo) = &self.repo {
                        match focus_field {
                            PracticeField::Module => {
                                if *selected_module > 0 {
                                    *selected_module -= 1;
                                }
                            }
                            PracticeField::Count => {
                                if *question_count < 20 {
                                    *question_count += 1;
                                }
                            }
                        }
                    }
                }
                KeyCode::Down => {
                    if let Some(repo) = &self.repo {
                        match focus_field {
                            PracticeField::Module => {
                                if *selected_module < repo.modules.len().saturating_sub(1) {
                                    *selected_module += 1;
                                }
                            }
                            PracticeField::Count => {
                                if *question_count > 1 {
                                    *question_count -= 1;
                                }
                            }
                        }
                    }
                }
                KeyCode::Enter => {
                    if let Some(ref repo) = self.repo {
                        if let Some(module) = repo.modules.get(*selected_module) {
                            // 生成练习题
                            let questions = match module.id.as_str() {
                                "module-01-basics" | "01-基础入门" | "basics" => {
                                    crate::exercise::generate_basics_questions(*question_count)
                                }
                                _ => {
                                    self.message = Some("❌ 暂不支持该模块的练习题".to_string());
                                    return;
                                }
                            };

                            if !questions.is_empty() {
                                // 创建练习会话
                                let session = PracticeSession {
                                    questions,
                                    current_index: 0,
                                    answers: vec![None; *question_count],
                                    show_result: false,
                                };

                                // 切换到练习会话状态
                                self.state = AppState::PracticeSession { session };
                                self.update_help_text();
                            } else {
                                self.message = Some("❌ 没有可用的练习题".to_string());
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    /// 成就按键处理
    fn handle_achievements_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc | KeyCode::Char('q') => {
                // 返回主菜单
                self.state = AppState::MainMenu;
                self.state_stack.clear();
                self.update_help_text();
            }
            _ => {}
        }
    }

    /// 提醒设置按键处理
    fn handle_remind_setup_key(&mut self, key: KeyCode) {
        if let AppState::RemindSetup { ref mut hour, ref mut minute, ref mut focus_field } = self.state {
            match key {
                KeyCode::Esc | KeyCode::Char('q') => {
                    // 返回主菜单
                    self.state = AppState::MainMenu;
                    self.state_stack.clear();
                    self.update_help_text();
                }
                KeyCode::Tab => {
                    *focus_field = match focus_field {
                        TimeField::Hour => TimeField::Minute,
                        TimeField::Minute => TimeField::Hour,
                    };
                }
                KeyCode::Up => {
                    match focus_field {
                        TimeField::Hour => *hour = (*hour + 1).min(23),
                        TimeField::Minute => *minute = (*minute + 1).min(59),
                    }
                }
                KeyCode::Down => {
                    match focus_field {
                        TimeField::Hour => *hour = hour.saturating_sub(1),
                        TimeField::Minute => *minute = minute.saturating_sub(1),
                    }
                }
                KeyCode::Enter => {
                    let _ = crate::notify::set_reminder(*hour, *minute);
                    self.message = Some(format!("⏰ 已设置提醒时间为 {:02}:{:02}", hour, minute));
                    // 返回主菜单
                    self.state = AppState::MainMenu;
                    self.state_stack.clear();
                    self.update_help_text();
                }
                _ => {}
            }
        }
    }

    /// 导出按键处理
    fn handle_export_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc | KeyCode::Char('q') => {
                // 返回主菜单
                self.state = AppState::MainMenu;
                self.state_stack.clear();
                self.update_help_text();
            }
            KeyCode::Enter => {
                let _ = crate::storage::export_data();
                self.message = Some("📤 数据导出完成！".to_string());
            }
            _ => {}
        }
    }

    /// 练习会话按键处理
    fn handle_practice_session_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc | KeyCode::Char('q') => {
                // 退出练习会话，返回练习配置界面
                if let AppState::PracticeSession { .. } = self.state {
                    self.state = AppState::Practice {
                        selected_module: 0,
                        question_count: 5,
                        focus_field: PracticeField::Module,
                    };
                    self.update_help_text();
                }
            }
            KeyCode::Char('1') | KeyCode::Char('2') | KeyCode::Char('3') | KeyCode::Char('4') | KeyCode::Char('5') |
            KeyCode::Char('6') | KeyCode::Char('7') | KeyCode::Char('8') | KeyCode::Char('9') => {
                // 选择答案
                if let AppState::PracticeSession { ref mut session } = self.state {
                    let answer_index = match key {
                        KeyCode::Char('1') => 0,
                        KeyCode::Char('2') => 1,
                        KeyCode::Char('3') => 2,
                        KeyCode::Char('4') => 3,
                        KeyCode::Char('5') => 4,
                        KeyCode::Char('6') => 5,
                        KeyCode::Char('7') => 6,
                        KeyCode::Char('8') => 7,
                        KeyCode::Char('9') => 8,
                        _ => return,
                    };
                    if let Some(question) = session.questions.get(session.current_index) {
                        if let Some(options) = &question.options {
                            if answer_index < options.len() {
                                session.answers[session.current_index] = Some(answer_index);
                            }
                        }
                    }
                }
            }
            KeyCode::Left => {
                // 上一题
                if let AppState::PracticeSession { ref mut session } = self.state {
                    if session.current_index > 0 {
                        session.current_index -= 1;
                    }
                }
            }
            KeyCode::Right => {
                // 下一题
                if let AppState::PracticeSession { ref mut session } = self.state {
                    if session.current_index < session.questions.len() - 1 {
                        session.current_index += 1;
                    }
                }
            }
            KeyCode::Enter => {
                // 如果当前是最后一题，显示结果
                if let AppState::PracticeSession { ref mut session } = self.state {
                    if session.current_index == session.questions.len() - 1 {
                        // 计算得分并显示结果
                        let correct_count = session.answers.iter()
                            .zip(session.questions.iter())
                            .filter(|(answer, question)| {
                                answer.map(|a| a.to_string() == question.correct_answer).unwrap_or(false)
                            })
                            .count();

                        let score = (correct_count as f32 / session.questions.len() as f32) * 100.0;
                        self.message = Some(format!(
                            "✅ 练习完成！得分: {:.1}% ({}/{})",
                            score, correct_count, session.questions.len()
                        ));

                        // 返回练习配置界面
                        self.state = AppState::Practice {
                            selected_module: 0,
                            question_count: 5,
                            focus_field: PracticeField::Module,
                        };
                    } else {
                        session.current_index += 1;
                    }
                }
            }
            _ => {}
        }
    }

    /// 文件查看器按键处理
    fn handle_file_viewer_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc | KeyCode::Char('q') => {
                // 退出文件查看器，返回之前的状态
                if let AppState::FileViewer { return_state, .. } = &self.state.clone() {
                    self.state = *return_state.clone();
                    self.update_help_text();
                }
            }
            KeyCode::Up => {
                // 向上滚动
                if let AppState::FileViewer { ref mut scroll_offset, .. } = self.state {
                    *scroll_offset = scroll_offset.saturating_sub(1);
                }
            }
            KeyCode::Down => {
                // 向下滚动
                if let AppState::FileViewer { ref mut scroll_offset, ref content, .. } = self.state {
                    let max_offset = content.lines().count().saturating_sub(20);
                    *scroll_offset = (*scroll_offset + 1).min(max_offset);
                }
            }
            KeyCode::PageUp => {
                // 向上翻页
                if let AppState::FileViewer { ref mut scroll_offset, .. } = self.state {
                    *scroll_offset = scroll_offset.saturating_sub(20);
                }
            }
            KeyCode::PageDown => {
                // 向下翻页
                if let AppState::FileViewer { ref mut scroll_offset, ref content, .. } = self.state {
                    let max_offset = content.lines().count().saturating_sub(20);
                    *scroll_offset = (*scroll_offset + 20).min(max_offset);
                }
            }
            _ => {}
        }
    }

    /// 推入新状态
    fn push_state(&mut self, new_state: AppState) {
        self.state_stack.push(self.state.clone());
        self.state = new_state;
        self.message = None;
        self.update_help_text();
    }

    /// 弹出状态
    fn pop_state(&mut self) {
        if let Some(prev_state) = self.state_stack.pop() {
            self.state = prev_state;
            self.message = None;
            self.update_help_text();
        }
    }

    /// 更新帮助文本
    fn update_help_text(&mut self) {
        self.help_text = match self.state {
            AppState::MainMenu => "↑↓ 移动 | Enter 确认 | q 退出".to_string(),
            AppState::Dashboard { .. } => "↑↓ 选择模块 | O 打开详情 | Enter 更新 | P 练习 | Esc 返回".to_string(),
            AppState::ModuleDetail { .. } => "↑↓ 选择任务 | O 打开文件 | Space 标记完成 | Tab 切换 | Esc 返回".to_string(),
            AppState::UpdateProgress { .. } => "↑↓ 选择 | Tab 切换 | Enter 确认 | Esc 返回模块详情".to_string(),
            AppState::UpdateProgressConfirm { .. } => "←→ 选择 | Enter 确认 | Esc 返回".to_string(),
            AppState::Practice { .. } => "↑↓ 选择 | Tab 切换 | Enter 开始 | Esc 返回仪表板".to_string(),
            AppState::Achievements => "Esc 返回主菜单".to_string(),
            AppState::RemindSetup { .. } => "↑↓ 调整时间 | Tab 切换 | Enter 确认 | Esc 返回".to_string(),
            AppState::Export => "Enter 导出 | Esc 返回".to_string(),
            AppState::PracticeSession { .. } => "1-9 选择答案 | ←→ 切换题目 | Enter 下一题/完成 | Esc 退出".to_string(),
            AppState::FileViewer { .. } => "↑↓ 滚动 | PgUp/PgDn 翻页 | Esc 退出".to_string(),
        };
    }

    /// 获取当前帮助文本
    pub fn get_help_text(&self) -> &str {
        &self.help_text
    }
}

/// 运行 TUI 应用
pub fn run_tui(project_path: &str) -> Result<()> {
    // 初始化终端
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 创建应用
    let mut app = App::new(project_path.to_string());

    // 主循环
    loop {
        // 绘制界面
        terminal.draw(|f| ui(f, &mut app))?;

        // 检查是否应该退出
        if app.should_quit {
            break;
        }

        // 读取事件（超时 100ms）
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                // 只处理按键按下事件，忽略按键释放事件（Windows 会报告两种事件）
                if key.kind == KeyEventKind::Press {
                    app.handle_key(key.code)?;
                }
            }
        }
    }

    // 恢复终端
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

/// 绘制 UI
fn ui(f: &mut Frame, app: &mut App) {
    // 获取终端尺寸
    let size = f.size();

    // 检查最小尺寸
    if size.width < 80 || size.height < 24 {
        draw_size_warning(f, size);
        return;
    }

    // 创建布局
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(size);

    // 绘制标题
    draw_header(f, chunks[0], &app.title);

    // 绘制主内容区
    match &app.state {
        AppState::MainMenu => draw_main_menu(f, chunks[1], app),
        AppState::Dashboard { .. } => draw_dashboard(f, chunks[1], app),
        AppState::ModuleDetail { selected_module, selected_task, focus_area } => {
            draw_module_detail(f, chunks[1], app, *selected_module, *selected_task, *focus_area);
        }
        AppState::UpdateProgress { selected_module, selected_task, focus_area } => {
            draw_update_progress(f, chunks[1], app, *selected_module, *selected_task, *focus_area);
        }
        AppState::UpdateProgressConfirm { selected_module, selected_task, confirmed } => {
            draw_update_progress_confirm(f, chunks[1], app, *selected_module, *selected_task, *confirmed);
        }
        AppState::Practice { selected_module, question_count, focus_field } => {
            draw_practice(f, chunks[1], app, *selected_module, *question_count, *focus_field);
        }
        AppState::Achievements => draw_achievements(f, chunks[1], app),
        AppState::RemindSetup { hour, minute, focus_field } => {
            draw_remind_setup(f, chunks[1], *hour, *minute, *focus_field);
        }
        AppState::Export => draw_export(f, chunks[1]),
        AppState::PracticeSession { ref session } => draw_practice_session(f, chunks[1], session),
        AppState::FileViewer { ref file_path, ref content, scroll_offset, .. } => draw_file_viewer(f, chunks[1], file_path, content, *scroll_offset),
    }

    // 绘制消息（如果有）
    if let Some(msg) = &app.message {
        // 消息显示在底部区域，占据整个底部区域
        draw_message(f, chunks[2], msg);
    } else {
        // 没有消息时显示帮助文本
        draw_footer(f, chunks[2], app.get_help_text());
    }
}

/// 绘制标题栏
fn draw_header(f: &mut Frame, area: Rect, title: &str) {
    let title = Paragraph::new(Line::from(title.to_string()))
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    f.render_widget(title, area);
}

/// 绘制底部提示栏
fn draw_footer(f: &mut Frame, area: Rect, help_text: &str) {
    let footer = Paragraph::new(Line::from(help_text.to_string()))
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Gray));
    f.render_widget(footer, area);
}

/// 绘制消息
fn draw_message(f: &mut Frame, area: Rect, message: &str) {
    let msg = Paragraph::new(Line::from(message.to_string()))
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Green))
        .block(Block::default().borders(Borders::ALL).title("提示"));
    f.render_widget(msg, area);
}

/// 绘制尺寸警告
fn draw_size_warning(f: &mut Frame, area: Rect) {
    let warning = Paragraph::new("终端窗口太小！\n请调整到至少 80x24 字符")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .wrap(Wrap { trim: true });
    f.render_widget(warning, area);
}

/// 生成文本进度条
fn generate_progress_bar(percent: u16, width: u16) -> String {
    let filled = (percent as u32 * width as u32 + 50) / 100; // 四舍五入
    let empty = width as u32 - filled;
    let fill_char = "█";
    let empty_char = "░";
    format!("[{}{}] {}%", fill_char.repeat(filled as usize), empty_char.repeat(empty as usize), percent)
}

/// 绘制主菜单
fn draw_main_menu(f: &mut Frame, area: Rect, app: &mut App) {
    let mut menu_lines: Vec<Line> = Vec::new();
    for (i, item) in app.main_menu_items.iter().enumerate() {
        let prefix = if i == app.main_menu_selected { ">> " } else { "   " };
        let style = if i == app.main_menu_selected {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        menu_lines.push(Line::from(vec![
            Span::styled(prefix, style),
            Span::styled(item.as_str(), style),
        ]));
    }

    let menu_paragraph = Paragraph::new(menu_lines)
        .block(Block::default().borders(Borders::ALL).title("主菜单"))
        .wrap(Wrap { trim: true });
    f.render_widget(menu_paragraph, area);
}

/// 绘制仪表板
fn draw_dashboard(f: &mut Frame, area: Rect, app: &App) {
    if let Some(repo) = &app.repo {
        let completion = repo.completion_percentage();
        let completed = repo.progress.iter().filter(|p| p.status == "[x]").count();
        let total = repo.modules.len();

        // 获取当前选中的模块索引
        let selected_module = if let AppState::Dashboard { selected_module } = app.state {
            selected_module
        } else {
            0
        };

        // 创建垂直布局
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(9), Constraint::Min(0)].as_ref())
            .split(area);

        // 生成文本进度条
        let progress_bar = generate_progress_bar(completion as u16, 20);

        // 顶部统计区域
        let stats_lines = vec![
            Line::from("📊 学习进度仪表板"),
            Line::from(""),
            Line::from(format!("总体完成度: {:.1}% ({}/{})", completion, completed, total)),
            Line::from(progress_bar),
            Line::from(""),
            Line::from("快捷键: ↑↓ 选择模块 | O 详情 | P 练习 | A 成就"),
        ];

        let stats = Paragraph::new(stats_lines)
            .block(Block::default().borders(Borders::ALL).title("统计"))
            .wrap(Wrap { trim: true });
        f.render_widget(stats, chunks[0]);

        // 模块列表 - 可选择
        let mut module_lines: Vec<Line> = Vec::new();
        for (i, module) in repo.modules.iter().enumerate() {
            let progress = repo.get_module_progress(&module.id);
            let status_icon = if let Some(p) = progress {
                match p.status.as_str() {
                    "[x]" => "✅",
                    "[~]" => "🟡",
                    _ => "⬜",
                }
            } else {
                "⬜"
            };

            let tasks_done = if let Some(p) = progress {
                let count = [p.concept, p.examples, p.exercises, p.project, p.checklist]
                    .iter()
                    .filter(|&&x| x)
                    .count();
                count
            } else {
                0
            };

            let prefix = if i == selected_module { ">> " } else { "   " };
            let content = format!("{} {} - {}/5 任务", status_icon, module.name, tasks_done);
            let style = if i == selected_module {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            module_lines.push(Line::from(vec![
                Span::styled(prefix, style),
                Span::styled(content, style),
            ]));
        }

        let module_paragraph = Paragraph::new(module_lines)
            .block(Block::default().borders(Borders::ALL).title("学习模块 (↑↓ 选择)"))
            .wrap(Wrap { trim: true });
        f.render_widget(module_paragraph, chunks[1]);
    } else {
        let text = vec![
            Line::from("📊 学习仪表板"),
            Line::from(""),
            Line::from("正在加载数据..."),
        ];
        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("仪表板"))
            .wrap(Wrap { trim: true });
        f.render_widget(paragraph, area);
    }
}

/// 绘制模块详情界面
fn draw_module_detail(f: &mut Frame, area: Rect, app: &App, selected_module: usize, selected_task: usize, focus_area: ModuleFocus) {
    if let Some(repo) = &app.repo {
        if let Some(module) = repo.modules.get(selected_module) {
            let progress = repo.get_module_progress(&module.id);

            // 创建布局：左侧任务列表，右侧文件信息
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
                .split(area);

            // 任务列表 - 使用手动前缀
            let task_names = ["概念学习", "代码示例", "练习题", "综合练习", "自检通过"];
            let task_getters: [fn(&ModuleProgress) -> bool; 5] = [
                |p| p.concept,
                |p| p.examples,
                |p| p.exercises,
                |p| p.project,
                |p| p.checklist,
            ];

            let mut task_lines: Vec<Line> = Vec::new();
            for (i, task_name) in task_names.iter().enumerate() {
                let is_done = if let Some(p) = progress {
                    task_getters[i](p)
                } else {
                    false
                };
                let prefix = if i == selected_task && focus_area == ModuleFocus::TaskList { ">> " } else { "   " };
                let content = format!("[{}] {}", if is_done { 'x' } else { ' ' }, task_name);
                let style = if i == selected_task && focus_area == ModuleFocus::TaskList {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };
                task_lines.push(Line::from(vec![
                    Span::styled(prefix, style),
                    Span::styled(content, style),
                ]));
            }

            let task_paragraph = Paragraph::new(task_lines)
                .block(Block::default().borders(Borders::ALL).title("任务列表"))
                .wrap(Wrap { trim: true });
            f.render_widget(task_paragraph, chunks[0]);

            // 文件信息区域
            let mut file_info_lines = vec![
                Line::from("📁 模块文件"),
                Line::from(""),
                Line::from(format!("路径: {}", module.directory.display())),
                Line::from(""),
            ];

            // 添加文件可用性信息
            let file_names = ["README.md", "examples/", "exercises.md", "tests/", "自检清单.md"];
            let file_status = [
                module.has_readme,
                true, // examples always exists as directory
                module.has_exercises,
                module.has_tests,
                module.has_checklist,
            ];

            for (i, file_name) in file_names.iter().enumerate() {
                let exists = file_status[i];
                let icon = if exists { "✅" } else { "❌" };
                let style = if exists {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::DarkGray)
                };
                file_info_lines.push(Line::from(vec![
                    Span::styled(format!("{} ", icon), style),
                    Span::styled(*file_name, style),
                ]));
            }

            // 操作提示
            let action_style = if focus_area == ModuleFocus::Action {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };

            file_info_lines.push(Line::from(""));
            file_info_lines.push(Line::from("---"));
            file_info_lines.push(Line::from("操作:"));
            file_info_lines.push(Line::from(vec![
                Span::raw("  [O] 打开文件  "),
                Span::styled("[Space] 标记完成", action_style),
            ]));

            // 当前选中任务的操作提示
            let current_task = task_names.get(selected_task).unwrap_or(&"未知");
            file_info_lines.push(Line::from(""));
            file_info_lines.push(Line::from(format!("当前: {}", current_task)));

            let file_info = Paragraph::new(file_info_lines)
                .block(Block::default().borders(Borders::ALL).title("文件与操作"))
                .wrap(Wrap { trim: true });
            f.render_widget(file_info, chunks[1]);
        }
    }
}

/// 绘制更新进度界面
fn draw_update_progress(f: &mut Frame, area: Rect, app: &App, selected_module: usize, selected_task: usize, focus_area: FocusArea) {
    if let Some(repo) = &app.repo {
        // 创建水平布局
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(area);

        // 模块列表 - 使用手动前缀
        let module_border_style = if focus_area == FocusArea::ModuleList {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };

        let mut module_lines: Vec<Line> = Vec::new();
        for (i, module) in repo.modules.iter().enumerate() {
            let prefix = if i == selected_module && focus_area == FocusArea::ModuleList { ">> " } else { "   " };
            let style = if i == selected_module && focus_area == FocusArea::ModuleList {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            module_lines.push(Line::from(vec![
                Span::styled(prefix, style),
                Span::styled(module.name.clone(), style),
            ]));
        }

        let module_paragraph = Paragraph::new(module_lines)
            .block(Block::default().borders(Borders::ALL).title("选择模块").border_style(module_border_style))
            .wrap(Wrap { trim: true });
        f.render_widget(module_paragraph, chunks[0]);

        // 任务列表 - 使用手动前缀
        if let Some(module) = repo.modules.get(selected_module) {
            let progress = repo.get_module_progress(&module.id);
            let border_style = if focus_area == FocusArea::TaskList {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };

            let task_names = ["概念学习", "代码示例", "练习题", "综合练习", "自检通过"];
            let task_getters: [fn(&ModuleProgress) -> bool; 5] = [
                |p| p.concept,
                |p| p.examples,
                |p| p.exercises,
                |p| p.project,
                |p| p.checklist,
            ];

            let mut task_lines: Vec<Line> = Vec::new();
            for (i, task_name) in task_names.iter().enumerate() {
                let is_done = if let Some(p) = progress {
                    task_getters[i](p)
                } else {
                    false
                };
                let prefix = if i == selected_task && focus_area == FocusArea::TaskList { ">> " } else { "   " };
                let content = format!("[{}] {}", if is_done { 'x' } else { ' ' }, task_name);
                let style = if i == selected_task && focus_area == FocusArea::TaskList {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };
                task_lines.push(Line::from(vec![
                    Span::styled(prefix, style),
                    Span::styled(content, style),
                ]));
            }

            let task_paragraph = Paragraph::new(task_lines)
                .block(Block::default().borders(Borders::ALL).title(format!("任务列表 - {}", module.name)).border_style(border_style))
                .wrap(Wrap { trim: true });
            f.render_widget(task_paragraph, chunks[1]);
        }
    }
}

/// 绘制练习界面
fn draw_practice(f: &mut Frame, area: Rect, app: &App, selected_module: usize, question_count: usize, focus_field: PracticeField) {
    if let Some(repo) = &app.repo {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(8), Constraint::Min(0)].as_ref())
            .split(area);

        // 顶部标题区域
        let title_lines = vec![
            Line::from("✏️  练习测试"),
            Line::from(""),
            Line::from("选择模块和题目数量，然后按 Enter 开始练习"),
            Line::from(""),
        ];

        let title = Paragraph::new(title_lines)
            .block(Block::default().borders(Borders::ALL).title("练习配置"))
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center);
        f.render_widget(title, chunks[0]);

        // 模块选择区域
        let module_style = if focus_field == PracticeField::Module {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };

        let count_style = if focus_field == PracticeField::Count {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };

        if let Some(module) = repo.modules.get(selected_module) {
            let config_lines = vec![
                Line::from(""),
                Line::from(vec![
                    Span::raw("  模块: "),
                    Span::styled(format!("{} (按 ↑↓ 切换)", module.name), module_style),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::raw("  题目数量: "),
                    Span::styled(format!("{} (按 ↑↓ 调整)", question_count), count_style),
                ]),
                Line::from(""),
                Line::from(""),
                Line::from("操作: Tab 切换焦点 | ↑↓ 调整 | Enter 开始练习"),
            ];

            let config = Paragraph::new(config_lines)
                .block(Block::default().borders(Borders::ALL).title("练习设置"))
                .wrap(Wrap { trim: true });
            f.render_widget(config, chunks[1]);
        }
    }
}

/// 绘制成就界面
fn draw_achievements(f: &mut Frame, area: Rect, _app: &App) {
    // 获取成就数据
    let achievements = crate::db::get_all_achievements().unwrap_or_default();

    let mut text = vec![
        Line::from("🏆 成就系统"),
        Line::from(""),
    ];

    if achievements.is_empty() {
        text.push(Line::from("还没有解锁任何成就"));
        text.push(Line::from(""));
        text.push(Line::from("继续学习，解锁更多成就！"));
    } else {
        let unlocked_count = achievements.iter().filter(|a| a.unlocked).count();
        text.push(Line::from(format!("已解锁: {}/{}", unlocked_count, achievements.len())));
        text.push(Line::from(""));
        text.push(Line::from(""));

        for achievement in &achievements {
            let icon = if achievement.unlocked { "🏆" } else { "🔒" };
            let style = if achievement.unlocked {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::DarkGray)
            };

            text.push(Line::from(vec![
                Span::styled(format!("{} ", icon), style),
                Span::styled(achievement.name.clone(), style),
            ]));

            if achievement.unlocked {
                text.push(Line::from(vec![
                    Span::raw("   "),
                    Span::styled(achievement.description.clone(), Style::default().fg(Color::Gray)),
                ]));
            }
            text.push(Line::from(""));
        }
    }

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("成就"))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

/// 绘制提醒设置界面
fn draw_remind_setup(f: &mut Frame, area: Rect, hour: u8, minute: u8, focus_field: TimeField) {
    let hour_style = if focus_field == TimeField::Hour {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let minute_style = if focus_field == TimeField::Minute {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let text = vec![
        Line::from("⏰ 学习提醒设置"),
        Line::from(""),
        Line::from("设置每日学习提醒时间:"),
        Line::from(""),
        Line::from(vec![
            Span::raw("  小时: [ "),
            Span::styled(format!("{:02}", hour), hour_style),
            Span::raw(" ]"),
        ]),
        Line::from(vec![
            Span::raw("  分钟: [ "),
            Span::styled(format!("{:02}", minute), minute_style),
            Span::raw(" ]"),
        ]),
        Line::from(""),
        Line::from(""),
        Line::from("操作: ↑↓ 调整 | Tab 切换 | Enter 确认"),
    ];

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("提醒设置"))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

/// 绘制导出界面
fn draw_export(f: &mut Frame, area: Rect) {
    let text = vec![
        Line::from("📤 导出学习数据"),
        Line::from(""),
        Line::from("将导出所有学习记录到文件"),
        Line::from(""),
        Line::from("(功能开发中...)"),
    ];

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("数据导出"))
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center);

    f.render_widget(paragraph, area);
}

/// 绘制练习会话界面
fn draw_practice_session(f: &mut Frame, area: Rect, session: &PracticeSession) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // 标题和进度
            Constraint::Min(0),     // 题目内容
            Constraint::Length(3),  // 操作提示
        ].as_ref())
        .split(area);

    // 标题和进度
    let progress = format!("题目 {}/{} - [{}]",
        session.current_index + 1,
        session.questions.len(),
        session.questions[session.current_index].topic
    );

    let title_lines = vec![
        Line::from("✏️  练习测试"),
        Line::from(""),
        Line::from(progress),
    ];

    let title = Paragraph::new(title_lines)
        .block(Block::default().borders(Borders::ALL).title("练习"))
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    // 题目内容区域
    let question = &session.questions[session.current_index];
    let current_answer = session.answers[session.current_index];

    let mut content_lines = vec![
        Line::from(""),
        Line::from(format!("问题: {}", question.prompt)),
        Line::from(""),
    ];

    // 显示选项
    if let Some(options) = &question.options {
        for (i, opt) in options.iter().enumerate() {
            let prefix = if current_answer == Some(i) { ">> " } else { "   " };
            let number = format!("{}. ", i + 1);

            let style = if current_answer == Some(i) {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            content_lines.push(Line::from(vec![
                Span::styled(prefix, style),
                Span::styled(number, style),
                Span::styled(opt.clone(), style),
            ]));
        }
    }

    // 添加解析（如果已回答）
    if current_answer.is_some() {
        content_lines.push(Line::from(""));
        content_lines.push(Line::from(format!("💡 解析: {}", question.explanation)));
    }

    let content = Paragraph::new(content_lines)
        .block(Block::default().borders(Borders::ALL).title("题目"))
        .wrap(Wrap { trim: true });
    f.render_widget(content, chunks[1]);

    // 操作提示
    let help_text = if session.current_index == session.questions.len() - 1 {
        "1-9 选择答案 | ←→ 切换题目 | Enter 完成练习 | Esc 退出"
    } else {
        "1-9 选择答案 | ←→ 切换题目 | Enter 下一题 | Esc 退出"
    };

    let help = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("操作"))
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Gray));
    f.render_widget(help, chunks[2]);
}

/// 绘制文件查看器界面
fn draw_file_viewer(f: &mut Frame, area: Rect, file_path: &str, content: &str, scroll_offset: usize) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // 文件信息
            Constraint::Min(0),     // 文件内容
            Constraint::Length(3),  // 操作提示
        ].as_ref())
        .split(area);

    // 文件信息
    let info_lines = vec![
        Line::from("📄 文件查看器"),
        Line::from(""),
        Line::from(format!("文件: {}", file_path)),
    ];

    let info = Paragraph::new(info_lines)
        .block(Block::default().borders(Borders::ALL).title("文件信息"))
        .alignment(Alignment::Center);
    f.render_widget(info, chunks[0]);

    // 文件内容
    let lines: Vec<Line> = content
        .lines()
        .skip(scroll_offset)
        .take(area.height as usize - 6) // 减去上下边框和操作区域
        .map(|line| Line::from(line.to_string()))
        .collect();

    let content_widget = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("内容"))
        .wrap(Wrap { trim: true });
    f.render_widget(content_widget, chunks[1]);

    // 操作提示
    let help_text = format!(
        "↑↓ 滚动 | PgUp/PgDn 翻页 | 总行数: {} | 当前行: {} | Esc 退出",
        content.lines().count(),
        scroll_offset + 1
    );

    let help = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("操作"))
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Gray));
    f.render_widget(help, chunks[2]);
}

/// 绘制更新进度确认界面
fn draw_update_progress_confirm(f: &mut Frame, area: Rect, app: &App, selected_module: usize, selected_task: usize, confirmed: bool) {
    if let Some(repo) = &app.repo {
        if let Some(module) = repo.modules.get(selected_module) {
            let task_names = ["概念学习", "代码示例", "练习题", "综合练习", "自检通过"];
            let task_name = task_names.get(selected_task).unwrap_or(&"任务");

            let yes_style = if confirmed {
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let no_style = if !confirmed {
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let text = vec![
                Line::from("✅ 确认更新进度"),
                Line::from(""),
                Line::from(""),
                Line::from(format!("模块: {}", module.name)),
                Line::from(format!("任务: {}", task_name)),
                Line::from(""),
                Line::from(""),
                Line::from("确认要标记为已完成吗？"),
                Line::from(""),
                Line::from(""),
                Line::from(vec![
                    Span::raw("  [ "),
                    Span::styled("是 (Y)", yes_style),
                    Span::raw(" ]    "),
                    Span::raw("[ "),
                    Span::styled("否 (N)", no_style),
                    Span::raw(" ]  "),
                ]),
                Line::from(""),
                Line::from("操作: ←→ 选择 | Enter 确认 | Esc 返回"),
            ];

            let paragraph = Paragraph::new(text)
                .block(Block::default().borders(Borders::ALL).title("确认"))
                .wrap(Wrap { trim: true })
                .alignment(Alignment::Center);

            f.render_widget(paragraph, area);
        }
    }
}
