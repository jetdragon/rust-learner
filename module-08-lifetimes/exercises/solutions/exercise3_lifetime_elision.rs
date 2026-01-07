//! # 练习 3: 生命周期省略规则 - 解答
//!
//! 难度: 简单
//! 时间: 10 分钟
//! 前置知识: 生命周期标注
//! 学习目标:
//!   - 理解生命周期省略的三条规则
//!   - 识别何时需要显式标注生命周期

// ============= 解决方案 =============

/// first_word 函数
///
/// 这个函数需要生命周期标注吗？
/// 答案: 不需要！编译器可以自动推断
///
/// 生命周期省略规则的应用:
/// 1. 规则 1: 每个引用参数有自己的生命周期
///    - 编译器推断: fn first_word<'a>(s: &'a str) -> &'a str
/// 2. 规则 2: 如果只有一个输入生命周期，赋给所有输出
///    - 因为只有一个参数 s，返回值生命周期也是 'a
///
/// 完整推断过程:
/// - 原始: fn first_word(s: &str) -> &str
/// - 规则 1 应用后: fn first_word<'a>(s: &'a str) -> &str
/// - 规则 2 应用后: fn first_word<'a>(s: &'a str) -> &'a str
pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

/// parse_command 函数
///
/// 这个函数需要生命周期标注吗？
/// 答案: 不需要！
///
/// 生命周期省略规则的应用:
/// 1. 规则 1: 每个参数有自己的生命周期
///    - 编译器推断: fn parse_command<'a, 'b>(input: &'a str) -> Result<&'a str, &'b str>
/// 2. 规则 3: 如果方法中有 &self 或 &mut self，返回生命周期赋给 self
///    - 这里不适用（不是方法）
/// 3. 对于 Result，两个分支都遵循规则 2
///    - Ok 分支返回 &input，所以是 'a
///    - Err 分文返回字符串字面量，可以是任何生命周期
///
/// 实际推断:
/// - input: &'a str
/// - Ok(...): &'a str（与输入相同）
/// - Err(...): 可以是 'static（字符串字面量）
pub fn parse_command(input: &str) -> Result<&str, &str> {
    if input.starts_with("/") {
        Ok(&input[1..])
    } else {
        Err("invalid command")
    }
}

// ============= 需要显式标注的例子 =============

/// 这个函数需要显式生命周期标注
///
/// 为什么: 有两个输入参数，编译器不知道返回哪个
///
/// 如果不标注，编译器无法确定:
/// - 返回 x 的生命周期？
/// - 返回 y 的生命周期？
/// - 需要明确的约束
pub fn needs_annotation<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// ============= 生命周期省略规则详解 =============

/*
三条生命周期省略规则:

规则 1: 每个引用参数都有自己的生命周期参数
- 编译器为每个 & 或 &mut 参数分配生命周期
- 例子: fn foo(x: &str, y: &str) 变成 fn foo<'a, 'b>(x: &'a str, y: &'b str)

规则 2: 如果只有一个输入生命周期参数，赋给所有输出
- 例子: fn foo(x: &str) -> &str 变成 fn foo<'a>(x: &'a str) -> &'a str
- 这是最常见的省略情况

规则 3: 如果有多个输入生命周期参数，但其中一个是 &self 或 &mut self，
       赋给所有输出生命周期参数
- 只适用于方法
- 例子: fn method(&self, x: &str) -> &str 变成 fn method<'a, 'b>(&'a self, x: &'b str) -> &'a str

规则应用顺序:
1. 先应用规则 1（输入参数）
2. 再应用规则 2（单一输入）
3. 最后应用规则 3（方法中的 self）

什么时候需要显式标注:
- 多个引用参数，返回其中之一
- 返回的引用与参数关系复杂
- 编译器无法自动推断
*/

// ============= 测试代码 =============

fn main() {
    println!("=== first_word 示例 ===");
    let text = "hello world";
    let first = first_word(text);
    println!("First word: {}", first);

    let text2 = "programming";
    let first2 = first_word(text2);
    println!("First word: {}", first2);

    println!("\n=== parse_command 示例 ===");

    match parse_command("/help") {
        Ok(cmd) => println!("Command: {}", cmd),
        Err(e) => println!("Error: {}", e),
    }

    match parse_command("help") {
        Ok(cmd) => println!("Command: {}", cmd),
        Err(e) => println!("Error: {}", e),
    }

    println!("\n=== 需要显式标注的例子 ===");
    let s1 = "long string";
    let s2 = "short";
    let result = needs_annotation(s1, s2);
    println!("Longer: {}", result);

    println!("\n=== 对比有标注和无标注 ===");
    compare_annotated_unannotated();
}

/// 对比显式标注和省略的代码
fn compare_annotated_unannotated() {
    // 省略写法（推荐）
    fn first_word_unannotated(s: &str) -> &str {
        s.split(' ').next().unwrap_or("")
    }

    // 显式标注（等价，但不必要）
    fn first_word_annotated<'a>(s: &'a str) -> &'a str {
        s.split(' ').next().unwrap_or("")
    }

    let text = "hello world";
    println!("Unannotated: {}", first_word_unannotated(text));
    println!("Annotated: {}", first_word_annotated(text));

    // 两种写法完全等价
    assert_eq!(
        first_word_unannotated(text),
        first_word_annotated(text)
    );
}

// ============= 更多示例 =============

/// 方法中的生命周期省略
///
/// 展示规则 3 的应用
struct Context<'a> {
    data: &'a str,
}

impl<'a> Context<'a> {
    /// 这个方法不需要生命周期标注
    /// 规则 3: &self 的生命周期赋给返回值
    pub fn get_data(&self) -> &str {
        self.data
    }

    /// 这个方法也不需要标注
    /// 规则 3 同样适用
    pub fn get_first_word(&self) -> &str {
        self.data.split(' ').next().unwrap_or("")
    }
}

/// 字符串字面量总是 'static
///
/// 这是在省略规则中很重要的概念
fn get_static() -> &str {
    "hello"  // 字符串字面量有 'static 生命周期
}

fn demo_static() {
    let s: &'static str = "I live forever";
    println!("Static string: {}", s);
}
