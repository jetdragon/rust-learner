// examples/lifetimes.rs
// 生命周期演示

use module_07_traits_generics::{longest, ImportantExcerpt};

fn main() {
    println!("=== 生命周期演示 ===\n");

    // 1. longest 函数
    println!("1. longest 函数:");
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
    println!(
        "  longest(\"{}\", \"{}\") = \"{}\"",
        string1, string2, result
    );

    // 2. 字符串字面量
    println!("\n2. 字符串字面量:");
    let s1 = "hello";
    let s2 = "world!";
    let result = longest(s1, s2);
    println!("  longest(\"{}\", \"{}\") = \"{}\"", s1, s2, result);

    // 3. 混合使用
    println!("\n3. 混合使用:");
    let s = String::from("hello");
    let result = longest(s.as_str(), "world");
    println!("  longest(\"{}\", \"{}\") = \"{}\"", s, "world", result);

    // 4. ImportantExcerpt 结构体
    println!("\n4. ImportantExcerpt 结构体:");
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let excerpt = ImportantExcerpt::new(first_sentence);
    println!("  摘录: \"{}\"", excerpt.part());
    println!("  level: {}", excerpt.level());

    // 5. 生命周期省略
    println!("\n5. 生命周期省略:");
    let excerpt = ImportantExcerpt::new("Hello, world!");
    let announcement = "Attention!";
    let part = excerpt.announce_and_return_part(announcement);
    println!("  返回的摘录: \"{}\"", part);

    // 6. 多个引用
    println!("\n6. 多个引用:");
    let string1 = String::from("long string");
    let string2 = String::from("short");
    let string3 = String::from("medium length");

    let result1 = longest(string1.as_str(), string2.as_str());
    let result2 = longest(result1, string3.as_str());
    println!("  最长的字符串: \"{}\"", result2);

    // 7. 静态生命周期
    println!("\n7. 静态生命周期:");
    let s: &'static str = "I have a static lifetime.";
    let result = longest(s, "short");
    println!("  longest(\"{}\", \"short\") = \"{}\"", s, result);

    // 8. 方法中的生命周期
    println!("\n8. 方法中的生命周期:");
    let text = "Hello, world! This is a test.";
    let excerpt = ImportantExcerpt::new(text);
    println!("  摘录: \"{}\"", excerpt.part());
    println!("  level: {}", excerpt.level());
}
