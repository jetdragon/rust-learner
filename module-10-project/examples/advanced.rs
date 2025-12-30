//! 高级功能示例：排序、标签操作和批量处理

use module_10_project::{new_todo_list, Filter, Priority};

fn main() {
    println!("=== 待办事项管理器 - 高级功能 ===\n");

    let mut list = new_todo_list();

    // 添加多个任务用于演示
    list.add(
        String::from("低优先级任务"),
        None,
        Priority::Low,
        vec![String::from("其他")],
    );
    list.add(
        String::from("高优先级任务 A"),
        None,
        Priority::High,
        vec![String::from("工作"), String::from("重要")],
    );
    list.add(
        String::from("中优先级任务"),
        None,
        Priority::Medium,
        vec![String::from("学习")],
    );
    list.add(
        String::from("高优先级任务 B"),
        None,
        Priority::High,
        vec![String::from("工作")],
    );
    list.add(
        String::from("带多个标签的任务"),
        None,
        Priority::Medium,
        vec![
            String::from("工作"),
            String::from("学习"),
            String::from("重要"),
        ],
    );

    // 1. 按优先级排序
    println!("1. 按优先级排序（高到低）:");
    list.sort_by_priority();
    for todo in list.iter() {
        println!("  {} - {} ({:?})", todo.priority, todo.title, todo.tags);
    }

    // 2. 标签操作
    println!("\n2. 标签操作:");
    if let Some(todo) = list.get_mut(1) {
        println!("  原始标签: {:?}", todo.tags);

        // 添加标签
        todo.add_tag(String::from("新标签"));
        println!("  添加'新标签'后: {:?}", todo.tags);

        // 尝试添加重复标签
        todo.add_tag(String::from("新标签"));
        println!("  再次添加'新标签'（不会重复）: {:?}", todo.tags);

        // 移除标签
        todo.remove_tag("其他");
        println!("  移除'其他'后: {:?}", todo.tags);
    }

    // 3. 按标签分组统计
    println!("\n3. 按标签统计:");
    let mut tag_counts: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
    for todo in list.iter() {
        for tag in &todo.tags {
            *tag_counts.entry(tag).or_insert(0) += 1;
        }
    }
    for (tag, count) in &tag_counts {
        println!("  {}: {} 个任务", tag, count);
    }

    // 4. 批量操作 - 完成所有高优先级任务
    println!("\n4. 批量完成高优先级任务:");
    let high_priority_ids: Vec<u32> = list
        .filter(&Filter::Priority(Priority::High))
        .iter()
        .map(|t| t.id)
        .collect();

    for id in &high_priority_ids {
        list.complete(*id).unwrap();
        println!("  已完成任务 {}", id);
    }

    // 5. 查看未完成任务
    println!("\n5. 剩余未完成任务:");
    for todo in list.filter(&Filter::Active) {
        println!("  {}: {} - {}", todo.id, todo.title, todo.priority);
    }

    // 6. 使用迭代器进行数据处理
    println!("\n6. 使用迭代器处理数据:");
    let titles: Vec<String> = list.iter().map(|t| t.title.clone()).collect();
    println!("  所有任务标题: {:?}", titles);

    let completed_titles: Vec<&str> = list
        .iter()
        .filter(|t| t.completed)
        .map(|t| t.title.as_str())
        .collect();
    println!("  已完成任务标题: {:?}", completed_titles);

    // 7. 任务年龄
    println!("\n7. 任务年龄（创建时间）:");
    for todo in list.iter().take(3) {
        let age = todo.age();
        println!("  {}: {} 秒前创建", todo.title, age.num_seconds());
    }

    // 8. 复杂过滤 - 多个条件的组合
    println!("\n8. 复杂过滤示例:");
    // 查找：未完成 + 高优先级 + 带"重要"标签的任务
    let complex_filtered: Vec<module_10_project::Todo> = list
        .iter()
        .filter(|t| !t.completed)
        .filter(|t| t.priority == Priority::High)
        .filter(|t| t.has_tag("重要"))
        .cloned()
        .collect();

    println!("  未完成 + 高优先级 + 重要标签的任务:");
    for todo in complex_filtered {
        println!("    {}: {}", todo.id, todo.title);
    }

    // 9. 清空列表
    println!("\n9. 清空列表:");
    println!("  清空前: {} 个任务", list.len());
    list.clear();
    println!("  清空后: {} 个任务", list.len());
}
