// examples/struct_lifetimes.rs
// 结构体生命周期演示

use module_08_lifetimes::{Context, ImportantExcerpt};

fn main() {
    println!("=== 结构体生命周期演示 ===\n");

    // 1. Context 结构体
    println!("1. Context 结构体:");
    let name = String::from("MyContext");
    let description = String::from("This is a test context");
    let ctx = Context::new(&name, &description);
    println!("  name: {}", ctx.name());
    println!("  description: {}", ctx.description());
    println!("  display: {}", ctx.display());

    // 2. 字符串字面量
    println!("\n2. 字符串字面量:");
    let ctx = Context::new("StaticName", "StaticDescription");
    println!("  name: {}", ctx.name());
    println!("  description: {}", ctx.description());

    // 3. ImportantExcerpt 结构体
    println!("\n3. ImportantExcerpt 结构体:");
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let excerpt = ImportantExcerpt::new(first_sentence);
    println!("  excerpt: \"{}\"", excerpt.part());
    println!("  level: {}", excerpt.level());

    // 4. announce_and_return_part 方法
    println!("\n4. announce_and_return_part 方法:");
    let excerpt = ImportantExcerpt::new("Hello, world!");
    let announcement = "Attention please!";
    let part = excerpt.announce_and_return_part(announcement);
    println!("  returned part: \"{}\"", part);

    // 5. 混合使用
    println!("\n5. 混合使用:");
    let text = String::from("Learning Rust is fun!");
    let ctx = Context::new("Learning", &text);
    let excerpt = ImportantExcerpt::new(&text);
    println!("  context: {}", ctx.display());
    println!("  excerpt: \"{}\"", excerpt.part());

    // 6. 多个上下文
    println!("\n6. 多个上下文:");
    let ctx1 = Context::new("Context1", "Description 1");
    let ctx2 = Context::new("Context2", "Description 2");
    println!("  ctx1: {}", ctx1.display());
    println!("  ctx2: {}", ctx2.display());

    // 7. 生命周期和作用域
    println!("\n7. 生命周期和作用域:");
    let string1 = String::from("Long-lived string");
    {
        let string2 = String::from("Short-lived string");
        let ctx = Context::new(&string1, &string2);
        println!("  inner context: {}", ctx.display());
    }
    // ctx 在这里不可用，因为 string2 已经被释放
    println!("  outer string still exists: {}", string1);

    // 8. 静态引用
    println!("\n8. 静态引用:");
    let ctx = Context::new("Static", "Static context");
    println!("  static context: {}", ctx.display());
}
