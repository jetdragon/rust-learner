//! # 练习 1: Trait 定义 - 解答
//!
//! 难度: 简单
//! 时间: 10 分钟
//! 前置知识: struct, impl 块
//! 学习目标:
//!   - 定义自定义 trait
//!   - 为类型实现 trait
//!   - 理解 trait bound 的基本概念

/// Summary trait - 要求类型能够生成摘要信息
pub trait Summary {
    /// 返回该类型的摘要字符串
    fn summarize(&self) -> String;
}

/// NewsArticle 结构体 - 表示新闻文章
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

/// Tweet 结构体 - 表示推文
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// ============= 解决方案 =============

/// 为 NewsArticle 实现 Summary trait
///
/// 实现说明:
/// - 使用 format! 宏创建格式化字符串
/// - 格式: "[headline] by [author] in [location]"
/// - 这展示了如何为自定义类型实现 trait
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{} by {} in {}",
            self.headline, self.author, self.location
        )
    }
}

/// 为 Tweet 实现 Summary trait
///
/// 实现说明:
/// - 格式: "[username]: [content]"
/// - 展示了另一种摘要格式
///
/// # 替代实现
/// ```rust
/// // 可以根据 reply/retweet 状态提供不同的摘要
/// impl Summary for Tweet {
///     fn summarize(&self) -> String {
///         let prefix = if self.retweet {
///             "RT"
///         } else if self.reply {
///             "Reply"
///         } else {
///             "Tweet"
///         };
///         format!("{}: {}: {}", prefix, self.username, self.content)
///     }
/// }
/// ```
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// ============= 测试代码 =============

fn main() {
    // 测试 NewsArticle
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available!");
    println!("Article summary: {}", article.summarize());
    println!();

    // 测试 Tweet
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    // 测试 trait bound 的使用
    println!("\n=== Trait Bound 示例 ===");
    notify(&tweet);
    notify(&article);
}

/// 使用 trait bound 的函数示例
///
/// 这个函数接受任何实现了 Summary trait 的类型
/// 这展示了 trait bound 的实际应用
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// ============= 扩展知识 =============

/*
Trait 定义的关键点:
1. Trait 定义了一组方法签名
2. 类型通过实现这些方法来实现 trait
3. 一个类型可以实现多个 trait
4. Trait 可以有默认实现（见下一个练习）

Trait Bound 的语法:
- impl Trait 语法 (简单)
- <T: Trait> 语法 (完整)
- where 子句 (复杂约束)

为什么使用 Trait:
1. 定义共享行为
2. 泛型约束
3. 运行时多态（trait 对象）
*/
