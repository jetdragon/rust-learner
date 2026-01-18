//! 通知模块
//!
//! 系统通知和定时提醒

use anyhow::Result;
use chrono::{Local, Timelike};
use notify_rust::Notification;
use std::thread;
use std::time::Duration;

/// 设置提醒
pub fn set_reminder(hour: u8, minute: u8) -> Result<()> {
    // 保存提醒设置到数据库
    let time_str = format!("{:02}:{:02}", hour, minute);
    crate::db::save_setting("remind_time", &time_str)?;

    // 在后台线程中运行提醒
    thread::spawn(move || {
        run_reminder_loop(hour, minute);
    });

    Ok(())
}

/// 运行提醒循环
fn run_reminder_loop(hour: u8, minute: u8) {
    loop {
        let now = Local::now();
        if now.hour() as u8 == hour && now.minute() as u8 == minute {
            if let Err(e) = send_learning_reminder() {
                eprintln!("发送提醒失败：{}", e);
            }
        }

        // 每分钟检查一次
        thread::sleep(Duration::from_secs(60));
    }
}

/// 发送学习提醒
fn send_learning_reminder() -> Result<()> {
    let messages = vec![
        "🦀 学习时间到了！今天的 Rust 学习安排好了吗？",
        "💪 坚持学习 Rust，你一定可以成为优秀的开发者！",
        "📚 每天进步一点点，积少成多！",
        "🔥 保持学习的热情，继续加油！",
        "⚡ 你的 Rust 技能正在提升中！",
    ];

    // 随机选择一条消息
    let index = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        % messages.len() as u64) as usize;

    Notification::new()
        .summary("Rust 学习伴侣")
        .body(messages[index])
        .show()?;

    Ok(())
}

/// 发送成就解锁通知
pub fn send_achievement_notification(achievement_name: &str) -> Result<()> {
    Notification::new()
        .summary("🏆 成就解锁！")
        .body(&format!("恭喜你解锁成就：{}", achievement_name))
        .show()?;

    Ok(())
}
