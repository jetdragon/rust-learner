//! 基本用法示例

use module_10_project::{new_todo_list, Priority};

fn main() {
    println!("=== 待办事项管理器 - 基本用法 ===\n");

    // 创建任务列表
    let mut list = new_todo_list();

    // 1. 添加任务
    println!("1. 添加任务:");
    let id1 = list.add(
        String::from("学习 Rust"),
        Some(String::from("完成所有学习模块")),
        Priority::High,
        vec![String::from("学习"), String::from("Rust")],
    );
    println!("  已添加任务 {}: \"学习 Rust\"", id1);

    let id2 = list.add(
        String::from("买菜"),
        Some(String::from("买蔬菜和水果")),
        Priority::Low,
        vec![String::from("生活")],
    );
    println!("  已添加任务 {}: \"买菜\"", id2);

    let id3 = list.add(
        String::from("写代码"),
        None,
        Priority::Medium,
        vec![String::from("工作")],
    );
    println!("  已添加任务 {}: \"写代码\"", id3);

    // 2. 列出所有任务
    println!("\n2. 列出所有任务:");
    println!("  总任务数: {}", list.len());
    println!("  未完成: {}", list.active_count());
    println!("  已完成: {}", list.completed_count());

    for todo in list.iter() {
        println!(
            "  {}: [{}] {} - {} (标签: {:?})",
            todo.id,
            if todo.completed { "✓" } else { " " },
            todo.title,
            todo.priority,
            todo.tags
        );
    }

    // 3. 标记任务完成
    println!("\n3. 标记任务完成:");
    list.complete(id1).unwrap();
    println!("  任务 {} 已完成", id1);

    // 4. 获取任务详情
    println!("\n4. 获取任务详情:");
    if let Some(todo) = list.get(id1) {
        println!("  任务: {}", todo.title);
        println!("  描述: {:?}", todo.description);
        println!("  优先级: {}", todo.priority);
        println!(
            "  状态: {}",
            if todo.completed {
                "已完成"
            } else {
                "未完成"
            }
        );
        println!("  标签: {:?}", todo.tags);
    }

    // 5. 编辑任务
    println!("\n5. 编辑任务:");
    if let Some(todo) = list.get_mut(id2) {
        todo.title = String::from("去超市购物");
        todo.add_tag(String::from("重要"));
        println!("  任务 {} 已更新", id2);
    }

    // 6. 删除任务
    println!("\n6. 删除任务:");
    let removed = list.remove(id3);
    if let Some(todo) = removed {
        println!("  已删除任务: {}", todo.title);
    }

    // 7. 最终状态
    println!("\n7. 最终状态:");
    println!("  总任务数: {}", list.len());
    for todo in list.iter() {
        println!(
            "  {}: [{}] {} - {}",
            todo.id,
            if todo.completed { "✓" } else { " " },
            todo.title,
            todo.priority
        );
    }
}
