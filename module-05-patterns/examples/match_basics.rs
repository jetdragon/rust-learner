// examples/match_basics.rs
// match 表达式基础演示

use module_04_patterns::{grade, grade_from_enum, Grade};

fn main() {
    println!("=== match 表达式基础 ===\n");

    // 1. 基础 match
    println!("1. 基础 match:");
    let number = 3;
    match number {
        1 => println!("  一"),
        2 => println!("  二"),
        3 => println!("  三"),
        4 => println!("  四"),
        _ => println!("  其他"),
    }

    // 2. match 返回值
    println!("\n2. match 作为表达式:");
    let result = match number {
        1 | 2 => "小",
        3 | 4 => "中",
        _ => "大",
    };
    println!("  结果: {}", result);

    // 3. 使用范围模式
    println!("\n3. 范围模式:");
    let score = 85;
    let letter = match score {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        _ => "F",
    };
    println!("  分数 {} 的等级: {}", score, letter);

    // 4. 使用 grade 函数
    println!("\n4. 评级函数:");
    for score in [95, 85, 70, 50].iter() {
        println!("  {}分: {}", score, grade(*score, 100));
    }

    // 5. 匹配枚举
    println!("\n5. 匹配枚举:");
    for g in [Grade::Excellent, Grade::Good, Grade::Pass, Grade::Fail].iter() {
        println!("  {:?}: {}", g, grade_from_enum(*g));
    }

    // 6. 多个模式
    println!("\n6. 多个模式:");
    let x = 5;
    match x {
        1 | 2 => println!("  1 或 2"),
        3 | 4 | 5 => println!("  3, 4 或 5"),
        _ => println!("  其他"),
    }

    // 7. 通配符
    println!("\n7. 通配符 _:");
    let y = 100;
    match y {
        0 => println!("  零"),
        1..=9 => println!("  一位数"),
        _ => println!("  多位数"),
    }

    // 8. 代码块作为分支
    println!("\n8. 代码块作为分支:");
    let z = 7;
    let description = match z {
        n if n % 2 == 0 => {
            format!("偶数: {}", n)
        }
        n => {
            format!("奇数: {}", n)
        }
    };
    println!("  {}", description);
}
