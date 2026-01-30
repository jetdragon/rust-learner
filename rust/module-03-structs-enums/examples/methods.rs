// examples/methods.rs
// 方法语法演示

use module_03_structs_enums::Rectangle;

fn main() {
    println!("=== 方法语法演示 ===\n");

    // 实例方法
    println!("1. 实例方法 (&self):");
    let rect = Rectangle::new(10, 20);
    println!("  面积: {}", rect.area());
    println!("  是正方形: {}", rect.is_square());

    // 关联函数
    println!("\n2. 关联函数 (类似静态方法):");
    let square = Rectangle::square(5);
    println!("  创建 5x5 正方形");
    println!("  面积: {}", square.area());
    println!("  是正方形: {}", square.is_square());

    // 可变方法
    println!("\n3. 可变方法 (&mut self):");
    use module_03_structs_enums::User;
    let mut user = User::new(String::from("alice"), String::from("alice@example.com"));
    println!("  初始登录次数: {}", user.sign_in_count);
    user.increment_sign_in_count();
    user.increment_sign_in_count();
    println!("  当前登录次数: {}", user.sign_in_count);

    // 方法链
    println!("\n4. 比较矩形:");
    let rect1 = Rectangle::new(10, 10);
    let rect2 = Rectangle::new(5, 5);
    println!("  rect1 能容纳 rect2: {}", rect1.can_hold(&rect2));
    println!("  rect2 能容纳 rect1: {}", rect2.can_hold(&rect1));
}
