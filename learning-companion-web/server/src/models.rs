use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningModule {
    pub id: String,
    pub name: String,
    pub has_readme: bool,
    pub has_exercises: bool,
    pub has_tests: bool,
    pub has_checklist: bool,
    pub progress: f32,
    pub tasks: ModuleTasks,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleTasks {
    pub concept: bool,
    pub examples: bool,
    pub exercises: bool,
    pub project: bool,
    pub checklist: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProgressRequest {
    pub task_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PracticeQuestion {
    pub id: usize,
    pub question: String,
    pub options: Vec<String>,
    pub correct_answer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PracticeSession {
    pub module_id: String,
    pub questions: Vec<PracticeQuestion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PracticeSubmission {
    pub module_id: String,
    pub answers: Vec<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PracticeResult {
    pub score: f32,
    pub correct_count: usize,
    pub total_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub name: String,
    pub description: String,
    pub unlocked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportData {
    pub modules: Vec<LearningModule>,
    pub achievements: Vec<Achievement>,
    pub export_date: String,
}
