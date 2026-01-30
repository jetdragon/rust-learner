// examples/move_semantics.rs
// 移动语义演示

fn main() {
    println!("=== 移动语义演示 ===\n");

    // 基本移动示例
    println!("基本移动:");
    let s1 = String::from("hello");
    let s2 = s1; // s1 移动到 s2
                 // println!("{}", s1);  // 编译错误！value borrowed here after move
    println!("  s2 = \"{}\"", s2);
    println!("  s1 不再有效\n");

    // 拷贝示例（Copy 类型）
    println!("Copy 类型（拷贝）:");
    let x = 5;
    let y = x; // i32 实现了 Copy，所以拷贝
    println!("  x = {}, y = {}", x, y);
    println!("  x 和 y 都有效\n");

    // 深拷贝示例
    println!("深拷贝 (.clone()):");
    let s3 = String::from("hello");
    let s4 = s3.clone(); // 显式深拷贝
    println!("  s3 = \"{}\", s4 = \"{}\"", s3, s4);
    println!("  s3 和 s4 都有效（通过 clone() 深拷贝）\n");

    // 函数调用中的移动
    println!("函数调用中的移动:");
    let s5 = String::from("function demo");
    println!("  调用前: s5 = \"{}\"", s5);
    takes_ownership(s5); // s5 的所有权移动到函数
                         // println!("{}", s5);  // 错误！s5 已失效

    let x = 5;
    println!("  x = {}", x);
    makes_copy(x); // i32 是 Copy 类型，拷贝传入
    println!("  调用 makes_copy 后: x = {} (仍然有效)\n", x);

    // 返回值和移动
    println!("返回值中的所有权转移:");
    let s6 = gives_ownership(); // 函数返回值的所有权转移给 s6
    println!("  s6 = \"{}\" (从函数获取所有权)", s6);

    let s7 = String::from("hello");
    let s8 = takes_and_gives_back(s7); // s7 移动到函数，再转移给 s8
    println!("  s8 = \"{}\" (从函数获取所有权)\n", s8);
    // println!("{}", s7);  // 错误！s7 已失效

    // 移动发生的场景总结
    println!("=== 移动发生的场景 ===");
    println!("1. 赋值给另一个变量（非 Copy 类型）");
    println!("2. 传递给函数（非 Copy 类型作为参数）");
    println!("3. 从函数返回（非 Copy 类型作为返回值）");
    println!("\n=== 拷贝发生的场景 ===");
    println!("1. Copy 类型赋值（如 i32, bool, char 等）");
    println!("2. Copy 类型传递给函数");
    println!("3. 显式调用 .clone()");
}

// 获取所有权的函数
fn takes_ownership(some_string: String) {
    println!("    函数接收到: \"{}\"", some_string);
} // some_string 超出作用域，被丢弃

// 拷贝传入的函数
fn makes_copy(some_integer: i32) {
    println!("    函数接收到: {}", some_integer);
} // some_integer 超出作用域，但无特殊处理

// 返回所有权的函数
fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // 返回 some_string，所有权转移给调用者
}

// 接收并返回所有权的函数
fn takes_and_gives_back(mut some_string: String) -> String {
    some_string.push_str(", world");
    some_string // 返回修改后的 String，所有权转移给调用者
}
