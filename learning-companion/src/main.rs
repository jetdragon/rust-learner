//! Rust 学习伴侣 - 终端版本
//!
//! 一个帮助追踪 Rust 学习进度、提供练习和激励的命令行工具

mod db;
mod exercise;
mod progress;
mod repo;
mod storage;
mod ui;
mod notify;

use clap::{Parser, Subcommand};
use chrono::Local;
use anyhow::Result;

#[derive(Parser)]
#[command(name = "learning-companion")]
#[command(about = "Rust 学习伴侣 - 追踪学习进度，提供练习和激励", long_about = None)]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 显示学习仪表板
    Dashboard {
        /// 仓库路径（默认为当前目录）
        #[arg(short, long, default_value = ".")]
        path: String,
    },
    /// 更新学习进度
    Update {
        /// 模块名称（如 module-01-basics）
        #[arg(short, long)]
        module: String,
        /// 完成的任务（概念、示例、练习、综合、自检）
        #[arg(short, long)]
        task: String,
    },
    /// 开始练习测试
    Practice {
        /// 模块名称
        #[arg(short, long)]
        module: String,
        /// 题目数量
        #[arg(short, long, default_value = "5")]
        count: usize,
    },
    /// 设置提醒
    Remind {
        /// 小时 (0-23)
        #[arg(short, long)]
        hour: u8,
        /// 分钟 (0-59)
        #[arg(short, long)]
        minute: u8,
    },
    /// 显示成就
    Achievements,
    /// 导出学习数据
    Export,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // 确保数据库存在
    db::init_db()?;

    match cli.command {
        Commands::Dashboard { path } => {
            ui::show_dashboard(&path)?;
        }
        Commands::Update { module, task } => {
            let repo = repo::LearningRepo::new(".")?;
            progress::update_task_status(&repo, &module, &task)?;
            println!("✅ 已更新 {} 的 {} 任务状态", module, task);
            ui::show_encouragement();
        }
        Commands::Practice { module, count } => {
            let repo = repo::LearningRepo::new(".")?;
            exercise::run_practice(&repo, &module, count)?;
        }
        Commands::Remind { hour, minute } => {
            notify::set_reminder(hour, minute)?;
            println!("⏰ 已设置提醒时间为 {:02}:{:02}", hour, minute);
            println!("💡 学习伴侣将在此时间提醒你学习");
        }
        Commands::Achievements => {
            ui::show_achievements()?;
        }
        Commands::Export => {
            storage::export_data()?;
        }
    }

    Ok(())
}
