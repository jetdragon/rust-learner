//! TUI 模块
//!
//! 交互式终端用户界面实现

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, ListState, Paragraph, Wrap},
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
    UpdateProgress { selected_module: usize, selected_task: usize, focus_area: FocusArea },
    Practice { selected_module: usize, question_count: usize, focus_field: PracticeField },
    Achievements,
    RemindSetup { hour: u8, minute: u8, focus_field: TimeField },
    Export,
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
        match self.state {
            AppState::MainMenu => self.handle_main_menu_key(key),
            AppState::Dashboard { .. } => self.handle_dashboard_key(key),
            AppState::UpdateProgress { .. } => self.handle_update_progress_key(key),
            AppState::Practice { .. } => self.handle_practice_key(key),
            AppState::Achievements => self.handle_achievements_key(key),
            AppState::RemindSetup { .. } => self.handle_remind_setup_key(key),
            AppState::Export => self.handle_export_key(key),
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
                    self.pop_state();
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
                _ => {}
            }
        }
    }

    /// 更新进度按键处理
    fn handle_update_progress_key(&mut self, key: KeyCode) {
        if let AppState::UpdateProgress { ref mut selected_module, ref mut selected_task, ref mut focus_area } = self.state {
            match key {
                KeyCode::Esc | KeyCode::Char('q') => {
                    self.pop_state();
                }
                KeyCode::Tab => {
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
                _ => {}
            }
        }
    }

    /// 练习按键处理
    fn handle_practice_key(&mut self, key: KeyCode) {
        if let AppState::Practice { ref mut selected_module, ref mut question_count, ref mut focus_field } = self.state {
            match key {
                KeyCode::Esc | KeyCode::Char('q') => {
                    self.pop_state();
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
                            let _ = crate::exercise::run_practice(repo, &module.id, *question_count);
                            self.message = Some(format!("✅ {} 的练习完成！(共 {} 题)", module.name, question_count));
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
                self.pop_state();
            }
            _ => {}
        }
    }

    /// 提醒设置按键处理
    fn handle_remind_setup_key(&mut self, key: KeyCode) {
        if let AppState::RemindSetup { ref mut hour, ref mut minute, ref mut focus_field } = self.state {
            match key {
                KeyCode::Esc | KeyCode::Char('q') => {
                    self.pop_state();
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
                    self.pop_state();
                }
                _ => {}
            }
        }
    }

    /// 导出按键处理
    fn handle_export_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.pop_state();
            }
            KeyCode::Enter => {
                let _ = crate::storage::export_data();
                self.message = Some("📤 数据导出完成！".to_string());
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
            AppState::Dashboard { .. } => "↑↓ 选择模块 | Enter 更新进度 | U 更新 | P 练习 | Esc 返回".to_string(),
            AppState::UpdateProgress { .. } => "↑↓ 选择 | Tab 切换 | Enter 确认 | Esc 返回仪表板".to_string(),
            AppState::Practice { .. } => "↑↓ 选择 | Tab 切换 | Enter 开始 | Esc 返回仪表板".to_string(),
            AppState::Achievements => "Esc 返回".to_string(),
            AppState::RemindSetup { .. } => "↑↓ 调整时间 | Tab 切换 | Enter 确认 | Esc 返回".to_string(),
            AppState::Export => "Enter 导出 | Esc 返回".to_string(),
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
                app.handle_key(key.code)?;
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
        AppState::UpdateProgress { selected_module, selected_task, focus_area } => {
            draw_update_progress(f, chunks[1], app, *selected_module, *selected_task, *focus_area);
        }
        AppState::Practice { selected_module, question_count, focus_field } => {
            draw_practice(f, chunks[1], app, *selected_module, *question_count, *focus_field);
        }
        AppState::Achievements => draw_achievements(f, chunks[1], app),
        AppState::RemindSetup { hour, minute, focus_field } => {
            draw_remind_setup(f, chunks[1], *hour, *minute, *focus_field);
        }
        AppState::Export => draw_export(f, chunks[1]),
    }

    // 绘制消息（如果有）
    if let Some(msg) = &app.message {
        let msg_area = Rect {
            x: chunks[2].x,
            y: chunks[2].y + 1,
            width: chunks[2].width,
            height: chunks[2].height.saturating_sub(1),
        };
        draw_message(f, msg_area, msg);
    }

    // 绘制底部提示
    let footer_area = if app.message.is_some() {
        Rect {
            x: chunks[2].x,
            y: chunks[2].y,
            width: chunks[2].width,
            height: 1,
        }
    } else {
        chunks[2]
    };
    draw_footer(f, footer_area, app.get_help_text());
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
        .style(Style::default().fg(Color::Green));
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

/// 绘制主菜单
fn draw_main_menu(f: &mut Frame, area: Rect, app: &mut App) {
    let items: Vec<ListItem> = app
        .main_menu_items
        .iter()
        .map(|item| ListItem::new(item.as_str()))
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("主菜单"))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    let mut list_state = ListState::default();
    list_state.select(Some(app.main_menu_selected));

    f.render_stateful_widget(list, area, &mut list_state);
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
            .constraints([Constraint::Length(10), Constraint::Min(0)].as_ref())
            .split(area);

        // 顶部统计区域
        let stats_lines = vec![
            Line::from("📊 学习进度仪表板"),
            Line::from(""),
            Line::from(format!("总体完成度: {:.1}% ({}/{})", completion, completed, total)),
            Line::from(""),
            Line::from("快捷键: ↑↓ 选择模块 | Enter/U 更新进度 | P 练习 | A 成就"),
        ];

        let stats = Paragraph::new(stats_lines)
            .block(Block::default().borders(Borders::ALL).title("统计"))
            .wrap(Wrap { trim: true });
        f.render_widget(stats, chunks[0]);

        // 进度条
        let gauge_area = Rect {
            x: chunks[0].x + 2,
            y: chunks[0].y + 7,
            width: chunks[0].width.saturating_sub(4),
            height: 1,
        };
        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL))
            .gauge_style(Style::default().fg(Color::Green))
            .percent(completion as u16);
        f.render_widget(gauge, gauge_area);

        // 模块列表 - 可选择
        let mut module_items = Vec::new();
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
            module_items.push(ListItem::new(format!(
                "{}{} {} - {}/5 任务",
                prefix, status_icon, module.name, tasks_done
            )));
        }

        let module_list = List::new(module_items)
            .block(Block::default().borders(Borders::ALL).title("学习模块 (↑↓ 选择)"))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

        let mut list_state = ListState::default();
        list_state.select(Some(selected_module));

        f.render_stateful_widget(module_list, chunks[1], &mut list_state);
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

