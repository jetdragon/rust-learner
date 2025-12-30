// examples/borrowing.rs
// 借用规则演示

fn main() {
    println!("=== 借用规则演示 ===\n");

    // 不可变借用示例
    println!("1. 不可变借用 (&T):");
    let s1 = String::from("hello");
    let len = calculate_length(&s1);  // 借用 s1
    println!("  '{}' 的长度是 {}", s1, len);
    println!("  s1 仍然有效，因为只借用了它\n");

    // 可变借用示例
    println!("2. 可变借用 (&mut T):");
    let mut s2 = String::from("hello");
    println!("  修改前: s2 = \"{}\"", s2);
    change(&mut s2);  // 可变借用
    println!("  修改后: s2 = \"{}\"\n", s2);

    // 借用规则演示
    println!("=== 借用规则 ===");
    println!("规则 1: 同一时间只能有一个可变引用");
    let mut s3 = String::from("hello");
    {
        let r1 = &mut s3;
        r1.push_str(", world");
        println!("  r1 = \"{}\"", r1);
        // let r2 = &mut s3;  // 错误！不能同时有两个可变引用
    }  // r1 作用域结束

    let r2 = &mut s3;  // 现在可以创建新的可变引用
    r2.push_str("!");
    println!("  r2 = \"{}\"\n", r2);

    println!("规则 2: 可以有多个不可变引用");
    let s4 = String::from("hello");
    let r1 = &s4;
    let r2 = &s4;
    println!("  r1 = \"{}\", r2 = \"{}\"", r1, r2);
    println!("  多个不可变引用是允许的\n");

    println!("规则 3: 可变引用和不可变引用不能同时存在");
    let mut s5 = String::from("hello");
    let r1 = &s5;
    println!("  不可变引用 r1 = \"{}\"", r1);
    // let r2 = &mut s5;  // 错误！不能在不可变引用存在时创建可变引用
    println!("  不能在 r1 有效时创建可变引用");
    drop(r1);  // 显式丢弃 r1

    let r2 = &mut s5;  // 现在可以了
    r2.push_str(" world");
    println!("  可变引用 r2 = \"{}\"\n", r2);

    // 引用作用域演示
    println!("=== 引用作用域 ===");
    let mut s6 = String::from("hello");
    let r1 = &s6;
    let r2 = &s6;
    println!("  两个不可变引用: r1 = \"{}\", r2 = \"{}\"", r1, r2);
    // 引用作用域从声明开始，到最后一次使用结束
    // 而不是到所在的作用域结束

    let r3 = &mut s6;  // 新的作用域开始
    r3.push_str(" world");
    println!("  可变引用 r3 = \"{}\"\n", r3);

    // 悬垂引用防护
    println!("=== 悬垂引用防护 ===");
    println!("  Rust 编译器防止悬垂引用");
    println!("  以下代码无法编译：");
    println!("  fn dangle() -> &String {{");
    println!("      let s = String::from(\"hello\");");
    println!("      &s  // 返回指向已释放内存的引用");
    println!("  }}");
    println!("  正确做法: 直接返回 String\n");
}

// 计算长度（借用，不获取所有权）
fn calculate_length(s: &String) -> usize {
    s.len()
}  // s 超出作用域，但因为它不拥有值，所以不会丢弃

// 修改字符串（可变借用）
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
