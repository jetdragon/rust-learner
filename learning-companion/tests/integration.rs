//! 集成测试 - 测试 TUI 核心功能
//!
//! 由于 TUI 需要交互式终端，这里测试核心逻辑而非 UI 渲染

use learning_companion::repo::LearningRepo;
use learning_companion::progress::{TaskType, calculate_mastery};
use learning_companion::db;

#[test]
fn test_learning_repo_scans_modules() {
    // 测试仓库扫描功能
    let temp_dir = std::env::current_dir().unwrap();
    // 使用父目录（包含 module-XX-* 目录）
    let parent_dir = temp_dir.parent().unwrap_or(&temp_dir);
    let repo = LearningRepo::new(parent_dir).expect("无法创建学习仓库");

    // 验证找到了模块
    assert!(!repo.modules.is_empty(), "应该找到至少一个学习模块");

    // 验证第一个模块的基本信息
    let first_module = &repo.modules[0];
    assert!(!first_module.name.is_empty());
    assert!(!first_module.id.is_empty());

    println!("找到 {} 个模块", repo.modules.len());
    for module in &repo.modules {
        println!("  - {} ({})", module.name, module.id);
    }
}

#[test]
fn test_module_progress_structure() {
    // 测试模块进度结构
    let temp_dir = std::env::current_dir().unwrap();
    let parent_dir = temp_dir.parent().unwrap_or(&temp_dir);
    let repo = LearningRepo::new(parent_dir).expect("无法创建学习仓库");

    if let Some(progress) = repo.get_module_progress(&repo.modules[0].id) {
        // 验证进度字段
        // ModuleProgress 应该包含 5 个任务状态
        let _tasks = [progress.concept, progress.examples, progress.exercises, progress.project, progress.checklist];
        println!("任务状态: 概念={}, 示例={}, 练习={}, 综合={}, 自检={}",
            progress.concept, progress.examples, progress.exercises, progress.project, progress.checklist);
    }
}

#[test]
fn test_task_type_parsing() {
    // 测试任务类型解析
    assert!(TaskType::from_str("concept").is_some());
    assert!(TaskType::from_str("概念").is_some());
    assert!(TaskType::from_str("examples").is_some());
    assert!(TaskType::from_str("示例").is_some());
    assert!(TaskType::from_str("exercises").is_some());
    assert!(TaskType::from_str("练习").is_some());
    assert!(TaskType::from_str("project").is_some());
    assert!(TaskType::from_str("综合").is_some());
    assert!(TaskType::from_str("checklist").is_some());
    assert!(TaskType::from_str("自检").is_some());

    // 测试无效输入
    assert!(TaskType::from_str("invalid").is_none());
}

#[test]
fn test_completion_percentage() {
    // 测试完成度计算
    let temp_dir = std::env::current_dir().unwrap();
    let parent_dir = temp_dir.parent().unwrap_or(&temp_dir);
    let repo = LearningRepo::new(parent_dir).expect("无法创建学习仓库");

    let completion = repo.completion_percentage();
    assert!(completion >= 0.0 && completion <= 100.0, "完成度应在 0-100 之间");

    println!("总体完成度: {:.1}%", completion);
}

#[test]
fn test_mastery_calculation() {
    // 测试掌握程度计算
    let module_id = "test-module";

    // 初始掌握程度应该是 0
    let mastery = calculate_mastery(module_id);
    assert_eq!(mastery, 0.0, "新模块的掌握程度应该是 0");
}

#[test]
fn test_achievement_system() {
    // 测试成就系统
    let achievements = db::get_all_achievements().unwrap();

    // 应该有预定义的成就
    assert_eq!(achievements.len(), 6, "应该有 6 个预定义成就");

    // 验证每个成就的结构
    for achievement in &achievements {
        assert!(!achievement.name.is_empty());
        assert!(!achievement.description.is_empty());
        // 新数据库的成就应该未解锁
    }

    println!("成就系统测试完成，共 {} 个成就", achievements.len());
}

#[test]
fn test_database_initialization() {
    // 测试数据库初始化
    db::init_db().unwrap();

    // 验证可以获取成就（间接验证数据库可用）
    let achievements = db::get_all_achievements().unwrap();
    assert!(!achievements.is_empty());
}

#[test]
fn test_app_state_transitions() {
    // TUI 状态测试移到单元测试中，这里跳过
    // 因为 tui 模块只在库的测试模式下暴露
    println!("应用状态转换测试跳过（TUI 状态在单元测试中验证）");
}

#[test]
fn test_question_generation() {
    // 测试练习题生成
    use learning_companion::exercise::{Question, QuestionType};

    // 创建一个测试问题
    let question = Question {
        question_type: QuestionType::MultipleChoice,
        prompt: "Rust 中哪个关键字用于声明不可变变量？".to_string(),
        options: Some(vec![
            "let".to_string(),
            "mut".to_string(),
            "const".to_string(),
            "var".to_string(),
        ]),
        correct_answer: "0".to_string(),
        explanation: "let 用于声明不可变变量".to_string(),
        topic: "变量声明".to_string(),
    };

    assert_eq!(question.options.as_ref().unwrap().len(), 4);
    assert_eq!(question.correct_answer, "0");
    assert!(!question.explanation.is_empty());
}
