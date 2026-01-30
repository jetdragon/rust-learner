// examples/patterns.rs
// 各种模式类型演示

use module_04_patterns::{
    age_category, describe_number, match_number, opposite_direction, season, Direction, Month,
};

fn main() {
    println!("=== 各种模式类型 ===\n");

    // 1. 字面值模式
    println!("1. 字面值模式:");
    let x = 1;
    match x {
        1 => println!("  一"),
        2 => println!("  二"),
        3 => println!("  三"),
        _ => println!("  其他"),
    }

    // 2. 变量模式
    println!("\n2. 变量模式:");
    let x = 5;
    match x {
        n => println!("  值是: {}", n),
    }

    // 3. 通配符模式
    println!("\n3. 通配符 _:");
    let x = 3;
    match x {
        1 => println!("  一"),
        2 => println!("  二"),
        _ => println!("  其他"),
    }

    // 4. 多个模式
    println!("\n4. 多个模式 |:");
    let x = 1;
    match x {
        1 | 2 => println!("  一或二"),
        3 => println!("  三"),
        _ => println!("  其他"),
    }

    // 5. 范围模式
    println!("\n5. 范围模式 ..=:");
    let x = 5;
    match x {
        1..=5 => println!("  1到5之间"),
        _ => println!("  超出范围"),
    }

    // 6. 范围演示
    println!("\n6. 使用 describe_number:");
    for n in [-4, -3, 0, 3, 4].iter() {
        println!("  {}: {}", n, describe_number(*n));
    }

    // 7. @ 绑定
    println!("\n7. @ 绑定:");
    for n in [0, 2, 5, 50, 200].iter() {
        println!("  {}: {}", n, match_number(*n));
    }

    // 8. @ 绑定与年龄
    println!("\n8. 年龄分类:");
    for age in [5, 10, 15, 25, 40, 70].iter() {
        println!("  {}", age_category(*age));
    }

    // 9. 匹配枚举
    println!("\n9. 匹配枚举 (季节):");
    let months = [Month::January, Month::April, Month::July, Month::October];
    for month in months.iter() {
        println!("  {:?}: {}", month, season(*month));
    }

    // 10. 反转方向
    println!("\n10. 反转方向:");
    for dir in [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ]
    .iter()
    {
        println!("  {:?} -> {:?}", dir, opposite_direction(*dir));
    }

    // 11. 守卫 (match guard)
    println!("\n11. 守卫:");
    let pair = (2, -2);
    match pair {
        (x, y) if x == -y => println!("  ({}, {}): 相反数", x, y),
        (0, y) => println!("  (0, {}): 第一个元素是0", y),
        (x, 0) => println!("  ({}, 0): 第二个元素是0", x),
        _ => println!("  ({}, {}): 其他", pair.0, pair.1),
    }

    // 12. 更复杂的守卫
    println!("\n12. 复杂守卫:");
    for num in [1, 2, 3, 4, 5, 6].iter() {
        match num {
            x if x % 2 == 0 => println!("  {} 是偶数", x),
            x if x % 3 == 0 => println!("  {} 能被3整除", x),
            _ => println!("  {}", num),
        }
    }
}
