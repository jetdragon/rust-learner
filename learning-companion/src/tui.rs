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
    text::Line,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};
use std::io;
use std::time::Duration;

/// 应用状态
#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    MainMenu,
    Dashboard,
    UpdateProgress,
    Practice,
    Achievements,
    RemindSetup,
    Export,
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
    /// 错误消息
    pub error_message: Option<String>,
    /// 是否应该退出
    pub should_quit: bool,
}

impl App {
    /// 创建新应用
    pub fn new() -> Self {
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
            error_message: None,
            should_quit: false,
        }
    }

    /// 处理按键事件
    pub fn handle_key(&mut self, key: KeyCode) {
        match self.state {
            AppState::MainMenu => self.handle_main_menu_key(key),
            AppState::Dashboard => self.handle_dashboard_key(key),
            AppState::UpdateProgress => self.handle_update_progress_key(key),
            AppState::Practice => self.handle_practice_key(key),
            AppState::Achievements => self.handle_achievements_key(key),
            AppState::RemindSetup => self.handle_remind_setup_key(key),
            AppState::Export => self.handle_export_key(key),
        }
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
                self.enter_main_menu_selection();
            }
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    /// 进入主菜单选中的项
    fn enter_main_menu_selection(&mut self) {
        match self.main_menu_selected {
            0 => self.push_state(AppState::Dashboard),
            1 => self.push_state(AppState::UpdateProgress),
            2 => self.push_state(AppState::Practice),
            3 => self.push_state(AppState::Achievements),
            4 => self.push_state(AppState::RemindSetup),
            5 => self.push_state(AppState::Export),
            6 => self.should_quit = true,
            _ => {}
        }
    }

    /// 仪表板按键处理
    fn handle_dashboard_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.pop_state();
            }
            _ => {}
        }
    }

    /// 更新进度按键处理
    fn handle_update_progress_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.pop_state();
            }
            _ => {}
        }
    }

    /// 练习按键处理
    fn handle_practice_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.pop_state();
            }
            _ => {}
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
        match key {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.pop_state();
            }
            _ => {}
        }
    }

    /// 导出按键处理
    fn handle_export_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.pop_state();
            }
            _ => {}
        }
    }

    /// 推入新状态
    fn push_state(&mut self, new_state: AppState) {
        self.state_stack.push(self.state.clone());
        self.state = new_state;
        self.update_help_text();
    }

    /// 弹出状态
    fn pop_state(&mut self) {
        if let Some(prev_state) = self.state_stack.pop() {
            self.state = prev_state;
            self.update_help_text();
        }
    }

    /// 更新帮助文本
    fn update_help_text(&mut self) {
        self.help_text = match self.state {
            AppState::MainMenu => "↑↓ 移动 | Enter 确认 | q 退出".to_string(),
            _ => "Esc 返回 | q 退出".to_string(),
        };
    }

    /// 获取当前帮助文本
    pub fn get_help_text(&self) -> &str {
        &self.help_text
    }
}

/// 运行 TUI 应用
pub fn run_tui() -> Result<()> {
    // 初始化终端
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 创建应用
    let mut app = App::new();

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
                app.handle_key(key.code);
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
    match app.state {
        AppState::MainMenu => draw_main_menu(f, chunks[1], app),
        AppState::Dashboard => draw_dashboard(f, chunks[1]),
        AppState::UpdateProgress => draw_update_progress(f, chunks[1]),
        AppState::Practice => draw_practice(f, chunks[1]),
        AppState::Achievements => draw_achievements(f, chunks[1]),
        AppState::RemindSetup => draw_remind_setup(f, chunks[1]),
        AppState::Export => draw_export(f, chunks[1]),
    }

    // 绘制底部提示
    draw_footer(f, chunks[2], app.get_help_text());
}

/// 绘制标题栏
fn draw_header(f: &mut Frame, area: Rect, title: &str) {
    let title = Paragraph::new(Line::from(title.to_string()))
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );
    f.render_widget(title, area);
}

/// 绘制底部提示栏
fn draw_footer(f: &mut Frame, area: Rect, help_text: &str) {
    let footer = Paragraph::new(Line::from(help_text.to_string()))
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(Color::Gray)
        );
    f.render_widget(footer, area);
}

/// 绘制尺寸警告
fn draw_size_warning(f: &mut Frame, area: Rect) {
    let warning = Paragraph::new("终端窗口太小！\n请调整到至少 80x24 字符")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
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
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    let mut list_state = ListState::default();
    list_state.select(Some(app.main_menu_selected));

    f.render_stateful_widget(list, area, &mut list_state);
}

/// 绘制仪表板
fn draw_dashboard(f: &mut Frame, area: Rect) {
    let text = vec![
        Line::from("📊 学习仪表板"),
        Line::from(""),
        Line::from("这里将显示学习进度和统计数据"),
        Line::from(""),
        Line::from("(功能开发中...)"),
    ];

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("仪表板"))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

/// 绘制更新进度界面
fn draw_update_progress(f: &mut Frame, area: Rect) {
    let text = vec![
        Line::from("📚 更新学习进度"),
        Line::from(""),
        Line::from("这里将显示模块选择和任务复选框"),
        Line::from(""),
        Line::from("(功能开发中...)"),
    ];

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("更新进度"))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

/// 绘制练习界面
fn draw_practice(f: &mut Frame, area: Rect) {
    let text = vec![
        Line::from("✏️  练习测试"),
        Line::from(""),
        Line::from("这里将显示练习题目和选项"),
        Line::from(""),
        Line::from("(功能开发中...)"),
    ];

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("练习测试"))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

/// 绘制成就界面
fn draw_achievements(f: &mut Frame, area: Rect) {
    let text = vec![
        Line::from("🏆 成就系统"),
        Line::from(""),
        Line::from("这里将显示已解锁和待解锁的成就"),
        Line::from(""),
        Line::from("(功能开发中...)"),
    ];

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("成就"))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

/// 绘制提醒设置界面
fn draw_remind_setup(f: &mut Frame, area: Rect) {
    let text = vec![
        Line::from("⏰ 学习提醒设置"),
        Line::from(""),
        Line::from("这里将设置每日学习提醒时间"),
        Line::from(""),
        Line::from("(功能开发中...)"),
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
        Line::from("这里将导出所有学习记录"),
        Line::from(""),
        Line::from("(功能开发中...)"),
    ];

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("数据导出"))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}
