use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::db;
use crate::models::{ExportData, LearningModule, ModuleTasks, PracticeQuestion, PracticeResult};

pub struct AppState {
    pub project_path: String,
}

pub async fn get_modules(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match learning_companion::repo::LearningRepo::new(&state.project_path) {
        Ok(repo) => {
            let modules: Vec<LearningModule> = repo
                .modules
                .iter()
                .map(|m| {
                    let progress = repo.get_module_progress(&m.id);
                    let mastery = db::get_module_mastery(&m.id).unwrap_or(0.0);

                    LearningModule {
                        id: m.id.clone(),
                        name: m.name.clone(),
                        has_readme: m.has_readme,
                        has_exercises: m.has_exercises,
                        has_tests: m.has_tests,
                        has_checklist: m.has_checklist,
                        progress: mastery,
                        tasks: ModuleTasks {
                            concept: progress.map(|p| p.concept).unwrap_or(false),
                            examples: progress.map(|p| p.examples).unwrap_or(false),
                            exercises: progress.map(|p| p.exercises).unwrap_or(false),
                            project: progress.map(|p| p.project).unwrap_or(false),
                            checklist: progress.map(|p| p.checklist).unwrap_or(false),
                        },
                    }
                })
                .collect();

            Json(modules)
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

#[derive(Deserialize)]
pub struct UpdateProgressBody {
    pub task_type: String,
}

pub async fn update_progress(
    Path(module_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<UpdateProgressBody>,
) -> impl IntoResponse {
    let increase = match body.task_type.as_str() {
        "concept" => 15.0,
        "examples" => 15.0,
        "exercises" => 30.0,
        "project" => 30.0,
        "checklist" => 10.0,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "error": "Invalid task type" })),
            )
                .into_response()
        }
    };

    let current = db::get_module_mastery(&module_id).unwrap_or(0.0);
    let new_score = (current + increase).min(100.0);

    match db::update_module_progress(&module_id, new_score) {
        Ok(_) => {
            if new_score >= 95.0 && module_id == "module-01-basics" {
                let _ = db::check_and_unlock_achievement("first_steps");
            }
            Json(serde_json::json!({
                "success": true,
                "mastery": new_score
            }))
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

pub async fn get_practice_questions(Path(module_id): Path<String>) -> impl IntoResponse {
    let questions = match module_id.as_str() {
        "module-01-basics" => generate_basics_questions(),
        _ => vec![],
    };

    Json(serde_json::json!({ "questions": questions }))
}

pub async fn submit_practice(
    Path(module_id): Path<String>,
    Json(body): Json<serde_json::Value>,
) -> impl IntoResponse {
    let answers: Vec<usize> = body["answers"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_u64())
                .map(|u| u as usize)
                .collect()
        })
        .unwrap_or_default();

    let questions = generate_basics_questions();
    let correct_count = answers
        .iter()
        .zip(questions.iter())
        .filter(|(ans, q)| ans.to_string() == q.correct_answer)
        .count();

    let score = (correct_count as f32 / questions.len() as f32) * 100.0;

    let _ = db::record_practice_result(
        &module_id,
        questions.len() as u32,
        correct_count as u32,
        score,
    );

    if score == 100.0 {
        let _ = db::check_and_unlock_achievement("practice_perfect");
    }

    Json(serde_json::json!(PracticeResult {
        score,
        correct_count,
        total_count: questions.len(),
    }))
}

pub async fn get_achievements() -> impl IntoResponse {
    let unlocked = db::get_all_achievements().unwrap_or_default();

    let achievements = vec![
        ("first_steps", "初次学习 - 完成第一个模块"),
        ("week_warrior", "坚持一周 - 连续学习 7 天"),
        ("month_master", "坚持一月 - 连续学习 30 天"),
        ("practice_perfect", "练习达人 - 单次练习 100% 正确"),
        ("half_way", "半程高手 - 完成 50% 的学习内容"),
        ("completionist", "学习大师 - 完成所有模块"),
    ];

    let result: Vec<serde_json::Value> = achievements
        .iter()
        .map(|(name, desc)| {
            serde_json::json!({
                "name": name,
                "description": desc,
                "unlocked": unlocked.contains(&name.to_string())
            })
        })
        .collect();

    Json(result)
}

pub async fn export_data(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let _repo = learning_companion::repo::LearningRepo::new(&state.project_path);
    let achievements = db::get_all_achievements().unwrap_or_default();

    let export_data = ExportData {
        modules: vec![],
        achievements: achievements
            .iter()
            .map(|name| crate::models::Achievement {
                name: name.clone(),
                description: String::new(),
                unlocked: true,
            })
            .collect(),
        export_date: chrono::Local::now().to_rfc3339(),
    };

    Json(export_data)
}

fn generate_basics_questions() -> Vec<PracticeQuestion> {
    vec![
        PracticeQuestion {
            id: 1,
            question: "Rust 中，使用 let 声明的变量默认是什么特性？".to_string(),
            options: vec![
                "可变的".to_string(),
                "不可变的".to_string(),
                "动态的".to_string(),
                "静态的".to_string(),
            ],
            correct_answer: "1".to_string(),
        },
        PracticeQuestion {
            id: 2,
            question: "如何声明一个可变的变量？".to_string(),
            options: vec![
                "let mut".to_string(),
                "let var".to_string(),
                "mut let".to_string(),
                "let const".to_string(),
            ],
            correct_answer: "0".to_string(),
        },
        PracticeQuestion {
            id: 3,
            question: "Rust 中默认的整数类型是什么？".to_string(),
            options: vec![
                "i8".to_string(),
                "i32".to_string(),
                "i64".to_string(),
                "isize".to_string(),
            ],
            correct_answer: "1".to_string(),
        },
        PracticeQuestion {
            id: 4,
            question: "变量遮蔽（shadowing）是指什么？".to_string(),
            options: vec![
                "隐藏外部作用域的变量".to_string(),
                "删除变量".to_string(),
                "复制变量".to_string(),
                "移动变量".to_string(),
            ],
            correct_answer: "0".to_string(),
        },
        PracticeQuestion {
            id: 5,
            question: "Rust 中字符串类型 String 和 &str 的主要区别是什么？".to_string(),
            options: vec![
                "没有区别".to_string(),
                "String 是拥有的，&str 是借用的".to_string(),
                "&str 是拥有的，String 是借用的".to_string(),
                "String 只能读，&str 只能写".to_string(),
            ],
            correct_answer: "1".to_string(),
        },
    ]
}
