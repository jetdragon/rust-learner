//! 用户界面模块
//!
//! 提供命令行界面和可视化展示

use crate::repo::LearningRepo;
use anyhow::Result;
use crate::db;
use rand::Rng;

/// 显示仪表板
pub fn show_dashboard(path: &str) -> Result<()> {
    let repo = LearningRepo::new(path)?;

    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║             🦀 Rust 学习伴侣 - 学习仪表板                      ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    // 总体进度
    let completion = repo.completion_percentage();
    let completed = repo.progress.iter().filter(|p| p.status == "[x]").count();
    let total = repo.modules.len();

    println!("📊 总体进度：{:.1}% ({}/{})\n", completion, completed, total);

    // 进度条
    let filled = (completion / 5.0) as usize;
    let empty = 20 - filled;
    print!("  ");
    for _ in 0..filled {
        print!("█");
    }
    for _ in 0..empty {
        print!("░");
    }
    println!(" {}\n", completion);

    // 连续学习天数
    let streak = db::get_streak_days().unwrap_or(0);
    if streak > 0 {
        println!("🔥 连续学习：{} 天\n", streak);
    }

    // 模块列表
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📚 模块状态");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    for module in &repo.modules {
        let progress = repo.get_module_progress(&module.id);
        let mastery = db::get_module_mastery(&module.id).unwrap_or(0.0);

        // 状态图标
        let status_icon = match progress {
            Some(p) if p.status == "[x]" => "✅",
            Some(p) if p.status == "[~]" => "🟡",
            _ => "⬜",
        };

        println!("{} {} - {}", status_icon, module.name, module.id);

        // 详细状态
        if let Some(p) = progress {
            print!("   任务：");
            let tasks = vec![
                (p.concept, "概念"),
                (p.examples, "示例"),
                (p.exercises, "练习"),
                (p.project, "综合"),
                (p.checklist, "自检"),
            ];

            for (done, name) in tasks {
                let icon = if done { "✓" } else { "○" };
                print!("{}{} ", icon, name);
            }
            println!();
        }

        // 掌握程度
        if mastery > 0.0 {
            print!("   掌握：");
            let filled = (mastery / 5.0) as usize;
            for _ in 0..filled {
                print!("█");
            }
            for _ in 0..(20 - filled) {
                print!("░");
            }
            println!(" {:.1}%", mastery);

            if mastery >= 95.0 {
                println!("   🎉 已掌握，可以进入下一模块！");
            }
        }

        println!();
    }

    // 学习建议
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("💡 学习建议");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let current_module = repo.modules.first();
    if let Some(module) = current_module {
        let recommendations = crate::progress::get_study_recommendations(&module.id);
        for rec in recommendations {
            println!("  • {}", rec);
        }
    }

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📖 常用命令");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    println!("  learning-companion update -m <模块> -t <任务>  更新学习进度");
    println!("  learning-companion practice -m <模块>         开始练习测试");
    println!("  learning-companion remind -H <时> -M <分>     设置学习提醒");
    println!("  learning-companion achievements               查看成就");
    println!("  learning-companion export                      导出学习数据");
    println!();

    Ok(())
}

/// 显示成就
pub fn show_achievements() -> Result<()> {
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║                   🏆 我的成就                                 ║");
    println!("╚══════════════════════════════════━━━━━━━━━━━━━━━━━━━━━━━━━━━╝\n");

    let achievements = db::get_all_achievements()?;

    if achievements.is_empty() {
        println!("  还没有解锁任何成就，继续加油！\n");
        return Ok(());
    }

    for achievement in &achievements {
        let icon = match achievement.achievement_type.as_str() {
            "first_module" => "🎓",
            "streak_7" => "🔥",
            "streak_30" => "⚡",
            "perfect_score" => "💯",
            "code_quality" => "🌟",
            _ => "🏅",
        };

        let name = match achievement.achievement_type.as_str() {
            "first_module" => "初学者",
            "streak_7" => "坚持一周",
            "streak_30" => "坚持一个月",
            "perfect_score" => "完美主义者",
            "code_quality" => "代码质量大师",
            _ => "未知成就",
        };

        println!("  {} {} - 解锁于 {}", icon, name,
            achievement.unlocked_at.format("%Y-%m-%d"));
    }

    println!();

    // 显示待解锁成就
    let locked = vec![
        ("first_module", "🎓 初学者 - 完成第一个模块"),
        ("streak_7", "🔥 坚持一周 - 连续学习 7 天"),
        ("streak_30", "⚡ 坚持一个月 - 连续学习 30 天"),
        ("perfect_score", "💯 完美主义者 - 练习得 100%"),
        ("code_quality", "🌟 代码质量大师 - 通过 clippy 检查"),
    ];

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🔒 待解锁成就");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let unlocked_types: std::collections::HashSet<String> =
        achievements.iter().map(|a| a.achievement_type.clone()).collect();

    for (id, desc) in locked {
        if !unlocked_types.contains(id) {
            println!("  🔒 {}", desc);
        }
    }

    println!();

    Ok(())
}

/// 显示鼓励消息
pub fn show_encouragement() {
    let messages = vec![
        "太棒了！继续加油！💪",
        "你做得很好！保持这个节奏！🌟",
        "每一步都是进步，继续前进！🚀",
        "你正在变得越来越强！⭐",
        "坚持就是胜利！🏆",
        "相信自己的能力！💫",
        "学习之路，步步为营！📚",
        "今天的努力，明天的收获！🌱",
    ];

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..messages.len());

    println!("\n  {}\n", messages[index]);
}
