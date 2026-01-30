// examples/advanced_lifetimes.rs
// 高级生命周期演示

use module_08_lifetimes::{choose_first, Pair, RefWrapper, StringBuilder, TextProcessor};

fn main() {
    println!("=== 高级生命周期演示 ===\n");

    // 1. 多个生命周期参数
    println!("1. 多个生命周期参数:");
    let s1 = String::from("first");
    let s2 = "second";
    let result = choose_first(&s1, s2);
    println!("  choose_first(\"{}\", \"{}\") = \"{}\"", s1, s2, result);

    // 2. Pair 结构体
    println!("\n2. Pair 结构体:");
    let a = 10;
    let b = 20;
    let pair = Pair::new(&a, &b);
    println!("  pair.first() = {}", pair.first());
    println!("  pair.second() = {}", pair.second());
    println!("  pair.swap() = {:?}", pair.swap());

    // 3. Pair 与字符串
    println!("\n3. Pair 与字符串:");
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let pair = Pair::new(&s1, &s2);
    println!("  pair.first() = {}", pair.first());
    println!("  pair.second() = {}", pair.second());

    // 4. StringBuilder
    println!("\n4. StringBuilder:");
    let result = StringBuilder::new()
        .append("Hello")
        .append(", ")
        .append("World")
        .append("!")
        .build();
    println!("  result: {}", result);

    // 5. 条件追加
    println!("\n5. 条件追加:");
    let result = StringBuilder::new()
        .append("Start")
        .append_if(", Middle", true)
        .append_if(", Skipped", false)
        .append(", End")
        .build();
    println!("  result: {}", result);

    // 6. TextProcessor
    println!("\n6. TextProcessor:");
    let text = "  Hello, World!  ";
    let processor = TextProcessor::new(text);
    println!("  original: \"{}\"", text);
    println!("  trim: \"{}\"", processor.trim());
    println!("  left(5): \"{}\"", processor.left(5));
    println!("  right(6): \"{}\"", processor.right(6));

    // 7. TextProcessor 方法
    println!("\n7. TextProcessor 方法:");
    let text = "The quick brown fox jumps over the lazy dog";
    let processor = TextProcessor::new(text);
    println!("  contains(\"fox\"): {}", processor.contains("fox"));
    println!("  starts_with(\"The\"): {}", processor.starts_with("The"));
    println!("  ends_with(\"dog\"): {}", processor.ends_with("dog"));

    // 8. 文本行
    println!("\n8. 文本行:");
    let text = "Line 1\nLine 2\nLine 3";
    let processor = TextProcessor::new(text);
    let lines = processor.lines();
    println!("  lines: {:?}", lines);

    // 9. RefWrapper
    println!("\n9. RefWrapper:");
    let value = 42;
    let wrapper = RefWrapper::new(&value);
    println!("  wrapper.get() = {}", wrapper.get());
    println!("  wrapper.equals(&42) = {}", wrapper.equals(&42));

    // 10. RefWrapper map
    println!("\n10. RefWrapper map:");
    let value = 10;
    let wrapper = RefWrapper::new(&value);
    let doubled = wrapper.map(|v| v * 2);
    println!("  doubled: {}", doubled);

    // 11. 生命周期子类型
    println!("\n11. 生命周期子类型:");
    let s1 = String::from("long-lived");
    {
        let s2 = String::from("short-lived");
        let result = choose_first(&s1, &s2);
        println!("  choose_first 返回: \"{}\"", result);
    }

    // 12. 泛型 Pair
    println!("\n12. 泛型 Pair:");
    let words = vec!["hello", "world", "rust"];
    let pair = Pair::new(&words[0], &words[1]);
    println!("  words pair: ({}, {})", pair.first(), pair.second());
}
