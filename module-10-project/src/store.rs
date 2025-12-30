//! 任务列表管理器

use crate::{error::Saveable, Error, Filter, Priority, Todo};
use std::fs;
use std::path::Path;

/// 任务列表管理器
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TodoList {
    todos: Vec<Todo>,
    next_id: u32,
}

impl TodoList {
    /// 创建新的空任务列表
    pub fn new() -> Self {
        Self {
            todos: Vec::new(),
            next_id: 1,
        }
    }

    /// 添加新任务，返回任务 ID
    pub fn add(
        &mut self,
        title: String,
        description: Option<String>,
        priority: Priority,
        tags: Vec<String>,
    ) -> u32 {
        let mut todo = Todo::new(self.next_id, title, priority);
        if let Some(desc) = description {
            todo = todo.with_description(desc);
        }
        if !tags.is_empty() {
            todo = todo.with_tags(tags);
        }
        self.todos.push(todo);
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// 根据 ID 获取任务引用
    pub fn get(&self, id: u32) -> Option<&Todo> {
        self.todos.iter().find(|t| t.id == id)
    }

    /// 根据 ID 获取可变引用
    pub fn get_mut(&mut self, id: u32) -> Option<&mut Todo> {
        self.todos.iter_mut().find(|t| t.id == id)
    }

    /// 删除任务，返回被删除的任务
    pub fn remove(&mut self, id: u32) -> Option<Todo> {
        let pos = self.todos.iter().position(|t| t.id == id)?;
        Some(self.todos.remove(pos))
    }

    /// 标记任务为完成
    pub fn complete(&mut self, id: u32) -> Result<(), Error> {
        match self.get_mut(id) {
            Some(todo) => {
                todo.complete();
                Ok(())
            }
            None => Err(Error::NotFound(id)),
        }
    }

    /// 取消任务完成状态
    pub fn uncomplete(&mut self, id: u32) -> Result<(), Error> {
        match self.get_mut(id) {
            Some(todo) => {
                todo.uncomplete();
                Ok(())
            }
            None => Err(Error::NotFound(id)),
        }
    }

    /// 返回任务数量
    pub fn len(&self) -> usize {
        self.todos.len()
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.todos.is_empty()
    }

    /// 返回迭代器
    pub fn iter(&self) -> impl Iterator<Item = &Todo> {
        self.todos.iter()
    }

    /// 返回未完成任务数量
    pub fn active_count(&self) -> usize {
        self.todos.iter().filter(|t| !t.completed).count()
    }

    /// 返回已完成任务数量
    pub fn completed_count(&self) -> usize {
        self.todos.iter().filter(|t| t.completed).count()
    }

    /// 根据过滤器筛选任务
    pub fn filter(&self, filter: &Filter) -> Vec<&Todo> {
        self.todos.iter().filter(|t| filter.matches(t)).collect()
    }

    /// 按优先级排序（高到低）
    pub fn sort_by_priority(&mut self) {
        self.todos.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    /// 按创建时间排序（新到旧）
    pub fn sort_by_created_at(&mut self) {
        self.todos.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    }

    /// 清空所有任务
    pub fn clear(&mut self) {
        self.todos.clear();
    }
}

impl Default for TodoList {
    fn default() -> Self {
        Self::new()
    }
}

impl Saveable for TodoList {
    fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let path = path.as_ref();
        // 创建备份
        if path.exists() {
            let backup_path = path.with_extension("json.bak");
            fs::copy(path, &backup_path)?;
        }
        // 序列化为 JSON
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let path = path.as_ref();
        let json = fs::read_to_string(path)?;
        let todo_list: TodoList = serde_json::from_str(&json)?;
        Ok(todo_list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let list = TodoList::new();
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn test_add() {
        let mut list = TodoList::new();
        let id = list.add(String::from("测试任务"), None, Priority::Medium, vec![]);
        assert_eq!(id, 1);
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_add_multiple() {
        let mut list = TodoList::new();
        list.add(String::from("任务1"), None, Priority::Medium, vec![]);
        list.add(String::from("任务2"), None, Priority::Medium, vec![]);
        list.add(String::from("任务3"), None, Priority::Medium, vec![]);
        assert_eq!(list.len(), 3);
        assert_eq!(list.next_id, 4);
    }

    #[test]
    fn test_get() {
        let mut list = TodoList::new();
        let id = list.add(String::from("测试任务"), None, Priority::Medium, vec![]);
        let todo = list.get(id);
        assert!(todo.is_some());
        assert_eq!(todo.unwrap().title, "测试任务");
    }

    #[test]
    fn test_get_nonexistent() {
        let list = TodoList::new();
        assert!(list.get(999).is_none());
    }

    #[test]
    fn test_get_mut() {
        let mut list = TodoList::new();
        let id = list.add(String::from("测试任务"), None, Priority::Medium, vec![]);
        if let Some(todo) = list.get_mut(id) {
            todo.title = String::from("已修改");
        }
        assert_eq!(list.get(id).unwrap().title, "已修改");
    }

    #[test]
    fn test_remove() {
        let mut list = TodoList::new();
        let id = list.add(String::from("测试任务"), None, Priority::Medium, vec![]);
        let removed = list.remove(id);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().title, "测试任务");
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_remove_nonexistent() {
        let mut list = TodoList::new();
        assert!(list.remove(999).is_none());
    }

    #[test]
    fn test_complete() {
        let mut list = TodoList::new();
        let id = list.add(String::from("测试任务"), None, Priority::Medium, vec![]);
        list.complete(id).unwrap();
        assert!(list.get(id).unwrap().completed);
    }

    #[test]
    fn test_complete_nonexistent() {
        let mut list = TodoList::new();
        assert!(list.complete(999).is_err());
    }

    #[test]
    fn test_uncomplete() {
        let mut list = TodoList::new();
        let id = list.add(String::from("测试任务"), None, Priority::Medium, vec![]);
        list.complete(id).unwrap();
        list.uncomplete(id).unwrap();
        assert!(!list.get(id).unwrap().completed);
    }

    #[test]
    fn test_active_and_completed_count() {
        let mut list = TodoList::new();
        let id1 = list.add(String::from("任务1"), None, Priority::Medium, vec![]);
        let id2 = list.add(String::from("任务2"), None, Priority::Medium, vec![]);
        assert_eq!(list.active_count(), 2);
        assert_eq!(list.completed_count(), 0);

        list.complete(id1).unwrap();
        assert_eq!(list.active_count(), 1);
        assert_eq!(list.completed_count(), 1);
    }

    #[test]
    fn test_filter() {
        let mut list = TodoList::new();
        let id1 = list.add(String::from("已完成任务"), None, Priority::High, vec![]);
        let id2 = list.add(String::from("未完成任务"), None, Priority::Medium, vec![]);
        list.complete(id1).unwrap();

        let completed = list.filter(&Filter::Completed);
        assert_eq!(completed.len(), 1);
        assert_eq!(completed[0].id, id1);

        let active = list.filter(&Filter::Active);
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].id, id2);
    }

    #[test]
    fn test_sort_by_priority() {
        let mut list = TodoList::new();
        list.add(String::from("低"), None, Priority::Low, vec![]);
        list.add(String::from("高"), None, Priority::High, vec![]);
        list.add(String::from("中"), None, Priority::Medium, vec![]);

        list.sort_by_priority();
        let titles: Vec<&str> = list.iter().map(|t| t.title.as_str()).collect();
        assert_eq!(titles, vec!["高", "中", "低"]);
    }

    #[test]
    fn test_clear() {
        let mut list = TodoList::new();
        list.add(String::from("任务1"), None, Priority::Medium, vec![]);
        list.add(String::from("任务2"), None, Priority::Medium, vec![]);
        list.clear();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_add_with_description_and_tags() {
        let mut list = TodoList::new();
        let id = list.add(
            String::from("任务"),
            Some(String::from("描述")),
            Priority::High,
            vec![String::from("工作"), String::from("重要")],
        );
        let todo = list.get(id).unwrap();
        assert_eq!(todo.description.as_ref().unwrap(), "描述");
        assert_eq!(todo.tags.len(), 2);
    }

    #[test]
    fn test_iter() {
        let mut list = TodoList::new();
        list.add(String::from("任务1"), None, Priority::Medium, vec![]);
        list.add(String::from("任务2"), None, Priority::Medium, vec![]);
        assert_eq!(list.iter().count(), 2);
    }
}
