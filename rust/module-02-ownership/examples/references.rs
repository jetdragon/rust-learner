// examples/references.rs
// 引用使用演示

fn main() {
    println!("=== 引用使用演示 ===\n");

    // 创建引用的基本语法
    println!("1. 创建引用:");
    let s1 = String::from("hello world");
    let len = calculate_length(&s1);
    println!("  原字符串: \"{}\"", s1);
    println!("  长度: {}", len);
    println!("  s1 仍然有效，因为函数只是借用它\n");

    // 可变引用
    println!("2. 可变引用:");
    let mut s2 = String::from("hello");
    println!("  修改前: \"{}\"", s2);
    append_world(&mut s2);
    println!("  修改后: \"{}\"\n", s2);

    // 引用的引用
    println!("3. 引用的引用:");
    let s3 = String::from("hello");
    let r1 = &s3;
    let r2 = &r1; // r2 是对引用的引用
    println!("  s3 = \"{}\"", s3);
    println!("  r1 引用 s3");
    println!("  r2 引用 r1");
    println!("  通过 r2 访问: \"{}\"\n", *r2); // 解引用两次

    // 使用引用避免拷贝
    println!("4. 避免不必要的拷贝:");
    let s4 = String::from("A very long string...");
    // 使用引用，字符串不会被拷贝
    print_length(&s4);
    print_length(&s4);
    println!("  两次调用都没有拷贝字符串\n");

    // 引用作为返回值
    println!("5. 返回引用:");
    let s5 = String::from("hello world rust");
    let first = first_word(&s5);
    println!("  原字符串: \"{}\"", s5);
    println!("  第一个单词: \"{}\"", first);
    println!("  返回的是切片，不拷贝数据\n");

    // 多个不可变引用
    println!("6. 多个不可变引用:");
    let s6 = String::from("hello");
    let r1 = &s6;
    let r2 = &s6;
    let r3 = &s6;
    println!("  可以同时有多个不可变引用");
    println!("  r1 = \"{}\", r2 = \"{}\", r3 = \"{}\"\n", r1, r2, r3);

    // 字符串字面量也是引用
    println!("7. 字符串字面量:");
    let s7: &str = "hello world";
    println!("  字符串字面量类型: &str");
    println!("  s7 = \"{}\"", s7);
    println!("  字符串字面量是不可变引用\n");

    // 引用的比较
    println!("8. 引用比较:");
    let s8 = String::from("hello");
    let s9 = String::from("hello");
    let r1 = &s8;
    let r2 = &s9;
    println!("  r1 指向 s8, r2 指向 s9");
    println!("  r1 == r2: {}", r1 == r2); // 比较值
    println!("  不同的内存地址，但值相同\n");

    // 引用的解引用
    println!("9. 解引用:");
    let s10 = String::from("hello");
    let r1 = &s10;
    println!("  r1 = &s10");
    println!("  *r1 = \"{}\" (解引用获取值)", *r1);
    println!("  但通常不需要显式解引用");
    println!("  r1.len() = {} (自动解引用)", r1.len());
}

// 计算长度（借用字符串）
fn calculate_length(s: &String) -> usize {
    s.len()
} // s 超出作用域，但不拥有值，所以不会丢弃

// 追加文本（可变借用）
fn append_world(s: &mut String) {
    s.push_str(" world");
}

// 打印长度（不获取所有权）
fn print_length(s: &String) {
    println!("  长度: {}", s.len());
}

// 获取第一个单词
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
