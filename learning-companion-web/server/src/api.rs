use crate::AppState;
use anyhow::Result;
use axum::{extract::Path, extract::State, response::Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub async fn get_modules(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let project_path = std::path::Path::new(&state.project_path);

    let mut modules = Vec::new();

    if let Ok(entries) = std::fs::read_dir(project_path) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();

            if name.starts_with("module-") && entry.path().is_dir() {
                let module_path = entry.path();
                let has_readme = module_path.join("README.md").exists();
                let has_exercises = module_path.join("exercises.md").exists();
                let has_tests = module_path.join("tests").is_dir();
                let has_checklist = module_path.join("自检清单.md").exists();

                let display_name = extract_module_name(&name);
                modules.push(serde_json::json!({
                    "id": name.clone(),
                    "name": display_name,
                    "has_readme": has_readme,
                    "has_exercises": has_exercises,
                    "has_tests": has_tests,
                    "has_checklist": has_checklist,
                    "progress": 0.0,
                    "tasks": {
                        "concept": false,
                        "examples": false,
                        "exercises": false,
                        "project": false,
                        "checklist": false,
                    },
                }));
            }
        }
    }

    modules.sort_by(|a, b| a["id"].as_str().unwrap().cmp(&b["id"].as_str().unwrap()));
    Json(serde_json::Value::Array(modules))
}

#[derive(Deserialize)]
pub struct UpdateProgressBody {
    pub task_type: String,
}

pub async fn update_progress(
    Path(_module_id): Path<String>,
    State(_state): State<Arc<AppState>>,
    Json(body): Json<UpdateProgressBody>,
) -> Json<serde_json::Value> {
    let _increase = match body.task_type.as_str() {
        "concept" => 15.0,
        "examples" => 15.0,
        "exercises" => 30.0,
        "project" => 30.0,
        "checklist" => 10.0,
        _ => return Json(serde_json::json!({"error": "Invalid task type"})),
    };

    Json(serde_json::json!({
        "success": true,
        "mastery": 15.0
    }))
}

pub async fn get_practice_questions(Path(_module_id): Path<String>) -> Json<serde_json::Value> {
    let questions = match _module_id.as_str() {
        "module-01-basics" => generate_basics_questions(),
        _ => vec![],
    };

    Json(serde_json::json!({ "questions": questions }))
}

pub async fn submit_practice(
    Path(_module_id): Path<String>,
    Json(body): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
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
        .filter(|(ans, q)| ans.to_string() == q["correct_answer"].as_str().unwrap())
        .count();

    let score = (correct_count as f32 / questions.len() as f32) * 100.0;

    Json(serde_json::json!({
        "score": score,
        "correct_count": correct_count,
        "total_count": questions.len(),
    }))
}

pub async fn get_achievements() -> Json<Vec<serde_json::Value>> {
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
                "unlocked": false,
            })
        })
        .collect();

    Json(result)
}

pub async fn export_data() -> Json<serde_json::Value> {
    let achievements = vec![
        ("first_steps", "初次学习 - 完成第一个模块"),
        ("week_warrior", "坚持一周 - 连续学习 7 天"),
        ("month_master", "坚持一月 - 连续学习 30 天"),
        ("practice_perfect", "练习达人 - 单次练习 100% 正确"),
        ("half_way", "半程高手 - 完成 50% 的学习内容"),
        ("completionist", "学习大师 - 完成所有模块"),
    ];

    let achievements_json: Vec<serde_json::Value> = achievements
        .iter()
        .map(|(name, desc)| {
            serde_json::json!({
                "name": name,
                "description": "",
                "unlocked": true,
            })
        })
        .collect();

    let export_data: serde_json::Value = serde_json::json!({
        "modules": Vec::<serde_json::Value>::new(),
        "achievements": achievements_json,
        "export_date": "2024-01-18T12:52:53.123456Z",
    });

    Json(export_data)
}

fn extract_module_name(id: &str) -> String {
    let names = vec![
        ("module-01-basics", "01-基础入门"),
        ("module-02-ownership", "02-所有权系统"),
        ("module-03-structs-enums", "03-结构体与枚举"),
        ("module-04-lifetimes", "04-生命周期"),
        ("module-05-patterns", "05-模式匹配"),
        ("module-06-error-handling", "06-错误处理"),
        ("module-07-collections", "07-集合类型"),
        ("module-08-traits-generics", "08-Trait与泛型"),
        ("module-09-concurrency", "09-并发编程"),
        ("module-10-project", "10-实战项目"),
        ("module-11-smart-pointers", "11-智能指针"),
        ("module-12-iterators", "12-迭代器"),
    ];

    for (id_pattern, name) in names {
        if id == id_pattern {
            return name.to_string();
        }
    }
    id.to_string()
}

fn generate_basics_questions() -> Vec<serde_json::Value> {
    vec![
        serde_json::json!({
            "id": 1,
            "question": "Rust 中，使用 let 声明的变量默认是什么特性？",
            "options": ["可变的", "不可变的", "动态的", "静态的"],
            "correct_answer": "1",
        }),
        serde_json::json!({
            "id": 2,
            "question": "如何声明一个可变的变量？",
            "options": ["let mut", "let var", "mut let", "let const"],
            "correct_answer": "0",
        }),
        serde_json::json!({
            "id": 3,
            "question": "Rust 中默认的整数类型是什么？",
            "options": ["i8", "i32", "i64", "isize"],
            "correct_answer": "1",
        }),
        serde_json::json!({
            "id": 4,
            "question": "变量遮蔽（shadowing）是指什么？",
            "options": ["隐藏外部作用域的变量", "删除变量", "复制变量", "移动变量"],
            "correct_answer": "0",
        }),
        serde_json::json!({
            "id": 5,
            "question": "Rust 中字符串类型 String 和 &str 的主要区别是什么？",
            "options": ["没有区别", "String 是拥有的，&str 是借用的", "&str 是拥有的，String 是借用的", "String 只能读，&str 只能写"],
            "correct_answer": "1",
        }),
    ]
}
