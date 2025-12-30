// examples/enums.rs
// 枚举定义和匹配演示

use module_03_structs_enums::{IpAddr, Shape, TrafficLight};

fn main() {
    println!("=== 枚举演示 ===\n");

    // 简单枚举
    println!("1. 简单枚举:");
    let red = TrafficLight::Red;
    println!("  红灯时长: {}秒", red.duration());
    println!("  下一个灯: {:?}", red.next());

    // 带数据的枚举
    println!("\n2. 带数据的枚举:");
    let localhost = IpAddr::V4(String::from("127.0.0.1"));
    println!("  本地地址: {}", localhost.address());
    println!("  是 IPv4: {}", localhost.is_v4());

    // 枚举模式匹配
    println!("\n3. 模式匹配:");
    let shapes = vec![
        Shape::Circle { radius: 1.0 },
        Shape::Rectangle { width: 10.0, height: 5.0 },
        Shape::Triangle { base: 6.0, height: 4.0 },
    ];

    for shape in shapes {
        println!("  形状: {:?}, 面积: {:.2}", shape, shape.area());
    }

    // Option 枚举
    println!("\n4. Option 枚举:");
    let some_number: Option<i32> = Some(5);
    let absent_number: Option<i32> = None;

    println!("  有值: {:?}", some_number);
    println!("  无值: {:?}", absent_number);

    // Result 枚举
    println!("\n5. Result 枚举:");
    use module_03_structs_enums::safe_divide;

    match safe_divide(10.0, 2.0) {
        Ok(result) => println!("  10 / 2 = {}", result),
        Err(e) => println!("  错误: {}", e),
    }

    match safe_divide(10.0, 0.0) {
        Ok(result) => println!("  10 / 0 = {}", result),
        Err(e) => println!("  错误: {}", e),
    }

    // if let 简化
    println!("\n6. if let 简化:");
    if let Some(n) = some_number {
        println!("  值是: {}", n);
    }
}
