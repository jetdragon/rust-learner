// 函数演示

fn main() {
    println!("=== 函数演示 ===\n");

    // 1. 基本函数调用
    println!("1. 基本函数调用");
    greet();
    println!();

    // 2. 带参数的函数
    println!("2. 带参数的函数");
    greet_name("张三");
    greet_name("李四");
    println!();

    // 3. 带返回值的函数
    println!("3. 带返回值的函数");
    let sum = add(5, 3);
    println!("5 + 3 = {}", sum);

    let product = multiply(4, 7);
    println!("4 * 7 = {}", product);
    println!();

    // 4. 表达式作为返回值
    println!("4. 表达式作为返回值");
    let result = give_five();
    println!("give_five() 返回: {}", result);

    let plus_one = add_one(5);
    println!("add_one(5) 返回: {}", plus_one);
    println!();

    // 5. 多返回值（通过元组）
    println!("5. 多返回值（元组）");
    let (area, perimeter) = rectangle(3.0, 4.0);
    println!("3x4 矩形:");
    println!("  面积 = {}", area);
    println!("  周长 = {}", perimeter);
    println!();

    // 6. 语句 vs 表达式
    println!("6. 语句 vs 表达式");
    let y = {
        let x = 3;
        x + 1 // 这是表达式，没有分号
    };
    println!("块表达式的结果: y = {}", y);
    println!();

    // 7. 提前返回
    println!("7. 提前返回");
    let numbers = [1, 2, 3, 4, 5];
    match find_first_even(&numbers) {
        Some(n) => println!("找到第一个偶数: {}", n),
        None => println!("没有找到偶数"),
    }

    let odds = [1, 3, 5];
    match find_first_even(&odds) {
        Some(n) => println!("找到第一个偶数: {}", n),
        None => println!("没有找到偶数"),
    }
    println!();

    println!("=== 演示完成 ===");
}

// 基本函数：无参数，无返回值
fn greet() {
    println!("你好，欢迎学习 Rust！");
}

// 带参数的函数
fn greet_name(name: &str) {
    println!("你好，{}！", name);
}

// 带返回值的函数
fn add(x: i32, y: i32) -> i32 {
    x + y // 表达式作为返回值（无分号）
}

fn multiply(x: i32, y: i32) -> i32 {
    return x * y; // 使用 return 关键字
}

// 直接返回字面值
fn give_five() -> i32 {
    5
}

// 表达式计算
fn add_one(x: i32) -> i32 {
    x + 1
}

// 返回多个值（元组）
fn rectangle(width: f64, height: f64) -> (f64, f64) {
    let area = width * height;
    let perimeter = 2.0 * (width + height);
    (area, perimeter)
}

// 提前返回示例
fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for &number in numbers {
        if number % 2 == 0 {
            return Some(number); // 找到偶数，提前返回
        }
    }
    None // 没有找到
}
