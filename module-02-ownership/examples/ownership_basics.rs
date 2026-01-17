// examples/ownership_basics.rs
// 所有权基本规则演示

fn main() {
    println!("=== 所有权基本规则演示 ===\n");

    // 规则 1：每个值都有一个变量作为它的所有者
    println!("规则 1：每个值都有一个所有者");
    let s = String::from("hello");
    println!("  s = \"{}\" 是 \"hello\" 的所有者\n", s);

    // 规则 2：同一时间只能有一个所有者
    println!("规则 2：同一时间只能有一个所有者");
    let s1 = String::from("hello");
    let s2 = s1; // s1 的所有权移动到 s2
    println!("  s2 现在是 \"hello\" 的所有者");
    println!("  s2 = \"{}\"", s2);
    // println!("{}", s1);  // 错误！s1 已失效
    println!("  s1 不再有效\n");

    // 规则 3：当所有者超出作用域，值将被丢弃
    println!("规则 3：所有者超出作用域时值被丢弃");
    {
        let s3 = String::from("world");
        println!("  在作用域内: s3 = \"{}\"", s3);
    } // s3 超出作用域，被丢弃
    println!("  s3 已超出作用域，内存被释放\n");

    // 演示变量作用域
    println!("=== 变量作用域演示 ===");
    {
        let s4 = String::from("scope demo");
        println!("  在作用域内: s4 有效");
        println!("  s4 = \"{}\"", s4);
    }
    // println!("{}", s4);  // 错误！s4 不再有效
    println!("  s4 已超出作用域\n");

    // String 类型在内存中的表示
    println!("=== String 内存结构 ===");
    let s5 = String::from("hello");
    println!("  String 包含：");
    println!("    - 指向堆数据的指针");
    println!("    - 长度 (len): {}", s5.len());
    println!("    - 容量 (capacity): {}", s5.capacity());
    println!("    - 堆上的数据: \"{}\"\n", s5);

    // 字符串字面量的区别
    println!("=== 字符串字面量 ===");
    let literal = "hello"; // &str 类型
    println!("  字符串字面量类型: &str");
    println!("  literal = \"{}\"", literal);
    println!("  字符串字面值硬编码在程序中\n");

    // Copy 类型演示
    println!("=== Copy 类型演示 ===");
    let x = 5;
    let y = x; // i32 实现了 Copy，所以拷贝而非移动
    println!("  x = {}, y = {}", x, y);
    println!("  x 和 y 都有效（i32 是 Copy 类型）\n");

    // 非 Copy 类型演示
    println!("=== 非 Copy 类型演示 ===");
    let s6 = String::from("hello");
    let s7 = s6; // String 不是 Copy，移动所有权
    println!("  s7 = \"{}\"", s7);
    println!("  s6 不再有效（String 不是 Copy 类型）\n");
}
