//! 数据库模块
//!
//! 使用 SQLite 存储学习记录和统计数据

use rusqlite::{Connection, OptionalExtension};
use chrono::{Local, NaiveDate};
use anyhow::Result;
use std::path::PathBuf;

/// 数据库文件路径
fn db_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".learning-companion");
    path.push("data.db");
    path
}

/// 初始化数据库
pub fn init_db() -> Result<()> {
    let db_path = db_path();

    // 创建目录（如果不存在）
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let conn = Connection::open(&db_path)?;

    // 创建表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS study_sessions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date TEXT NOT NULL,
            duration_minutes INTEGER NOT NULL,
            modules_studied TEXT NOT NULL,
            practice_count INTEGER NOT NULL DEFAULT 0,
            notes TEXT
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS module_progress (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            module_id TEXT NOT NULL UNIQUE,
            started_at TEXT,
            completed_at TEXT,
            mastery_score REAL DEFAULT 0.0,
            last_updated TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS practice_results (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            module_id TEXT NOT NULL,
            timestamp TEXT NOT NULL,
            questions_total INTEGER NOT NULL,
            questions_correct INTEGER NOT NULL,
            score REAL NOT NULL,
            weak_topics TEXT
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS achievements (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            achievement_type TEXT NOT NULL UNIQUE,
            unlocked_at TEXT NOT NULL,
            metadata TEXT
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        )",
        [],
    )?;

    Ok(())
}

/// 记录学习会话
pub fn record_study_session(
    duration_minutes: u32,
    modules_studied: Vec<String>,
    practice_count: u32,
    notes: Option<String>,
) -> Result<()> {
    let conn = Connection::open(db_path())?;

    let date = Local::now().format("%Y-%m-%d").to_string();
    let modules = modules_studied.join(",");

    conn.execute(
        "INSERT INTO study_sessions (date, duration_minutes, modules_studied, practice_count, notes)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        [date, duration_minutes.to_string(), modules, practice_count.to_string(), notes.unwrap_or_default()],
    )?;

    Ok(())
}

/// 更新模块进度
pub fn update_module_progress(
    module_id: &str,
    mastery_score: f32,
) -> Result<()> {
    let conn = Connection::open(db_path())?;
    let now = Local::now().to_rfc3339();

    conn.execute(
        "INSERT INTO module_progress (module_id, mastery_score, last_updated)
         VALUES (?1, ?2, ?3)
         ON CONFLICT(module_id) DO UPDATE SET
         mastery_score = ?2,
         last_updated = ?3",
        [module_id, &mastery_score.to_string(), &now],
    )?;

    Ok(())
}

/// 记录练习结果
pub fn record_practice_result(
    module_id: &str,
    questions_total: u32,
    questions_correct: u32,
    score: f32,
    weak_topics: Vec<String>,
) -> Result<()> {
    let conn = Connection::open(db_path())?;
    let timestamp = Local::now().to_rfc3339();
    let topics = weak_topics.join(",");

    conn.execute(
        "INSERT INTO practice_results (module_id, timestamp, questions_total, questions_correct, score, weak_topics)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        [
            module_id,
            &timestamp,
            &questions_total.to_string(),
            &questions_correct.to_string(),
            &score.to_string(),
            &topics,
        ],
    )?;

    Ok(())
}

/// 检查并解锁成就
pub fn check_and_unlock_achievement(achievement_type: &str) -> Result<bool> {
    let conn = Connection::open(db_path())?;

    // 检查是否已解锁
    let mut stmt = conn.prepare(
        "SELECT COUNT(*) FROM achievements WHERE achievement_type = ?1"
    )?;

    let count: i64 = stmt.query_row([achievement_type], |row| row.get(0))?;

    if count > 0 {
        return Ok(false); // 已解锁
    }

    // 解锁成就
    let now = Local::now().to_rfc3339();
    conn.execute(
        "INSERT INTO achievements (achievement_type, unlocked_at) VALUES (?1, ?2)",
        [achievement_type, &now],
    )?;

    Ok(true)
}

