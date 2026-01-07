//! # 练习 2: Trait 默认实现 - 解答
//!
//! 难度: 简单
//! 时间: 10 分钟
//! 前置知识: trait 基础
//! 学习目标:
//!   - 为 trait 方法提供默认实现
//!   - 理解默认实现可以被覆盖

/// Summary trait - 展示默认实现的概念
///
/// 关键点:
/// - summarize() 有默认实现
/// - summarize_authored() 依赖于 get_author()
/// - get_author() 没有默认实现，必须由类型提供
pub trait Summary {
    /// 默认实现 - 返回通用摘要
    ///
    /// 实现此 trait 的类型可以选择:
    /// 1. 使用这个默认实现
    /// 2. 覆盖它提供自定义实现
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }

    /// 使用 get_author() 的方法
    ///
    /// 这个方法展示了默认实现可以调用其他必须实现的方法
    fn summarize_authored(&self) -> String {
        format!("(Read more from {}...)", self.get_author())
    }

    /// 必须实现的方法 - 没有默认实现
    ///
    /// 这是类型的"契约" - 必须提供这个方法
    fn get_author(&self) -> String;
}

/// Post 结构体 - 博客文章
pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}

// ============= 解决方案 =============

/// 为 Post 实现 Summary trait
///
/// 实现说明:
/// - 只实现 get_author() 方法
/// - 使用 summarize() 的默认实现
/// - 自动获得 summarize_authored() 方法
///
/// 这展示了默认实现的优势:
/// 1. 减少重复代码
/// 2. 提供通用行为
/// 3. 允许选择性覆盖
impl Summary for Post {
    // 只实现必须的方法
    fn get_author(&self) -> String {
        self.author.clone()
    }

    // 可选: 覆盖默认实现以提供更好的摘要
    fn summarize(&self) -> String {
        format!("{}: {}", self.title, self.get_author())
    }
}

// ============= 测试代码 =============

fn main() {
    let post = Post {
        title: String::from("Rust Traits"),
        author: String::from("Jane Doe"),
        content: String::from("Traits are awesome..."),
    };

    println!("=== 使用默认实现 ===");
    println!("Post summary: {}", post.summarize());
    println!("Post summary (authored): {}", post.summarize_authored());
}

// ============= 扩展示例 =============

/// 另一个实现示例 - 完全使用默认实现
struct SimplePost {
    author: String,
}

impl Summary for SimplePost {
    fn get_author(&self) -> String {
        self.author.clone()
    }
    // 不覆盖 summarize()，使用默认实现
}

/// 覆盖所有方法的示例
struct DetailedPost {
    title: String,
    author: String,
    word_count: usize,
}

impl Summary for DetailedPost {
    fn get_author(&self) -> String {
        self.author.clone()
    }

    // 覆盖默认实现
    fn summarize(&self) -> String {
        format!(
            "\"{}\" by {} ({} words)",
            self.title, self.author, self.word_count
        )
    }

    // 也可以覆盖其他方法
    fn summarize_authored(&self) -> String {
        format!(
            "Read more from {} (author of \"{}\")",
            self.author, self.title
        )
    }
}

// ============= 扩展知识 =============

/*
默认实现的规则:
1. 可以为任何 trait 方法提供默认实现
2. 默认实现可以调用同一 trait 中的其他方法
3. 实现者可以选择使用默认实现或覆盖它
4. 调用默认实现的方法时，使用的是该类型的实现

设计模式:
1. 模板方法模式: 定义算法骨架，某些步骤由子类实现
2. 最小接口原则: 只要求实现必要的方法
3. 渐进式增强: 提供基本功能，允许增强

实际应用示例:
- Iterator trait: 提供 fold、map 等基于 next() 的方法
- From trait: 提供 Into 的默认实现
*/