/// 绘制更新进度界面
fn draw_update_progress(f: &mut Frame, area: Rect, app: &App, selected_module: usize, selected_task: usize, focus_area: FocusArea) {
    if let Some(repo) = &app.repo {
        // 创建水平布局
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(area);

        // 模块列表
        let mut module_items = Vec::new();
        for (i, module) in repo.modules.iter().enumerate() {
            let prefix = if i == selected_module && focus_area == FocusArea::ModuleList {
                ">> "
            } else {
                "   "
            };
            module_items.push(ListItem::new(format!("{}{}", prefix, module.name)));
        }

        let module_border_style = if focus_area == FocusArea::ModuleList {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };

        let module_list = List::new(module_items)
            .block(Block::default().borders(Borders::ALL).title("选择模块").border_style(module_border_style))
            .style(Style::default().fg(Color::White));
        f.render_widget(module_list, chunks[0]);

        // 任务列表
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

            let mut task_lines = vec![
                Line::from("选择要标记完成的任务:"),
                Line::from(""),
            ];

            for (i, task_name) in task_names.iter().enumerate() {
                let is_done = if let Some(p) = progress {
                    task_getters[i](p)
                } else {
                    false
                };
                let is_selected = i == selected_task && focus_area == FocusArea::TaskList;
                let marker = if is_selected { ">> " } else { "   " };

                task_lines.push(Line::from(format!(
                    "{}[{}] {}",
                    marker,
                    if is_done { 'x' } else { ' ' },
                    task_name
                )));
            }

            task_lines.push(Line::from(""));
            task_lines.push(Line::from("操作: ↑↓ 选择 | Tab 切换 | Enter 确认"));

            let task_paragraph = Paragraph::new(task_lines)
                .block(Block::default().borders(Borders::ALL).title("任务列表").border_style(border_style))
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
