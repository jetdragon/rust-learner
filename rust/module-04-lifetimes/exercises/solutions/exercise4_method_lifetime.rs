//! # 练习 4: 方法的生命周期 - 解答
//!
//! 难度: 中等
//! 时间: 15 分钟
//! 前置知识: 结构体生命周期
//! 学习目标:
//!   - 理解 impl 块中的生命周期声明
///   - 区分 self 的生命周期和其他参数的生命周期

/// Context 结构体 - 持有字符串引用
///
/// 这个结构体有生命周期参数 'a
pub struct Context<'a> {
    data: &'a str,
}

impl<'a> Context<'a> {
    /// 创建新的 Context
    ///
    /// 方法不需要额外的生命周期参数
    /// 使用结构体的生命周期 'a
    pub fn new(data: &'a str) -> Self {
        Context { data }
    }

    /// get_data 方法
    ///
    /// 返回值生命周期是什么？
    /// 答案: 'a（与 self 相同）
    ///
    /// 生命周期省略规则应用:
    /// - 规则 1: 输入参数 &self 有生命周期 'a
    /// - 规则 3: 因为有 &self，返回值生命周期赋给 self
    /// - 推断结果: fn get_data(&'a self) -> &'a str
    pub fn get_data(&self) -> &str {
        self.data
    }

    /// compare_with_input 方法
    ///
    /// 这个方法与上面有什么不同？
    /// 答案: 有第二个参数，需要不同的生命周期
    ///
    /// 完整签名:
    /// fn compare_with_input<'b>(&self, input: &'b str) -> bool
    ///
    /// 生命周期说明:
    /// - 'a 是 self 的生命周期（结构体的生命周期）
    /// - 'b 是 input 的生命周期（独立于 self）
    /// - 返回 bool，不涉及引用，不需要生命周期
    pub fn compare_with_input<'b>(&self, input: &'b str) -> bool {
        self.data == input
    }
}

// ============= 扩展示例 =============

/// 展示方法中生命周期的各种情况
impl<'a> Context<'a> {
    /// 返回 self 引用的方法
    /// 省略规则适用，返回 'a
    pub fn get_prefix(&self, count: usize) -> &str {
        &self.data[..count.min(self.data.len())]
    }

    /// 比较两个字符串切片的方法
    /// 需要独立的生命周期
    pub fn contains<'b>(&self, other: &'b str) -> bool {
        self.data.contains(other)
    }

    /// 返回与参数相同生命周期的引用
    /// 这个比较特殊
    pub fn compare_and_return<'b>(&self, other: &'b str) -> &'b str {
        if self.data.len() > other.len() {
            self.data  // 错误！应该返回 'a 不是 'b
        } else {
            other
        }
    }
}

// 修正上面的方法 - 需要相同的生命周期
impl<'a> Context<'a> {
    /// 修正版本 - 要求相同生命周期
    pub fn compare_and_return_fixed(&self, other: &str) -> &str {
        if self.data.len() > other.len() {
            self.data
        } else {
            other
        }
    }
}

// ============= 测试代码 =============

fn main() {
    println!("=== 基础方法示例 ===");

    let text = String::from("Hello, World!");
    let ctx = Context::new(&text);

    println!("Data: {}", ctx.get_data());

    println!("\n=== compare_with_input 示例 ===");
    let text = String::from("Hello, World!");
    let ctx = Context::new(&text);

    println!("Match: {}", ctx.compare_with_input("Hello, World!"));
    println!("Match: {}", ctx.compare_with_input("Goodbye"));

    println!("\n=== 不同生命周期 ===");

    // 展示不同生命周期的参数
    let long_lived = String::from("Long lived string");
    let ctx = Context::new(&long_lived);

    {
        let short_lived = String::from("Short");
        let result = ctx.compare_with_input(&short_lived);
        println!("Comparison result: {}", result);
        // short_lived 在这里失效，但没关系
    }

    println!("Long lived still valid: {}", long_lived);

    println!("\n=== get_prefix 示例 ===");
    let text = "Hello, World!";
    let ctx = Context::new(text);
    println!("First 5 chars: {}", ctx.get_prefix(5));

    println!("\n=== contains 示例 ===");
    let text = "The quick brown fox";
    let ctx = Context::new(text);
    println!("Contains 'quick': {}", ctx.contains("quick"));
    println!("Contains 'slow': {}", ctx.contains("slow"));

    println!("\n=== 实际应用 ===");
    practical_example();
}

/// 实际应用示例
fn practical_example() {
    // 解析器示例
    struct Parser<'a> {
        input: &'a str,
        position: usize,
    }

    impl<'a> Parser<'a> {
        pub fn new(input: &'a str) -> Self {
            Parser {
                input,
                position: 0,
            }
        }

        /// 获取当前位置
        pub fn current(&self) -> &str {
            &self.input[self.position..]
        }

        /// 检查是否匹配某个字符串
        pub fn matches(&self, pattern: &str) -> bool {
            self.current().starts_with(pattern)
        }

        /// 高级方法 - 返回输入的生命周期
        pub fn consume<'b>(&'b mut self, count: usize) -> &'b str {
            let start = self.position;
            self.position += count.min(self.current().len());
            &self.input[start..self.position]
        }
    }

    let input = "Hello, World!";
    let mut parser = Parser::new(input);

    println!("Current: {}", parser.current());
    println!("Matches 'Hello': {}", parser.matches("Hello"));

    let consumed = parser.consume(5);
    println!("Consumed: '{}', Remaining: '{}'", consumed, parser.current());
}

// ============= 方法生命周期总结 =============

/*
方法中的生命周期规则:

1. impl 块声明:
   impl<'a> Struct<'a> { }
   - 生命周期参数必须在 impl 后声明
   - 在结构体名后使用

2. self 的生命周期:
   - &self 的生命周期与结构体的生命周期相同
   - &mut self 同样

3. 方法参数:
   - 可以有独立的生命周期参数
   - 不需要与结构体的生命周期相同

4. 返回值:
   - 通常返回 self 的生命周期（规则 3）
   - 如果返回其他参数的引用，需要明确标注

常见模式:

模式 1: 返回 self 的引用
fn get(&self) -> &str  // 返回 'a

模式 2: 接受独立生命周期的参数
fn compare(&self, other: &str) -> bool  // 'a vs 'b

模式 3: 返回参数的引用
fn process<'b>(&self, data: &'b str) -> &'b str

模式 4: 链式调用
fn method(&self) -> &Self  // 返回 self 引用

错误示例:

❌ 不能返回比 self 活得更久的引用
fn bad(&self) -> &'static str {
    self.data  // 错误: self.data 不是 'static
}

❌ 不能混合不兼容的生命周期
fn bad<'b>(&self, other: &'b str) -> &'a str {
    other  // 错误: 返回 'b 但声明 'a
}

最佳实践:

1. 尽量使用省略规则
2. 只在必要时显式标注
3. 使用有意义的生命周期名称
4. 保持方法签名简单清晰
*/
