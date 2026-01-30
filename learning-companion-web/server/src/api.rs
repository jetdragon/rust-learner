use crate::AppState;
use axum::{extract::Path, extract::State, response::Json};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;

// Import ModuleState from main
use crate::ModuleState;

pub async fn get_modules(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let project_path = std::path::Path::new(&state.project_path);

    let mut modules = Vec::new();

    // 支持多语言目录结构：rust/, python/, go/
    let language_dirs = ["rust", "python", "go"];

    for lang in &language_dirs {
        let lang_path = project_path.join(lang);

        // 如果语言目录不存在，跳过
        if !lang_path.exists() || !lang_path.is_dir() {
            continue;
        }

        // 扫描该语言目录下的所有 module-XX-* 子目录
        if let Ok(entries) = std::fs::read_dir(&lang_path) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();

                if name.starts_with("module-") && entry.path().is_dir() {
                    let module_path = entry.path();
                    let has_readme = module_path.join("README.md").exists();
                    let has_exercises = module_path.join("exercises.md").exists();
                    let has_tests = module_path.join("tests").is_dir();
                    let has_checklist = module_path.join("自检清单.md").exists();

                    let display_name = extract_module_name(&name);

                    // Get or create module state
                    let module_state = {
                        let mut states = state.module_states.lock().await;
                        states.get(&name).cloned().unwrap_or_else(|| {
                            // Initialize with empty tasks
                            let mut tasks_completed = std::collections::HashMap::new();
                            tasks_completed.insert("concept".to_string(), false);
                            tasks_completed.insert("examples".to_string(), false);
                            tasks_completed.insert("exercises".to_string(), false);
                            tasks_completed.insert("project".to_string(), false);
                            tasks_completed.insert("checklist".to_string(), false);

                            ModuleState {
                                progress: 0.0,
                                tasks_completed,
                            }
                        })
                    };

                    modules.push(serde_json::json!({
                        "id": name.clone(),
                        "name": display_name,
                        "language": lang,  // 添加语言标识
                        "has_readme": has_readme,
                        "has_exercises": has_exercises,
                        "has_tests": has_tests,
                        "has_checklist": has_checklist,
                        "progress": module_state.progress,
                        "tasks": {
                            "concept": module_state.tasks_completed.get("concept").copied().unwrap_or(false),
                            "examples": module_state.tasks_completed.get("examples").copied().unwrap_or(false),
                            "exercises": module_state.tasks_completed.get("exercises").copied().unwrap_or(false),
                            "project": module_state.tasks_completed.get("project").copied().unwrap_or(false),
                            "checklist": module_state.tasks_completed.get("checklist").copied().unwrap_or(false),
                        },
                    }));
                }
            }
        }
    }

    // 按语言和模块编号排序
    modules.sort_by(|a, b| {
        let lang_a = a["language"].as_str().unwrap_or("");
        let lang_b = b["language"].as_str().unwrap_or("");
        match lang_a.cmp(&lang_b) {
            std::cmp::Ordering::Equal => a["id"]
                .as_str()
                .unwrap_or("")
                .cmp(&b["id"].as_str().unwrap_or("")),
            other => other,
        }
    });

    Json(serde_json::Value::Array(modules))
}

#[derive(Deserialize)]
pub struct UpdateProgressBody {
    pub task_type: String,
}

pub async fn update_progress(
    Path(module_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<UpdateProgressBody>,
) -> Json<serde_json::Value> {
    let increase = match body.task_type.as_str() {
        "concept" => 15.0,
        "examples" => 15.0,
        "exercises" => 30.0,
        "project" => 30.0,
        "checklist" => 10.0,
        _ => return Json(serde_json::json!({"error": "Invalid task type"})),
    };

    // Update module state
    let mut states = state.module_states.lock().await;
    let module_state = states.entry(module_id.clone()).or_insert_with(|| {
        // Initialize with empty tasks
        let mut tasks_completed = std::collections::HashMap::new();
        tasks_completed.insert("concept".to_string(), false);
        tasks_completed.insert("examples".to_string(), false);
        tasks_completed.insert("exercises".to_string(), false);
        tasks_completed.insert("project".to_string(), false);
        tasks_completed.insert("checklist".to_string(), false);

        ModuleState {
            progress: 0.0,
            tasks_completed,
        }
    });

    // Mark task as completed
    module_state
        .tasks_completed
        .insert(body.task_type.clone(), true);

    // Calculate progress based on completed tasks
    let completed_tasks = module_state
        .tasks_completed
        .values()
        .filter(|&&v| v)
        .count();
    let total_tasks = module_state.tasks_completed.len();
    module_state.progress = (completed_tasks as f32 / total_tasks as f32) * 100.0;

    Json(serde_json::json!({
        "success": true,
        "mastery": module_state.progress
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

pub async fn get_module_content(
    Path((module_id, content_type)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let project_path = std::path::Path::new(&state.project_path);
    let module_path = project_path.join(&module_id);

    if !module_path.exists() || !module_path.is_dir() {
        return Json(serde_json::json!({
            "error": "Module not found",
            "module_id": module_id
        }));
    }

    let content = match content_type.as_str() {
        "readme" => {
            let readme_path = module_path.join("README.md");
            if readme_path.exists() {
                std::fs::read_to_string(readme_path)
                    .unwrap_or_else(|_| "# No README content found".to_string())
            } else {
                "# No README.md file found for this module".to_string()
            }
        }
        "exercises" => {
            let exercises_path = module_path.join("exercises.md");
            if exercises_path.exists() {
                std::fs::read_to_string(exercises_path)
                    .unwrap_or_else(|_| "# No exercises content found".to_string())
            } else {
                "# No exercises.md file found for this module".to_string()
            }
        }
        "project" => {
            // For project, return a description
            format!(
                "# 综合练习 - {}\n\n完成本模块的综合练习项目，巩固所学知识。\n\n查看项目文件并开始实践！",
                extract_module_name(&module_id)
            )
        }
        _ => {
            return Json(serde_json::json!({
                "error": "Invalid content type",
                "valid_types": ["readme", "exercises", "project"]
            }));
        }
    };

    Json(serde_json::json!({
        "module_id": module_id,
        "content_type": content_type,
        "content": content
    }))
}

pub async fn list_examples(
    Path(module_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let project_path = std::path::Path::new(&state.project_path);
    let module_path = project_path.join(&module_id);
    let examples_dir = module_path.join("examples");

    if !examples_dir.exists() || !examples_dir.is_dir() {
        return Json(serde_json::json!({
            "module_id": module_id,
            "examples": Vec::<String>::new()
        }));
    }

    let mut examples = Vec::new();
    if let Ok(entries) = std::fs::read_dir(examples_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
                if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                    examples.push(filename.to_string());
                }
            }
        }
    }

    examples.sort();

    Json(serde_json::json!({
        "module_id": module_id,
        "examples": examples
    }))
}

pub async fn get_example_content(
    Path((module_id, filename)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let project_path = std::path::Path::new(&state.project_path);
    let module_path = project_path.join(&module_id);
    let examples_dir = module_path.join("examples");
    let file_path = examples_dir.join(&filename);

    if !file_path.exists() || !file_path.is_file() {
        return Json(serde_json::json!({
            "error": "Example file not found",
            "module_id": module_id,
            "filename": filename
        }));
    }

    let content = std::fs::read_to_string(&file_path)
        .unwrap_or_else(|_| "// Could not read file content".to_string());

    Json(serde_json::json!({
        "module_id": module_id,
        "filename": filename,
        "content": content
    }))
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
