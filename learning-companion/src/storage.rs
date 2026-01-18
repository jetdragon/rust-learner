//! 存储模块
//!
//! 数据导出和持久化

use anyhow::Result;
use chrono::Local;

/// 导出学习数据为 JSON
pub fn export_data() -> Result<()> {
    let data = crate::db::export_all_data()?;

    let filename = format!(
        "learning-companion-export-{}.json",
        Local::now().format("%Y%m%d")
    );

    std::fs::write(&filename, data)?;

    println!("✅ 学习数据已导出到：{}", filename);

    Ok(())
}

/// 导出学习报告为 Markdown
pub fn export_report() -> Result<String> {
    let mut report = String::new();

    report.push_str("# Rust 学习报告\n\n");
    report.push_str(&format!(
        "生成时间：{}\n\n",
        Local::now().format("%Y-%m-%d %H:%M")
    ));

    // 从数据库获取数据并格式化
    let data = crate::db::export_all_data()?;
    report.push_str(&data);

    Ok(report)
}
