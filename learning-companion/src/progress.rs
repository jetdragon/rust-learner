//! 进度计算模块
//!
//! 计算学习进度和掌握程度

use crate::repo::LearningRepo;
use anyhow::Result;

/// 任务类型
#[derive(Debug, Clone, Copy)]
pub enum TaskType {
    Concept,   // 概念学习
    Examples,  // 代码示例
    Exercises, // 练习题
    Project,   // 综合练习
    Checklist, // 自检通过
}

impl TaskType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "概念" | "concept" => Some(TaskType::Concept),
            "示例" | "examples" => Some(TaskType::Examples),
            "练习" | "exercises" => Some(TaskType::Exercises),
            "综合" | "project" => Some(TaskType::Project),
            "自检" | "checklist" => Some(TaskType::Checklist),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            TaskType::Concept => "概念学习",
            TaskType::Examples => "代码示例",
            TaskType::Exercises => "练习题完成",
            TaskType::Project => "综合练习",
            TaskType::Checklist => "自检通过",
        }
    }
}

/// 更新任务状态
pub fn update_task_status(repo: &LearningRepo, module_id: &str, task_str: &str) -> Result<()> {
    let task = TaskType::from_str(task_str);

    if let Some(task_type) = task {
        println!("✓ 标记 {} 的 {} 为已完成", module_id, task_type.as_str());

        // 更新数据库中的模块进度
        // 简化实现：每次更新增加掌握分数
        let increase = match task_type {
            TaskType::Concept => 15.0,
            TaskType::Examples => 15.0,
            TaskType::Exercises => 30.0,
            TaskType::Project => 30.0,
            TaskType::Checklist => 10.0,
        };

        // 获取当前进度并更新
        let current_score = crate::db::get_module_mastery(module_id).unwrap_or(0.0);
        let new_score = (current_score + increase).min(100.0);

        crate::db::update_module_progress(module_id, new_score)?;

        println!("📊 当前掌握程度：{:.1}%", new_score);

        if new_score >= 95.0 {
            println!("🎉 恭喜！你已掌握该模块，可以进入下一阶段学习！");
        } else if new_score >= 80.0 {
            println!("💪 做得不错！继续加油！");
        } else {
            println!("📚 继续学习，你可以的！");
        }

        return Ok(());
    }

    // 如果不是标准任务名，尝试匹配
    let task_lower = task_str.to_lowercase();
    if task_lower.contains("概念") || task_lower.contains("concept") {
        return update_task_status(repo, module_id, "concept");
    } else if task_lower.contains("示例") || task_lower.contains("example") {
        return update_task_status(repo, module_id, "examples");
    } else if task_lower.contains("练习") || task_lower.contains("exercise") {
        return update_task_status(repo, module_id, "exercises");
    } else if task_lower.contains("综合") || task_lower.contains("project") {
        return update_task_status(repo, module_id, "project");
    } else if task_lower.contains("自检") || task_lower.contains("checklist") {
        return update_task_status(repo, module_id, "checklist");
    }

    println!("❌ 未知的任务类型：{}", task_str);
    println!("💡 支持的任务类型：概念(concept)、示例(examples)、练习(exercises)、综合(project)、自检(checklist)");
    Err(anyhow::anyhow!("未知任务类型"))
}

/// 计算模块掌握程度（从数据库）
pub fn calculate_mastery(module_id: &str) -> f32 {
    crate::db::get_module_mastery(module_id).unwrap_or(0.0)
}

/// 检查是否可以进入下一模块
pub fn can_advance_to_next(module_id: &str) -> bool {
    let mastery = calculate_mastery(module_id);
    mastery >= 95.0
}

/// 获取学习建议
pub fn get_study_recommendations(module_id: &str) -> Vec<String> {
    let mastery = calculate_mastery(module_id);
    let mut recommendations = Vec::new();

    if mastery < 30.0 {
        recommendations.push("建议开始学习该模块的基础概念".to_string());
    } else if mastery < 60.0 {
        recommendations.push("建议完成代码示例的学习".to_string());
        recommendations.push("尝试做一些简单的练习".to_string());
    } else if mastery < 95.0 {
        recommendations.push("建议完成所有练习题".to_string());
        recommendations.push("完成综合练习".to_string());
        recommendations.push("做自检清单".to_string());
    } else {
        recommendations.push("恭喜！可以进入下一模块学习了".to_string());
    }

    recommendations
}
