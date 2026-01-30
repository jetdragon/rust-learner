// examples/basic_lifetimes.rs
// 基础生命周期演示

use module_08_lifetimes::{first_part, first_word, longest};

fn main() {
    println!("=== 基础生命周期演示 ===\n");

    // 1. longest 函数
    println!("1. longest 函数:");
    let s1 = "long string is long";
    let s2 = "short";
    let result = longest(s1, s2);
    println!("  longest(\"{}\", \"{}\") = \"{}\"", s1, s2, result);

    // 2. 动态字符串
    println!("\n2. 动态字符串:");
    let string1 = String::from("long string");
    let string2 = String::from("short");
    let result = longest(&string1, &string2);
    println!(
        "  longest(\"{}\", \"{}\") = \"{}\"",
        string1, string2, result
    );

    // 3. first_word 函数
    println!("\n3. first_word 函数:");
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("  first_word(\"{}\") = \"{}\"", s, word);

    // 4. first_part 函数
    println!("\n4. first_part 函数:");
    let s = "hello world rust";
    let part = first_part(s);
    println!("  first_part(\"{}\") = \"{}\"", s, part);

    // 5. 字符串字面量
    println!("\n5. 字符串字面量:");
    let s1 = "hello";
    let s2 = "world!";
    let result = longest(s1, s2);
    println!("  longest(\"{}\", \"{}\") = \"{}\"", s1, s2, result);

    // 6. 生命周期推断
    println!("\n6. 生命周期推断:");
    let s = "Hello, world!";
    let word = first_word(s);
    println!("  first_word 返回的引用有效: \"{}\"", word);

    // 7. 不同来源的引用
    println!("\n7. 不同来源的引用:");
    let string1 = String::from("dynamic string");
    let string2 = "static string";
    let result = longest(&string1, string2);
    println!(
        "  longest(\"{}\", \"{}\") = \"{}\"",
        string1, string2, result
    );

    // 8. 空字符串处理
    println!("\n8. 空字符串处理:");
    let empty = "";
    let non_empty = "non-empty";
    let result = longest(empty, non_empty);
    println!(
        "  longest(\"{}\", \"{}\") = \"{}\"",
        empty, non_empty, result
    );

    // 9. 相同长度的字符串
    println!("\n9. 相同长度的字符串:");
    let s1 = "equal";
    let s2 = "size";
    let result = longest(s1, s2);
    println!("  longest(\"{}\", \"{}\") = \"{}\"", s1, s2, result);

    // 10. 单词边界
    println!("\n10. 单词边界:");
    let s = "hello world rust programming";
    let word = first_word(s);
    println!("  first_word(\"{}\") = \"{}\"", s, word);
}
