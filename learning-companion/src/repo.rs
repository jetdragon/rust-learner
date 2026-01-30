//! 仓库扫描和解析模块
//!
//! 扫描 Rust 学习仓库，解析进度文件和模块结构

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// 学习模块信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningModule {
    pub id: String,
    pub name: String,
    pub language: String, // 语言标识: "rust", "python", "go"
    pub directory: PathBuf,
    pub has_readme: bool,
    pub has_exercises: bool,
    pub has_tests: bool,
    pub has_checklist: bool,
}

/// 进度文件中的模块状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleProgress {
    pub name: String,
    pub status: String, // "[ ]", "[~]", or "[x]"
    pub concept: bool,
    pub examples: bool,
    pub exercises: bool,
    pub project: bool,
    pub checklist: bool,
}

/// 学习仓库
pub struct LearningRepo {
    pub path: PathBuf,
    pub modules: Vec<LearningModule>,
    pub progress: Vec<ModuleProgress>,
}

impl LearningRepo {
    /// 扫描学习仓库
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().canonicalize()?;
        let modules = Self::scan_modules(&path)?;
        let progress = Self::parse_progress_file(&path)?;

        Ok(Self {
            path: path.to_path_buf(),
            modules,
            progress,
        })
    }

    /// 扫描所有学习模块
    fn scan_modules(base_path: &Path) -> Result<Vec<LearningModule>> {
        let mut modules = Vec::new();

        // 支持多语言目录结构：rust/, python/, go/
        let language_dirs = ["rust", "python", "go"];

        for lang in &language_dirs {
            let lang_path = base_path.join(lang);

            // 如果语言目录不存在，跳过
            if !lang_path.exists() || !lang_path.is_dir() {
                continue;
            }

            // 扫描该语言目录下的所有 module-XX-* 子目录
            for entry in fs::read_dir(&lang_path)? {
                let entry = entry?;
                let name = entry.file_name().to_string_lossy().to_string();

                // 识别模块目录 (module-XX-*)
                if name.starts_with("module-") && entry.path().is_dir() {
                    let module_path = entry.path();

                    let module = LearningModule {
                        id: name.clone(),
                        name: Self::extract_module_name(&name),
                        language: lang.to_string(), // 添加语言标识
                        directory: module_path.clone(),
                        has_readme: module_path.join("README.md").exists(),
                        has_exercises: module_path.join("exercises.md").exists(),
                        has_tests: module_path.join("tests").exists(),
                        has_checklist: module_path.join("自检清单.md").exists(),
                    };

                    modules.push(module);
                }
            }
        }

        // 按语言和模块编号排序
        modules.sort_by(|a, b| match a.language.cmp(&b.language) {
            std::cmp::Ordering::Equal => a.id.cmp(&b.id),
            other => other,
        });

        Ok(modules)
    }

    /// 从目录名提取模块中文名
    fn extract_module_name(id: &str) -> String {
        // 从 module-01-basics 提取并映射到中文名
        let names = vec![
            ("module-01-basics", "01-基础入门"),
            ("module-02-ownership", "02-所有权系统"),
            ("module-03-structs-enums", "03-结构体与枚举"),
            ("module-04-patterns", "04-模式匹配"),
            ("module-05-error-handling", "05-错误处理"),
            ("module-06-collections", "06-集合类型"),
            ("module-07-generics", "07-泛型与Trait"),
            ("module-08-lifetimes", "08-生命周期"),
            ("module-09-concurrency", "09-并发编程"),
            ("module-10-project", "10-实战项目"),
        ];

        for (id_pattern, name) in names {
            if id == id_pattern {
                return name.to_string();
            }
        }

        id.to_string()
    }

    /// 解析 进度.md 文件
    fn parse_progress_file(base_path: &Path) -> Result<Vec<ModuleProgress>> {
        let progress_path = base_path.join("进度.md");
        if !progress_path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&progress_path)?;
        let mut progress_list = Vec::new();

        // 简单解析（实际可以用正则表达式更精确）
        let lines: Vec<&str> = content.lines().collect();
        let mut current_module: Option<ModuleProgress> = None;

        for line in lines {
            // 检测模块标题
            if line.contains("### ")
                && (line.contains("-基础入门")
                    || line.contains("-所有权系统")
                    || line.contains("-结构体")
                    || line.contains("-模式匹配")
                    || line.contains("-错误处理")
                    || line.contains("-集合类型")
                    || line.contains("-泛型")
                    || line.contains("-生命周期")
                    || line.contains("-并发编程")
                    || line.contains("-实战项目"))
            {
                if let Some(module) = current_module.take() {
                    progress_list.push(module);
                }

                // 提取模块名
                let name = line.split("### ").nth(1).unwrap_or("").trim().to_string();
                current_module = Some(ModuleProgress {
                    name: name.clone(),
                    status: "[ ]".to_string(),
                    concept: false,
                    examples: false,
                    exercises: false,
                    project: false,
                    checklist: false,
                });
            }

            // 解析任务状态
            if let Some(ref mut module) = current_module {
                if line.contains("- [x] 概念学习") || line.contains("- [x] 代码示例") {
                    module.concept = true;
                    module.examples = true;
                    module.status = "[~]".to_string();
                }
                if line.contains("- [x] 练习题完成") {
                    module.exercises = true;
                }
                if line.contains("- [x] 综合练习") {
                    module.project = true;
                }
                if line.contains("- [x] 自检通过") {
                    module.checklist = true;
                    module.status = "[x]".to_string();
                }
            }
        }

        if let Some(module) = current_module {
            progress_list.push(module);
        }

        Ok(progress_list)
    }

    /// 计算总体完成百分比
    pub fn completion_percentage(&self) -> f32 {
        if self.modules.is_empty() {
            return 0.0;
        }

        let completed = self.progress.iter().filter(|p| p.status == "[x]").count();

        (completed as f32 / self.modules.len() as f32) * 100.0
    }

    /// 获取模块进度
    pub fn get_module_progress(&self, module_name: &str) -> Option<&ModuleProgress> {
        self.progress.iter().find(|p| p.name.contains(module_name))
    }
}
