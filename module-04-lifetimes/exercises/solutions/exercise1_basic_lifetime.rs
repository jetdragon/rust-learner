//! # 练习 1: 基础生命周期 - 解答
//!
//! 难度: 简单
//! 时间: 15 分钟
//! 前置知识: 借用检查器基础
//! 学习目标:
//!   - 理解生命周期参数的用途
//!   - 使用生命周期标注函数签名

// ============= 解决方案 =============

/// longest 函数 - 返回两个字符串切片中较长的一个
///
/// 生命周期参数说明:
/// - 'a 是生命周期参数名（任意名称，通常用短名称）
/// - x: &'a str 表示 x 的生命周期是 'a
/// - y: &'a str 表示 y 的生命周期也是 'a
/// - -> &'a str 表示返回值的生命周期是 'a
///
/// 含义:
/// - 返回值的生命周期与参数中较短的一个相同
/// - 确保返回的引用在函数返回后仍然有效
///
/// # 为什么需要生命周期
///
/// 编译器不知道返回的是 x 还是 y
/// - 如果返回 x，生命周期与 x 相同
/// - 如果返回 y，生命周期与 y 相同
/// - 必须确保返回值在调用者使用时仍然有效
///
/// # 示例
///
/// ```
/// let s1 = String::from("long string");
/// {
///     let s2 = String::from("short");
///     let result = longest(&s1, &s2);
///     // result 在这里有效，因为生命周期与较短的一样
/// }
/// ```
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/// 不需要生命周期标注的例子
///
/// 返回非引用类型不需要生命周期
pub fn first_word_length(s: &str) -> usize {
    s.find(' ').unwrap_or(s.len())
}

// ============= 扩展示例 =============

/// 三个参数的生命周期
///
/// 展示了更复杂的情况
pub fn longest_three<'a>(x: &'a str, y: &'a str, z: &'a str) -> &'a str {
    let mut longest = x;
    if y.len() > longest.len() {
        longest = y;
    }
    if z.len() > longest.len() {
        longest = z;
    }
    longest
}

/// 不同的生命周期
///
/// 有时参数有不同的生命周期
pub fn announce_and_return<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    println!("Comparing: {} and {}", x, y);
    x  // 返回 x，所以返回类型是 'a
}

// ============= 测试代码 =============

fn main() {
    println!("=== 基础生命周期示例 ===");

    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");

    let result = longest(&string1, &string2);
    println!("The longest string is {}", result);

    println!("\n=== 不同作用域 ===");

    {
        let string1 = String::from("long string is long");

        {
            let string2 = String::from("xyz");
            let result = longest(&string1, &string2);
            println!("The longest string is {}", result);
            // result 在这里有效，因为 string2 仍然有效
        }
        // string2 在这里失效，但 result 也已经不再使用
    }

    println!("\n=== 三个参数 ===");
    let s1 = "short";
    let s2 = "medium length";
    let s3 = "very very long string";
    let result = longest_three(s1, s2, s3);
    println!("Longest of three: {}", result);

    println!("\n=== 不同生命周期 ===");
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let result = announce_and_return(&s1, &s2);
    println!("Returned: {}", result);

    println!("\n=== 字符串字面量 ===");
    // 字符串字面量有 'static 生命周期
    let s1 = "long static string";
    let s2 = "short";
    let result = longest(s1, s2);
    println!("Longest with literal: {}", result);
}

// ============= 生命周期语法详解 =============

/*
生命周期标注语法:

1. 单个生命周期:
   fn fn_name<'a>(param: &'a Type) -> &'a Type

2. 多个生命周期:
   fn fn_name<'a, 'b>(x: &'a str, y: &'b str) -> &'a str

3. 在结构体中:
   struct Name<'a> {
       field: &'a str,
   }

4. 在方法中:
   impl<'a> Type<'a> {
       fn method(&self) -> &'a str { ... }
   }

命名约定:
- 'a, 'b, 'c - 通用生命周期
- 'static - 特殊的静态生命周期
- 'input, 'output - 描述性名称

生命周期省略规则（详见后面练习）:

规则 1: 每个引用参数都有自己的生命周期
规则 2: 如果只有一个输入生命周期，赋给所有输出生命周期
规则 3: 如果有多个输入生命周期但其中一个是 &self 或 &mut self，
       赋给所有输出生命周期

为什么需要生命周期:

1. 编译器无法推断返回值来自哪个参数
2. 需要确保引用在使用时仍然有效
3. 防止悬垂引用（dangling references）

常见错误:

1. 悬垂引用:
   fn bad() -> &'a str {
       let s = String::from("hello");
       &s  // 错误: s 会失效
   }

2. 不匹配的生命周期:
   fn bad<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
       if x.len() > y.len() {
           y  // 错误: 返回 'b 但应该是 'a
       } else {
           x
       }
   }
*/
