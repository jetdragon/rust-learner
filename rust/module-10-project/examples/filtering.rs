//! 过滤和搜索示例

use module_10_project::{new_todo_list, Filter, Priority};

fn main() {
    println!("=== 待办事项管理器 - 过滤和搜索 ===\n");

    let mut list = new_todo_list();

    // 添加多个任务
    list.add(
        String::from("学习 Rust"),
        Some(String::from("掌握 Rust 基础语法")),
        Priority::High,
        vec![String::from("学习"), String::from("编程")],
    );
    list.add(
        String::from("学习 Python"),
        Some(String::from("学习数据分析")),
        Priority::Medium,
        vec![String::from("学习"), String::from("编程")],
    );
    list.add(
        String::from("买菜"),
        None,
        Priority::Low,
        vec![String::from("生活")],
    );
    list.add(
        String::from("锻炼身体"),
        Some(String::from("每天跑步30分钟")),
        Priority::Medium,
        vec![String::from("健康"), String::from("运动")],
    );
    list.add(
        String::from("完成项目"),
        Some(String::from("完成所有 Rust 模块")),
        Priority::High,
        vec![String::from("工作"), String::from("Rust")],
    );

    // 标记一些任务为完成
    list.complete(1).unwrap();
    list.complete(3).unwrap();

    // 1. 查看所有任务
    println!("1. 所有任务:");
    for todo in list.filter(&Filter::All) {
        println!(
            "  {}: [{}] {} - {}",
            todo.id,
            if todo.completed { "✓" } else { " " },
            todo.title,
            todo.priority
        );
    }

    // 2. 查看未完成任务
    println!("\n2. 未完成任务:");
    for todo in list.filter(&Filter::Active) {
        println!("  {}: {} - {}", todo.id, todo.title, todo.priority);
    }

    // 3. 查看已完成任务
    println!("\n3. 已完成任务:");
    for todo in list.filter(&Filter::Completed) {
        println!("  {}: {}", todo.id, todo.title);
    }

    // 4. 按优先级过滤 - 高优先级
    println!("\n4. 高优先级任务:");
    for todo in list.filter(&Filter::Priority(Priority::High)) {
        println!(
            "  {}: [{}] {}",
            todo.id,
            if todo.completed { "✓" } else { " " },
            todo.title
        );
    }

    // 5. 按标签过滤
    println!("\n5. 带'学习'标签的任务:");
    for todo in list.filter(&Filter::Tag(String::from("学习"))) {
        println!("  {}: {}", todo.id, todo.title);
    }

    println!("\n6. 带'编程'标签的任务:");
    for todo in list.filter(&Filter::Tag(String::from("编程"))) {
        println!("  {}: {}", todo.id, todo.title);
    }

    // 7. 搜索关键词
    println!("\n7. 搜索'Rust':");
    for todo in list.filter(&Filter::Search(String::from("Rust"))) {
        println!("  {}: {}", todo.id, todo.title);
    }

    println!("\n8. 搜索'学习':");
    for todo in list.filter(&Filter::Search(String::from("学习"))) {
        println!(
            "  {}: {} (描述: {:?})",
            todo.id, todo.title, todo.description
        );
    }

    // 9. 统计信息
    println!("\n9. 统计信息:");
    println!("  总任务数: {}", list.len());
    println!("  未完成: {}", list.active_count());
    println!("  已完成: {}", list.completed_count());
    println!(
        "  高优先级: {}",
        list.filter(&Filter::Priority(Priority::High)).len()
    );
    println!(
        "  中优先级: {}",
        list.filter(&Filter::Priority(Priority::Medium)).len()
    );
    println!(
        "  低优先级: {}",
        list.filter(&Filter::Priority(Priority::Low)).len()
    );
}
