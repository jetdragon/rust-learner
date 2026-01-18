use anyhow::Result;
use chrono::Local;
use rusqlite::Connection;
use std::path::{Path, PathBuf};

fn db_path() -> PathBuf {
    dirs::home_dir()
        .map(|mut p| {
            p.push(".learning-companion");
            p.push("data.db");
        })
        .unwrap_or_else(|| PathBuf::from(".learning-companion/data.db"))
}

pub fn init_db() -> Result<()> {
    let db_path = db_path();

    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let conn = Connection::open(&db_path)?;

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

    Ok(())
}

pub fn get_module_mastery(module_id: &str) -> Option<f32> {
    let conn = Connection::open(db_path()).ok()?;
    let mut stmt = conn
        .prepare("SELECT mastery_score FROM module_progress WHERE module_id = ?1")
        .ok()?;

    stmt.query_row([module_id], |row| row.get(0)).ok()
}

pub fn update_module_progress(module_id: &str, mastery_score: f32) -> Result<()> {
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

pub fn record_practice_result(
    module_id: &str,
    questions_total: u32,
    questions_correct: u32,
    score: f32,
) -> Result<()> {
    let conn = Connection::open(db_path())?;
    let timestamp = Local::now().to_rfc3339();

    conn.execute(
        "INSERT INTO practice_results (module_id, timestamp, questions_total, questions_correct, score)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        [
            module_id,
            &timestamp,
            &questions_total.to_string(),
            &questions_correct.to_string(),
            &score.to_string(),
        ],
    )?;

    Ok(())
}

pub fn check_and_unlock_achievement(achievement_type: &str) -> Result<bool> {
    let conn = Connection::open(db_path())?;

    let mut stmt = conn.prepare("SELECT COUNT(*) FROM achievements WHERE achievement_type = ?1")?;
    let count: i64 = stmt.query_row([achievement_type], |row| row.get(0))?;

    if count > 0 {
        return Ok(false);
    }

    let now = Local::now().to_rfc3339();
    conn.execute(
        "INSERT INTO achievements (achievement_type, unlocked_at) VALUES (?1, ?2)",
        [achievement_type, &now],
    )?;

    Ok(true)
}

pub fn get_all_achievements() -> Result<Vec<String>> {
    let conn = Connection::open(db_path())?;
    let mut stmt = conn.prepare("SELECT achievement_type FROM achievements")?;

    let unlocked_types: Vec<String> = stmt
        .query_map([], |row| row.get(0))?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(unlocked_types)
}
