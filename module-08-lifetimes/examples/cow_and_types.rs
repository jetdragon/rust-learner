// examples/cow_and_types.rs
// Cow 和高级类型演示

use module_08_lifetimes::{
    get_static_str, maybe_clone, parse_string, to_uppercase_cow, SplitIter, GREETING,
};

use std::borrow::Cow;

fn main() {
    println!("=== Cow 和高级类型演示 ===\n");

    // 1. to_uppercase_cow
    println!("1. to_uppercase_cow:");
    let s = "UPPER";
    let result = to_uppercase_cow(s);
    println!("  input: \"{}\"", s);
    println!("  result: \"{}\"", result);
    println!("  is borrowed: {}", matches!(result, Cow::Borrowed(..)));

    let s = "lower";
    let result = to_uppercase_cow(s);
    println!("  input: \"{}\"", s);
    println!("  result: \"{}\"", result);
    println!("  is owned: {}", matches!(result, Cow::Owned(..)));

    // 2. maybe_clone
    println!("\n2. maybe_clone:");
    let s = "test string";
    let result = maybe_clone(s, false);
    println!("  should_clone=false: {:?}", result);
    let result = maybe_clone(s, true);
    println!("  should_clone=true: {:?}", result);

    // 3. SplitIter
    println!("\n3. SplitIter:");
    let text = "apple,banana,cherry,date";
    let iter = SplitIter::new(text, ',');
    for (i, part) in iter.enumerate() {
        println!("  part {}: \"{}\"", i + 1, part);
    }

    // 4. 静态生命周期
    println!("\n4. 静态生命周期:");
    let s = get_static_str();
    println!("  get_static_str(): \"{}\"", s);
    println!("  GREETING: \"{}\"", GREETING);

    // 5. 静态引用
    println!("\n5. 静态引用:");
    let s: &'static str = "I live forever!";
    println!("  static str: \"{}\"", s);

    // 6. parse_string (类型别名)
    println!("\n6. parse_string (类型别名):");
    match parse_string("valid string") {
        Ok(s) => println!("  parsed: \"{}\"", s),
        Err(e) => println!("  error: {}", e),
    }

    match parse_string("") {
        Ok(s) => println!("  parsed: \"{}\"", s),
        Err(e) => println!("  error: {}", e),
    }

    // 7. Cow 性能演示
    println!("\n7. Cow 性能演示:");
    let data = vec!["UPPER", "LOWER", "MIXED", "ALLCAPS"];
    for s in data {
        let result = to_uppercase_cow(s);
        let borrowed = matches!(result, Cow::Borrowed(..));
        println!("  \"{}\" -> \"{}\" (borrowed: {})", s, result, borrowed);
    }

    // 8. SplitIter 与不同分隔符
    println!("\n8. SplitIter 与不同分隔符:");
    let text = "one|two|three|four";
    let iter = SplitIter::new(text, '|');
    let parts: Vec<&str> = iter.collect();
    println!("  parts: {:?}", parts);

    // 9. 空字符串处理
    println!("\n9. 空字符串处理:");
    let text = "";
    let iter = SplitIter::new(text, ',');
    let parts: Vec<&str> = iter.collect();
    println!("  empty text parts: {:?}", parts);

    // 10. 连续分隔符
    println!("\n10. 连续分隔符:");
    let text = "a,,b,,c";
    let iter = SplitIter::new(text, ',');
    let parts: Vec<&str> = iter.collect();
    println!("  parts: {:?}", parts);

    // 11. 字符串结尾分隔符
    println!("\n11. 字符串结尾分隔符:");
    let text = "a,b,c,";
    let iter = SplitIter::new(text, ',');
    let parts: Vec<&str> = iter.collect();
    println!("  parts: {:?}", parts);

    // 12. Cow 与字符串比较
    println!("\n12. Cow 与字符串比较:");
    let s1 = "hello";
    let s2 = "world";
    let cow1 = to_uppercase_cow(s1);
    let cow2 = to_uppercase_cow(s2);
    println!("  \"{}\" + \" {}\" = \"{} {}\"", s1, s2, cow1, cow2);
}
