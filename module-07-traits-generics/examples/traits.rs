// examples/traits.rs
// Trait 基础演示

use module_07_traits_generics::{
    notify, notify_explicit, notify_where, return_summarizable, NewsArticle, Summary, Tweet,
};

fn main() {
    println!("=== Trait 基础演示 ===\n");

    // 1. Trait 默认实现
    println!("1. Trait 默认实现:");
    let article = NewsArticle {
        headline: String::from("Penguins win!"),
        location: String::from("Pittsburgh"),
        author: String::from("Iceburgh"),
        content: String::from("The Penguins are champions!"),
    };
    println!("  摘要: {}", article.summarize());
    println!("  作者: {}", article.author());

    // 2. Trait 方法调用
    println!("\n2. Trait 方法调用:");
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("  摘要: {}", tweet.summarize());
    println!("  作者: {}", tweet.author());

    // 3. impl Trait Bound
    println!("\n3. impl Trait Bound:");
    notify(&article);

    // 4. 完整 Trait Bound 语法
    println!("\n4. 完整 Trait Bound 语法:");
    notify_explicit(&tweet);

    // 5. where 子句
    println!("\n5. where 子句:");
    notify_where(&article);

    // 6. 返回 impl Trait
    println!("\n6. 返回 impl Trait:");
    let summary = return_summarizable();
    println!("  摘要: {}", summary.summarize());

    // 7. Trait 作为 Trait Bound
    println!("\n7. Trait 作为 Trait Bound:");
    let items: Vec<Box<dyn Summary>> = vec![Box::new(article), Box::new(tweet)];
    for item in items {
        println!("  摘要: {}", item.summarize());
    }
}
