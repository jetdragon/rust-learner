// examples/destructuring.rs
// 解构结构体、枚举和元组演示

use module_04_patterns::{
    describe_employee, is_in_quadrant, list_contains, list_length, quadrant, shape_area, Employee,
    List, Point, Role, Shape,
};

fn main() {
    println!("=== 解构演示 ===\n");

    // 1. 解构结构体 - 基础
    println!("1. 解构结构体 - 基础:");
    let p = Point { x: 0, y: 7 };
    match p {
        Point { x: 0, y } => println!("  在y轴上: y = {}", y),
        Point { x, y: 0 } => println!("  在x轴上: x = {}", x),
        Point { x, y } => println!("  点坐标: ({}, {})", x, y),
    }

    // 2. 解构并重命名字段
    println!("\n2. 解构并重命名字段:");
    let p = Point { x: 1, y: 2 };
    match p {
        Point { x: a, y: b } => println!("  重命名: x = {}, y = {}", a, b),
    }

    // 3. 使用 .. 忽略剩余字段
    println!("\n3. 使用 .. 忽略剩余字段:");
    let p = Point { x: 5, y: 10 };
    match p {
        Point { x, .. } => println!("  只关心 x: {}", x),
    }

    // 4. 解构元组
    println!("\n4. 解构元组:");
    let tuple = (1, 2, 3, 4, 5);
    match tuple {
        (first, _, third, _, fifth) => {
            println!("  第1,3,5元素: {}, {}, {}", first, third, fifth);
        }
    }

    // 5. 使用 .. 忽略元组剩余部分
    println!("\n5. 使用 .. 忽略元组剩余部分:");
    let tuple = (1, 2, 3, 4, 5);
    let (first, second, ..) = tuple;
    println!("  前两个元素: {}, {}", first, second);

    // 6. 解构枚举 - 无数据
    println!("\n6. 解构枚举 - 无数据:");
    use module_04_patterns::{grade_from_enum, Grade};
    println!("  Excellent: {}", grade_from_enum(Grade::Excellent));

    // 7. 解构枚举 - 单值
    println!("\n7. 解构枚举 - 单值:");
    let maybe_number = Some(42);
    match maybe_number {
        Some(n) => println!("  有值: {}", n),
        None => println!("  没有值"),
    }

    // 8. 解构枚举 - 多值元组
    println!("\n8. 解构枚举 - 多值元组:");
    let color = (255, 0, 128);
    match color {
        (255, 0, _) => println!("  红色变种"),
        (0, 255, _) => println!("  绿色变种"),
        (0, 0, _) => println!("  蓝色变种"),
        _ => println!("  其他颜色"),
    }

    // 9. 解构枚举 - 命名字段
    println!("\n9. 解构枚举 - 命名字段:");
    let circle = Shape::Circle { radius: 5.0 };
    match circle {
        Shape::Circle { radius: _ } => {
            println!("  圆形面积: {:.2}", shape_area(&circle));
        }
        Shape::Rectangle { width, height } => {
            println!("  矩形: {} x {}", width, height);
        }
        Shape::Triangle { base, height } => {
            println!("  三角形: 底={}, 高={}", base, height);
        }
    }

    // 10. 嵌套解构
    println!("\n10. 嵌套解构:");
    let manager = Employee {
        name: "Alice".to_string(),
        role: Role::Manager {
            department: "工程".to_string(),
        },
    };
    println!("  {}", describe_employee(&manager));

    let engineer = Employee {
        name: "Bob".to_string(),
        role: Role::Engineer {
            level: 3,
            language: "Rust".to_string(),
        },
    };
    println!("  {}", describe_employee(&engineer));

    // 11. 解构和移动
    println!("\n11. 解构和所有权:");
    let point = Point { x: 3, y: 4 };
    let Point { x, y } = point; // point 被 move
    println!("  解构后: x = {}, y = {}", x, y);
    // println!("{:?}", point); // 错误：point 已被移动

    // 12. 解构引用
    println!("\n12. 解构引用:");
    let point = Point { x: 5, y: 10 };
    let Point { x, y } = &point; // 借用，不移动
    println!("  借用: x = {}, y = {}", x, y);
    println!("  原点仍在有效: {:?}", point);

    // 13. 解构链表
    println!("\n13. 解构链表:");
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
    println!("  包含 2? {}", list_contains(&list, 2));
    println!("  包含 5? {}", list_contains(&list, 5));
    println!("  长度: {}", list_length(&list));
    match &list {
        List::Cons(first, _rest) => {
            println!("  首元素: {}", first);
        }
        List::Nil => println!("  空链表"),
    }

    // 14. while let 解构
    println!("\n14. while let 解构:");
    let list = List::Cons(
        10,
        Box::new(List::Cons(
            20,
            Box::new(List::Cons(30, Box::new(List::Nil))),
        )),
    );
    let mut current = &list;
    print!("  链表元素: ");
    while let List::Cons(item, rest) = current {
        print!("{} ", item);
        current = rest;
    }
    println!();

    // 15. 象限判断
    println!("\n15. 使用解构判断象限:");
    let points = [
        Point { x: 1, y: 1 },
        Point { x: -1, y: 1 },
        Point { x: -1, y: -1 },
        Point { x: 1, y: -1 },
        Point { x: 0, y: 0 },
    ];
    for p in &points {
        println!(
            "  {:?}: {} (在Q1? {})",
            p,
            quadrant(p),
            is_in_quadrant(p, 1)
        );
    }
}
