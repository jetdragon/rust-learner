//! # 10-综合项目：待办事项管理器
//!
//! 这是一个综合项目，运用前面学习的所有 Rust 知识构建一个完整的命令行待办事项应用。

pub mod error;
pub mod filter;
pub mod priority;
pub mod store;
pub mod todo;

pub use error::{Error, Saveable};
pub use filter::Filter;
pub use priority::Priority;
pub use store::TodoList;
pub use todo::Todo;

/// 创建一个新的空任务列表
pub fn new_todo_list() -> TodoList {
    TodoList::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_todo_list() {
        let list = new_todo_list();
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn test_add_todo() {
        let mut list = new_todo_list();
        let id = list.add(String::from("测试任务"), None, Priority::Medium, vec![]);
        assert_eq!(id, 1);
        assert_eq!(list.len(), 1);
        assert!(!list.is_empty());
    }

    #[test]
    fn test_get_todo() {
        let mut list = new_todo_list();
        let id = list.add(String::from("测试任务"), None, Priority::Medium, vec![]);
        let todo = list.get(id);
        assert!(todo.is_some());
        assert_eq!(todo.unwrap().title, "测试任务");
    }

    #[test]
    fn test_get_nonexistent_todo() {
        let list = new_todo_list();
        assert!(list.get(999).is_none());
    }

    #[test]
    fn test_complete_todo() {
        let mut list = new_todo_list();
        let id = list.add(String::from("测试任务"), None, Priority::Medium, vec![]);
        list.complete(id).unwrap();
        let todo = list.get(id).unwrap();
        assert!(todo.completed);
        assert!(todo.completed_at.is_some());
    }

    #[test]
    fn test_complete_nonexistent_todo() {
        let mut list = new_todo_list();
        assert!(list.complete(999).is_err());
    }

    #[test]
    fn test_uncomplete_todo() {
        let mut list = new_todo_list();
        let id = list.add(String::from("测试任务"), None, Priority::Medium, vec![]);
        list.complete(id).unwrap();
        list.uncomplete(id).unwrap();
        let todo = list.get(id).unwrap();
        assert!(!todo.completed);
        assert!(todo.completed_at.is_none());
    }

    #[test]
    fn test_remove_todo() {
        let mut list = new_todo_list();
        let id = list.add(String::from("测试任务"), None, Priority::Medium, vec![]);
        let removed = list.remove(id);
        assert!(removed.is_some());
        assert_eq!(list.len(), 0);
        assert!(list.get(id).is_none());
    }

    #[test]
    fn test_remove_nonexistent_todo() {
        let mut list = new_todo_list();
        assert!(list.remove(999).is_none());
    }

    #[test]
    fn test_active_and_completed_count() {
        let mut list = new_todo_list();
        let id1 = list.add(String::from("任务1"), None, Priority::Medium, vec![]);
        let id2 = list.add(String::from("任务2"), None, Priority::Medium, vec![]);
        assert_eq!(list.active_count(), 2);
        assert_eq!(list.completed_count(), 0);

        list.complete(id1).unwrap();
        assert_eq!(list.active_count(), 1);
        assert_eq!(list.completed_count(), 1);
    }

    #[test]
    fn test_filter_all() {
        let mut list = new_todo_list();
        list.add(String::from("任务1"), None, Priority::Medium, vec![]);
        list.add(String::from("任务2"), None, Priority::High, vec![]);
        let todos = list.filter(&Filter::All);
        assert_eq!(todos.len(), 2);
    }

    #[test]
    fn test_filter_completed() {
        let mut list = new_todo_list();
        let id1 = list.add(String::from("任务1"), None, Priority::Medium, vec![]);
        let id2 = list.add(String::from("任务2"), None, Priority::Medium, vec![]);
        list.complete(id1).unwrap();

        let completed = list.filter(&Filter::Completed);
        assert_eq!(completed.len(), 1);
        assert_eq!(completed[0].id, id1);

        let active = list.filter(&Filter::Active);
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].id, id2);
    }

    #[test]
    fn test_filter_by_priority() {
        let mut list = new_todo_list();
        let id1 = list.add(String::from("任务1"), None, Priority::High, vec![]);
        list.add(String::from("任务2"), None, Priority::Low, vec![]);

        let high_priority = list.filter(&Filter::Priority(Priority::High));
        assert_eq!(high_priority.len(), 1);
        assert_eq!(high_priority[0].id, id1);
    }

    #[test]
    fn test_filter_by_tag() {
        let mut list = new_todo_list();
        let id1 = list.add(
            String::from("任务1"),
            None,
            Priority::Medium,
            vec![String::from("工作")],
        );
        list.add(
            String::from("任务2"),
            None,
            Priority::Medium,
            vec![String::from("个人")],
        );

        let work_tasks = list.filter(&Filter::Tag(String::from("工作")));
        assert_eq!(work_tasks.len(), 1);
        assert_eq!(work_tasks[0].id, id1);
    }

    #[test]
    fn test_filter_by_search() {
        let mut list = new_todo_list();
        let id1 = list.add(String::from("学习 Rust 编程"), None, Priority::High, vec![]);
        list.add(String::from("买菜"), None, Priority::Low, vec![]);

        let results = list.filter(&Filter::Search(String::from("Rust")));
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, id1);
    }

    #[test]
    fn test_priority_ordering() {
        assert!(Priority::High > Priority::Medium);
        assert!(Priority::Medium > Priority::Low);
        assert!(Priority::Low < Priority::High);
    }

    #[test]
    fn test_priority_display() {
        assert_eq!(format!("{}", Priority::Low), "低");
        assert_eq!(format!("{}", Priority::Medium), "中");
        assert_eq!(format!("{}", Priority::High), "高");
    }

    #[test]
    fn test_priority_from_str() {
        assert_eq!("low".parse::<Priority>().unwrap(), Priority::Low);
        assert_eq!("medium".parse::<Priority>().unwrap(), Priority::Medium);
        assert_eq!("high".parse::<Priority>().unwrap(), Priority::High);
        assert_eq!("低".parse::<Priority>().unwrap(), Priority::Low);
        assert_eq!("中".parse::<Priority>().unwrap(), Priority::Medium);
        assert_eq!("高".parse::<Priority>().unwrap(), Priority::High);
        assert!("invalid".parse::<Priority>().is_err());
    }

    #[test]
    fn test_save_and_load() {
        let mut list = new_todo_list();
        list.add(
            String::from("测试任务"),
            Some(String::from("描述")),
            Priority::High,
            vec![String::from("测试")],
        );

        list.save("test_todos.json").unwrap();
        let loaded = TodoList::load("test_todos.json").unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded.get(1).unwrap().title, "测试任务");

        // 清理
        std::fs::remove_file("test_todos.json").ok();
        std::fs::remove_file("test_todos.json.bak").ok();
    }

    #[test]
    fn test_error_display() {
        let err = Error::NotFound(123);
        assert_eq!(format!("{}", err), "任务 123 不存在");

        let err = Error::InvalidInput(String::from("test"));
        assert_eq!(format!("{}", err), "无效输入: test");
    }

    #[test]
    fn test_todo_with_description() {
        let mut list = new_todo_list();
        let id = list.add(
            String::from("有描述的任务"),
            Some(String::from("这是描述")),
            Priority::Medium,
            vec![],
        );
        let todo = list.get(id).unwrap();
        assert_eq!(todo.description.as_ref().unwrap(), "这是描述");
    }

    #[test]
    fn test_todo_with_tags() {
        let mut list = new_todo_list();
        let id = list.add(
            String::from("有标签的任务"),
            None,
            Priority::Medium,
            vec![String::from("工作"), String::from("重要")],
        );
        let todo = list.get(id).unwrap();
        assert_eq!(todo.tags.len(), 2);
        assert!(todo.tags.contains(&"工作".to_string()));
        assert!(todo.tags.contains(&"重要".to_string()));
    }

    #[test]
    fn test_iter() {
        let mut list = new_todo_list();
        list.add(String::from("任务1"), None, Priority::Medium, vec![]);
        list.add(String::from("任务2"), None, Priority::Medium, vec![]);

        let count = list.iter().count();
        assert_eq!(count, 2);
    }
}
