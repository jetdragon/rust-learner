//! 数据持久化示例

use module_10_project::{new_todo_list, Priority, Saveable};

fn main() {
    println!("=== 待办事项管理器 - 数据持久化 ===\n");

    let filename = "example_todos.json";

    // 1. 创建并保存任务列表
    println!("1. 创建任务列表:");
    let mut list = new_todo_list();

    list.add(
        String::from("学习 Rust"),
        Some(String::from("掌握所有权和借用")),
        Priority::High,
        vec![String::from("学习")],
    );
    list.add(
        String::from("完成项目"),
        None,
        Priority::Medium,
        vec![String::from("工作")],
    );
    list.add(
        String::from("锻炼"),
        Some(String::from("每天运动")),
        Priority::Low,
        vec![String::from("健康")],
    );

    println!("  创建了 {} 个任务", list.len());

    // 2. 保存到文件
    println!("\n2. 保存到文件:");
    match list.save(filename) {
        Ok(_) => println!("  成功保存到 {}", filename),
        Err(e) => println!("  保存失败: {}", e),
    }

    // 3. 从文件加载
    println!("\n3. 从文件加载:");
    let loaded_list = match module_10_project::TodoList::load(filename) {
        Ok(l) => {
            println!("  成功从 {} 加载 {} 个任务", filename, l.len());
            l
        }
        Err(e) => {
            println!("  加载失败: {}", e);
            new_todo_list()
        }
    };

    // 4. 显示加载的任务
    println!("\n4. 加载的任务:");
    for todo in loaded_list.iter() {
        println!(
            "  {}: [{}] {} - {}",
            todo.id,
            if todo.completed { "✓" } else { " " },
            todo.title,
            todo.priority
        );
    }

    // 5. 修改并保存
    println!("\n5. 修改任务并保存:");
    let mut modified_list = loaded_list;
    modified_list.complete(1).unwrap();
    println!("  标记任务 1 为完成");

    match modified_list.save(filename) {
        Ok(_) => println!("  保存成功"),
        Err(e) => println!("  保存失败: {}", e),
    }

    // 6. 再次加载验证
    println!("\n6. 再次加载验证:");
    match module_10_project::TodoList::load(filename) {
        Ok(l) => {
            println!("  加载成功，共 {} 个任务", l.len());
            println!("  已完成: {}", l.completed_count());
            println!("  未完成: {}", l.active_count());
        }
        Err(e) => println!("  加载失败: {}", e),
    }

    // 7. 备份信息
    println!("\n7. 备份信息:");
    println!("  备份文件已自动创建: {}.bak", filename);

    // 清理提示
    println!(
        "\n提示: 运行结束后可删除 {} 和 {}.bak 文件",
        filename, filename
    );
}
