//! # 练习 6: 多个生命周期参数 - 解答
//!
//! 难度: 中等
//! 时间: 15 分钟
//! 前置知识: 生命周期标注
//! 学习目标:
//!   - 使用多个生命周期参数
//!   - 理解不同生命周期之间的关系

// ============= 解决方案 =============

/// format_with_title - 使用多个生命周期参数
///
/// 生命周期说明:
/// - 'a: title 的生命周期
/// - 'b: content 的生命周期
/// - where 'a: 'b 表示 'a 必须至少和 'b 一样长（子类型关系）
///
/// 为什么需要约束:
/// - 返回 String（拥有类型），不直接依赖引用
/// - 但创建新 String 时需要确保引用有效
/// - where 子句确保 title 在 content 之前或同时失效
pub fn format_with_title<'a, 'b>(title: &'a str, content: &'b str) -> String
where
    'a: 'b,
{
    format!("=== {} ===\n{}", title, content)
}

/// Document 结构体 - 持有不同生命周期的引用
pub struct Document<'a, 'b> {
    pub title: &'a str,
    pub content: &'b str,
}

impl<'a, 'b> Document<'a, 'b> {
    pub fn new(title: &'a str, content: &'b str) -> Self {
        Document { title, content }
    }

    /// get_summary 方法
    ///
    /// 返回值应该使用哪个生命周期？
    /// 答案: 返回 String（拥有类型），所以不需要生命周期
    /// 但需要 where 子句确保引用在函数调用时有效
    pub fn get_summary(&self) -> String
    where
        'a: 'b,
    {
        format!("{}: {}", self.title, self.content)
    }
}

// ============= 扩展示例 =============

/// 生命周期子类型关系示例
///
/// 'a: 'b 表示 'a 至少和 'b 一样长
/// 或者说 'a 是 'b 的子类型
fn demonstrate_subtyping<'a, 'b>(long: &'a str, short: &'b str)
where
    'a: 'b,
{
    // 可以将 'a 传给期望 'b 的函数
    use_short(short);
    use_short(long);  // OK，因为 'a: 'b

    // 但不能反过来
    // use_long(short);  // 错误！
    use_long(long);
}

fn use_short(_: &str) {
    println!("Using short lifetime reference");
}

fn use_long(_: &str) {
    println!("Using long lifetime reference");
}

/// 多个生命周期返回引用
///
/// 更复杂的情况 - 返回引用
struct Context<'a> {
    data: &'a str,
}

impl<'a> Context<'a> {
    /// 返回 self 的引用（'a）
    pub fn get_data(&self) -> &str {
        self.data
    }

    /// 返回参数的引用（'b）
    pub fn compare<'b>(&self, other: &'b str, prefer_other: bool) -> &str
    where
        'a: 'b,
    {
        if prefer_other {
            other  // 返回 'b
        } else {
            self.data  // 返回 'a，可以转换为 'b 因为 'a: 'b
        }
    }
}

// ============= 测试代码 =============

fn main() {
    println!("=== format_with_title 示例 ===");

    let title = String::from("My Document");
    let content = String::from("This is the content.");

    let result = format_with_title(&title, &content);
    println!("{}\n", result);

    println!("=== Document 示例 ===");
    let title = String::from("My Document");
    let content = String::from("This is the content.");

    {
        let local_content = String::from("Short content");
        let doc = Document::new(&title, &local_content);
        println!("Summary: {}", doc.get_summary());
    }

    println!("=== 生命周期子类型 ===");
    let long_lived = String::from("I live long");
    let short_lived = String::from("I'm short");

    demonstrate_subtyping(&long_lived, &short_lived);

    println!("\n=== Context 示例 ===");
    let data = String::from("Context data");
    let ctx = Context::new(&data);

    let other = String::from("Other data");
    let result = ctx.compare(&other, false);
    println!("Prefer self: {}", result);

    let result = ctx.compare(&other, true);
    println!("Prefer other: {}", result);

    println!("\n=== 实际应用 ===");
    practical_example();
}

/// 实际应用示例
fn practical_example() {
    // 配置解析器
    struct Config<'a, 'b> {
        name: &'a str,
        value: &'b str,
    }

    impl<'a, 'b> Config<'a, 'b> {
        /// 创建配置，要求 name 比 value 活得更久
        pub fn new(name: &'a str, value: &'b str) -> Self
        where
            'a: 'b,
        {
            Config { name, value }
        }

        /// 格式化配置
        pub fn format(&self) -> String
        where
            'a: 'b,
        {
            format!("{} = {}", self.name, self.value)
        }
    }

    let name = String::from("host");
    let value = String::from("localhost");
    let config = Config::new(&name, &value);
    println!("Config: {}", config.format());

    // 为什么需要 'a: 'b?
    // - 如果我们想缓存 format 的结果，需要确保 name 比 value 活得更久
    // - 这保证了即使 value 失效，基于 name 的缓存仍然有效
}

// ============= 多生命周期规则 =============

/*
多个生命周期参数的规则:

1. 声明多个生命周期:
   fn func<'a, 'b>(x: &'a str, y: &'b str)

2. 生命周期子类型关系:
   where 'a: 'b  // 'a 至少和 'b 一样长

3. 生命周期独立性:
   - 'a 和 'b 可以完全独立
   - 不需要有关系

4. 返回值的选择:
   - 返回哪个参数的引用，就用哪个生命周期
   - 返回多个引用时，选择最短的

子类型关系:

'a: 'b  // 'a 是 'b 的子类型
- 'a 活得至少和 'b 一样长
- 可以将 'a 传给期望 'b 的函数
- 类似于继承关系

使用场景:

1. 参数有不同生命周期:
   fn process<'a, 'b>(x: &'a str, y: &'b str)

2. 需要生命周期约束:
   fn process<'a, 'b>(x: &'a str, y: &'b str)
   where
       'a: 'b,

3. 返回值依赖约束:
   fn choose<'a, 'b>(x: &'a str, y: &'b str, prefer_x: bool) -> &'a str
   where
       'a: 'b,

实际应用:

1. 结构体多个引用:
   struct Pair<'a, 'b> {
       first: &'a str,
       second: &'b str,
   }

2. 链式引用:
   struct Chain<'a, 'b, 'c> {
       first: &'a str,
       second: &'b str,
       third: &'c str,
   }

3. 层级关系:
   struct Node<'a, 'b> {
       name: &'a str,
       value: &'b str,
   }
   where 'a: 'b  // name 比 value 活得更久

最佳实践:

1. 只在必要时使用多个生命周期
2. 清楚地标明生命周期关系
3. 使用有意义的生命周期名称
4. 优先使用生命周期省略规则
*/
