//! Todo 任务结构体定义

use crate::Priority;
use chrono::{DateTime, Duration, Utc};

/// 待办事项任务
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Todo {
    /// 唯一标识符
    pub id: u32,
    /// 任务标题
    pub title: String,
    /// 可选的任务描述
    pub description: Option<String>,
    /// 完成状态
    pub completed: bool,
    /// 优先级
    pub priority: Priority,
    /// 标签列表
    pub tags: Vec<String>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 完成时间
    pub completed_at: Option<DateTime<Utc>>,
}

impl Todo {
    /// 创建新任务
    pub fn new(id: u32, title: String, priority: Priority) -> Self {
        Self {
            id,
            title,
            description: None,
            completed: false,
            priority,
            tags: Vec::new(),
            created_at: Utc::now(),
            completed_at: None,
        }
    }

    /// 设置描述
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// 设置标签
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    /// 标记任务为完成
    pub fn complete(&mut self) {
        self.completed = true;
        self.completed_at = Some(Utc::now());
    }

    /// 取消完成状态
    pub fn uncomplete(&mut self) {
        self.completed = false;
        self.completed_at = None;
    }

    /// 添加标签
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    /// 移除标签
    pub fn remove_tag(&mut self, tag: &str) -> bool {
        if let Some(pos) = self.tags.iter().position(|t| t == tag) {
            self.tags.remove(pos);
            true
        } else {
            false
        }
    }

    /// 检查是否包含标签
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }

    /// 返回任务创建至今的时间
    pub fn age(&self) -> Duration {
        Utc::now() - self.created_at
    }

    /// 检查是否过期（当前未实现，返回 false）
    pub fn is_overdue(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_todo() {
        let todo = Todo::new(1, String::from("测试"), Priority::High);
        assert_eq!(todo.id, 1);
        assert_eq!(todo.title, "测试");
        assert_eq!(todo.priority, Priority::High);
        assert!(!todo.completed);
        assert!(todo.description.is_none());
        assert!(todo.tags.is_empty());
    }

    #[test]
    fn test_with_description() {
        let todo = Todo::new(1, String::from("测试"), Priority::High)
            .with_description(String::from("描述"));
        assert_eq!(todo.description.as_ref().unwrap(), "描述");
    }

    #[test]
    fn test_with_tags() {
        let todo = Todo::new(1, String::from("测试"), Priority::High)
            .with_tags(vec![String::from("工作"), String::from("重要")]);
        assert_eq!(todo.tags.len(), 2);
    }

    #[test]
    fn test_complete() {
        let mut todo = Todo::new(1, String::from("测试"), Priority::High);
        todo.complete();
        assert!(todo.completed);
        assert!(todo.completed_at.is_some());
    }

    #[test]
    fn test_uncomplete() {
        let mut todo = Todo::new(1, String::from("测试"), Priority::High);
        todo.complete();
        todo.uncomplete();
        assert!(!todo.completed);
        assert!(todo.completed_at.is_none());
    }

    #[test]
    fn test_add_tag() {
        let mut todo = Todo::new(1, String::from("测试"), Priority::High);
        todo.add_tag(String::from("工作"));
        assert_eq!(todo.tags.len(), 1);
        assert!(todo.has_tag("工作"));
    }

    #[test]
    fn test_add_duplicate_tag() {
        let mut todo = Todo::new(1, String::from("测试"), Priority::High);
        todo.add_tag(String::from("工作"));
        todo.add_tag(String::from("工作"));
        assert_eq!(todo.tags.len(), 1);
    }

    #[test]
    fn test_remove_tag() {
        let mut todo = Todo::new(1, String::from("测试"), Priority::High);
        todo.add_tag(String::from("工作"));
        assert!(todo.remove_tag("工作"));
        assert_eq!(todo.tags.len(), 0);
        assert!(!todo.has_tag("工作"));
    }

    #[test]
    fn test_remove_nonexistent_tag() {
        let mut todo = Todo::new(1, String::from("测试"), Priority::High);
        assert!(!todo.remove_tag("工作"));
    }

    #[test]
    fn test_has_tag() {
        let mut todo = Todo::new(1, String::from("测试"), Priority::High);
        todo.add_tag(String::from("工作"));
        assert!(todo.has_tag("工作"));
        assert!(!todo.has_tag("个人"));
    }

    #[test]
    fn test_age() {
        let todo = Todo::new(1, String::from("测试"), Priority::High);
        let age = todo.age();
        assert!(age.num_seconds() >= 0);
    }
}
