// examples/structs.rs
// 结构体定义和使用演示

use module_03_structs_enums::{Rectangle, User};

fn main() {
    println!("=== 结构体演示 ===\n");

    // 字段结构体
    println!("1. 字段结构体:");
    let user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        sign_in_count: 1,
        active: true,
    };
    println!("  用户: {}", user1.username);
    println!("  邮箱: {}", user1.email);
    println!("  状态: {}", user1.status());

    // 字段初始化简写
    let username = String::from("bob");
    let email = String::from("bob@example.com");
    let user2 = User {
        username,
        email,
        ..user1 // 结构体更新语法
    };
    println!("  用户2: {}", user2.username);

    // 元组结构体
    println!("\n2. 元组结构体:");
    use module_03_structs_enums::{Color, Point};
    let black = Color(0, 0, 0);
    let origin = Point(0, 0);
    println!("  黑色: {:?}", black);
    println!("  原点: {:?}", origin);

    // 单元结构体
    println!("\n3. 单元结构体:");
    use module_03_structs_enums::AlwaysEqual;
    let subject = AlwaysEqual;
    println!("  AlwaysEqual 实例: {:?}", subject);

    // 矩形示例
    println!("\n4. 矩形示例:");
    let rect = Rectangle::new(10, 20);
    println!("  尺寸: {} x {}", rect.width, rect.height);
    println!("  面积: {}", rect.area());
    println!("  是正方形: {}", rect.is_square());

    let square = Rectangle::square(5);
    println!("  正方形: {} x {}", square.width, square.height);
    println!("  是正方形: {}", square.is_square());
}
