//! # 练习 2: 结构体生命周期 - 解答
//!
//! 难度: 简单
//! 时间: 10 分钟
//! 前置知识: 生命周期参数基础
//! 学习目标:
//!   - 在结构体定义中使用生命周期
//!   - 理解结构体中的引用有效性

/// ImportantExcerpt 结构体 - 持有字符串引用
///
/// 生命周期说明:
/// - <'a> 声明生命周期参数
/// - part: &'a str 表示 part 字段的生命周期是 'a
/// - ImportantExcerpt 实例不能比它持有的引用活得更久
///
/// 含义:
/// - 结构体的生命周期由它持有的引用决定
/// - 不能创建包含失效引用的结构体实例
pub struct ImportantExcerpt<'a> {
    pub part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    /// 返回级别（不涉及引用，不需要生命周期）
    pub fn level(&self) -> i32 {
        3
    }

    /// announce_and_return_part 方法
    ///
    /// 这个方法需要生命周期参数吗？
    /// 答案: 不需要显式标注！
    ///
    /// 原因 - 生命周期省略规则:
    /// 1. &self 的生命周期是 'a
    /// 2. announcement 是独立的引用
    /// 3. 返回 &self.part，生命周期与 self 相同
    /// 4. 编译器可以自动推断
    ///
    /// 如果手动标注会是这样:
    /// pub fn announce_and_return_part<'b>(&self, announcement: &'b str) -> &'a str
    pub fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// ============= 扩展示例 =============

/// 多个引用的结构体
///
/// 展示了结构体可以持有多个不同生命周期的引用
pub struct Book<'a> {
    title: &'a str,
    author: &'a str,
    // 如果需要不同的生命周期:
    // pub reference<'b>: &'b str,
}

impl<'a> Book<'a> {
    pub fn new(title: &'a str, author: &'a str) -> Self {
        Book { title, author }
    }

    pub fn get_title(&self) -> &str {
        self.title
    }
}

/// 泛型加生命周期
///
/// 展示了同时使用泛型类型和生命周期
pub struct Holder<'a, T: ?Sized> {
    value: &'a T,
}

impl<'a, T: ?Sized> Holder<'a, T> {
    pub fn new(value: &'a T) -> Self {
        Holder { value }
    }

    pub fn get(&self) -> &'a T {
        self.value
    }
}

// ============= 测试代码 =============

fn main() {
    println!("=== 基础结构体生命周期 ===");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };

    println!("Excerpt: {}", excerpt.part);
    println!("Level: {}", excerpt.level());

    println!("\n=== 方法调用 ===");
    let result = excerpt.announce_and_return_part("Hey there!");
    println!("Returned: {}", result);

    println!("\n=== 不同作用域 ===");
    demonstrate_scopes();

    println!("\n=== Book 示例 ===");
    let title = String::from("The Rust Book");
    let author = String::from("Steve Klabnik");
    let book = Book::new(&title, &author);
    println!("Book: {} by {}", book.get_title(), author);

    println!("\n=== Holder 示例 ===");
    let number = 42;
    let holder = Holder::new(&number);
    println!("Holder value: {}", holder.get());
}

/// 展示作用域对结构体生命周期的限制
fn demonstrate_scopes() {
    let novel = String::from("Outside. The air was cool.");

    {
        let first_sentence = novel.split('.').next().unwrap();
        let excerpt = ImportantExcerpt {
            part: first_sentence,
        };
        println!("Inner excerpt: {}", excerpt.part);
        // excerpt 在这里失效，但这是可以的
        // 因为 first_sentence 也在这个作用域内
    }

    // 如果这样写会编译错误:
    // let excerpt;
    // {
    //     let first_sentence = novel.split('.').next().unwrap();
    //     excerpt = ImportantExcerpt { part: first_sentence };
    // } // first_sentence 在这里失效
    // println!("{}", excerpt.part); // excerpt 持有失效的引用

    println!("Correct: novel still valid: {}", novel);
}

// ============= 扩展知识 =============

/*
结构体生命周期规则:

1. 基本语法:
   struct Name<'a> {
       field: &'a Type,
   }

2. 多个生命周期:
   struct Name<'a, 'b> {
       field1: &'a str,
       field2: &'b str,
   }

3. 生命周期加泛型:
   struct Name<'a, T> {
       field: &'a T,
   }

4. Self 引用:
   impl<'a> Type<'a> {
       fn method(&self) -> &'a str { ... }
   }

关键点:

✅ 结构体的生命周期是它持有引用的最小生命周期
✅ 结构体实例不能比它持有的引用活得更久
✅ 编译器确保引用在结构体使用时仍然有效

❌ 不能创建包含失效引用的结构体

验证方法:

1. 编译器借用检查
2. 确保所有引用在使用时有效
3. 生命周期参数确保正确的关系

实际应用:

1. 解析结果持有输入的引用
2. 配置结构体持有字符串引用
3. 视图类型（slice、str、slice）

零成本抽象:
- 生命周期只是编译时检查
- 运行时没有开销
- 不影响性能
*/
