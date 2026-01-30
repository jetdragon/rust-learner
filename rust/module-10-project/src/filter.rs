//! 过滤器类型定义

use crate::Priority;
use crate::Todo;

/// 任务过滤器
#[derive(Debug, Clone)]
pub enum Filter {
    /// 所有任务
    All,
    /// 已完成的任务
    Completed,
    /// 未完成的任务
    Active,
    /// 按优先级过滤
    Priority(Priority),
    /// 按标签过滤
    Tag(String),
    /// 搜索关键词（在标题和描述中）
    Search(String),
}

impl Filter {
    /// 检查任务是否匹配过滤器
    pub fn matches(&self, todo: &Todo) -> bool {
        match self {
            Filter::All => true,
            Filter::Completed => todo.completed,
            Filter::Active => !todo.completed,
            Filter::Priority(p) => todo.priority == *p,
            Filter::Tag(tag) => todo.has_tag(tag),
            Filter::Search(query) => {
                todo.title.contains(query)
                    || todo.description.as_ref().is_some_and(|d| d.contains(query))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn create_test_todo(id: u32, title: &str, completed: bool, priority: Priority) -> Todo {
        Todo {
            id,
            title: title.to_string(),
            description: None,
            completed,
            priority,
            tags: Vec::new(),
            created_at: Utc::now(),
            completed_at: if completed { Some(Utc::now()) } else { None },
        }
    }

    #[test]
    fn test_filter_all() {
        let todo = create_test_todo(1, "测试", false, Priority::Medium);
        assert!(Filter::All.matches(&todo));
    }

    #[test]
    fn test_filter_completed() {
        let completed = create_test_todo(1, "已完成", true, Priority::Medium);
        let active = create_test_todo(2, "未完成", false, Priority::Medium);
        assert!(Filter::Completed.matches(&completed));
        assert!(!Filter::Completed.matches(&active));
    }

    #[test]
    fn test_filter_active() {
        let completed = create_test_todo(1, "已完成", true, Priority::Medium);
        let active = create_test_todo(2, "未完成", false, Priority::Medium);
        assert!(!Filter::Active.matches(&completed));
        assert!(Filter::Active.matches(&active));
    }

    #[test]
    fn test_filter_priority() {
        let high = create_test_todo(1, "高优先级", false, Priority::High);
        let low = create_test_todo(2, "低优先级", false, Priority::Low);
        assert!(Filter::Priority(Priority::High).matches(&high));
        assert!(!Filter::Priority(Priority::High).matches(&low));
    }

    #[test]
    fn test_filter_tag() {
        let mut todo = create_test_todo(1, "有标签", false, Priority::Medium);
        todo.tags = vec![String::from("工作"), String::from("重要")];
        assert!(Filter::Tag(String::from("工作")).matches(&todo));
        assert!(!Filter::Tag(String::from("个人")).matches(&todo));
    }

    #[test]
    fn test_filter_search_title() {
        let todo = create_test_todo(1, "学习 Rust 编程", false, Priority::Medium);
        assert!(Filter::Search(String::from("Rust")).matches(&todo));
        assert!(!Filter::Search(String::from("Python")).matches(&todo));
    }

    #[test]
    fn test_filter_search_description() {
        let mut todo = create_test_todo(1, "任务", false, Priority::Medium);
        todo.description = Some(String::from("这是一个关于编程的任务"));
        assert!(Filter::Search(String::from("编程")).matches(&todo));
    }
}