/// 获取连续学习天数
pub fn get_streak_days() -> Result<u32> {
    let conn = Connection::open(db_path())?;

    let mut stmt = conn.prepare(
        "SELECT DISTINCT date FROM study_sessions ORDER BY date DESC"
    )?;

    let dates: Result<Vec<String>, _> = stmt
        .query_map([], |row| row.get(0))
        .unwrap()
        .collect();

    let dates = dates?;
    if dates.is_empty() {
        return Ok(0);
    }

    let mut streak = 1u32;
    let mut prev_date = NaiveDate::parse_from_str(&dates[0], "%Y-%m-%d").unwrap();

    for date_str in dates.iter().skip(1) {
        let current_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
        let days_diff = (prev_date - current_date).num_days();

        if days_diff == 1 {
            streak += 1;
            prev_date = current_date;
        } else if days_diff > 1 {
            break;
        }
    }

    Ok(streak)
}

/// 保存设置
pub fn save_setting(key: &str, value: &str) -> Result<()> {
    let conn = Connection::open(db_path())?;
    conn.execute(
        "INSERT INTO settings (key, value) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value = ?2",
        [key, value],
    )?;
    Ok(())
}

/// 获取设置
pub fn get_setting(key: &str) -> Result<Option<String>> {
    let conn = Connection::open(db_path())?;
    let mut stmt = conn.prepare("SELECT value FROM settings WHERE key = ?1")?;

    let result: Result<Option<String>, _> = stmt
        .query_row([key], |row| row.get(0))
        .optional();

    result.map_err(Into::into)
}

/// 导出所有数据
pub fn export_all_data() -> Result<String> {
    let conn = Connection::open(db_path())?;

    let mut sessions = String::new();
    let mut stmt = conn.prepare("SELECT * FROM study_sessions")?;
    let session_rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, i32>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, i32>(4)?,
            row.get::<_, String>(5)?,
        ))
    })?;

    for session in session_rows {
        let (id, date, duration, modules, practice, notes) = session?;
        sessions.push_str(&format!(
            "Session {}: {} | {}min | modules: {} | practice: {} | notes: {}\n",
            id, date, duration, modules, practice, notes
        ));
    }

    Ok(sessions)
}

/// 获取模块掌握程度
pub fn get_module_mastery(module_id: &str) -> Option<f32> {
    let conn = Connection::open(db_path()).ok()?;
    let mut stmt = conn.prepare(
        "SELECT mastery_score FROM module_progress WHERE module_id = ?1"
    ).ok()?;

    stmt.query_row([module_id], |row| row.get(0)).ok()
}

/// 成就记录
#[derive(Debug, Clone)]
pub struct Achievement {
    pub name: String,
    pub description: String,
    pub unlocked: bool,
    pub unlocked_at: Option<chrono::DateTime<Local>>,
}

/// 成就定义
const ALL_ACHIEVEMENTS: &[(&str, &str)] = &[
    ("first_steps", "初次学习 - 完成第一个模块"),
    ("week_warrior", "坚持一周 - 连续学习 7 天"),
    ("month_master", "坚持一月 - 连续学习 30 天"),
    ("practice_perfect", "练习达人 - 单次练习 100% 正确"),
    ("half_way", "半程高手 - 完成 50% 的学习内容"),
    ("completionist", "学习大师 - 完成所有模块"),
];

/// 获取所有成就
pub fn get_all_achievements() -> Result<Vec<Achievement>> {
    let conn = Connection::open(db_path())?;

    // 获取已解锁的成就
    let mut stmt = conn.prepare(
        "SELECT achievement_type FROM achievements"
    )?;

    let unlocked_types: Vec<String> = stmt.query_map([], |row| row.get(0))?
        .collect::<Result<Vec<_>, _>>()?;

    let mut achievements = Vec::new();
    for (achievement_type, description) in ALL_ACHIEVEMENTS {
        let unlocked = unlocked_types.contains(&achievement_type.to_string());
        achievements.push(Achievement {
            name: achievement_type.to_string(),
            description: description.to_string(),
            unlocked,
            unlocked_at: None, // 简化实现
        });
    }

    Ok(achievements)
}
