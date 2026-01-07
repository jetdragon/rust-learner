//! # 练习 5: impl Trait - 解答
//!
//! 难度: 中等
//! 时间: 15 分钟
//! 前置知识: trait, 泛型
//! 学习目标:
//!   - 使用 impl Trait 作为参数和返回类型
//!   - 理解 impl Trait 的限制

use std::fmt::Display;

// ============= 解决方案 =============

/// 使用 impl Trait 简化参数类型
///
/// 对比泛型语法:
/// ```rust
/// // 传统泛型语法
/// pub fn notify<T: Display>(item: &T) {
///     println!("Breaking news! {}", item);
/// }
///
/// // impl Trait 语法（更简洁）
/// pub fn notify(item: &impl Display) {
///     println!("Breaking news! {}", item);
/// }
/// ```
///
/// impl Trait 参数的优势:
/// 1. 语法更简洁
/// 2. 意图更明确
/// 3. 对于简单场景更易读
///
/// 等价性:
/// 这两种语法在参数位置完全等价，编译器会生成相同的代码
pub fn notify(item: &impl Display) {
    println!("Breaking news! {}", item);
}

/// 使用 impl Trait 作为返回类型
///
/// 关键点:
/// - 返回一个实现了 Display 的具体类型
/// - 调用者不知道具体类型，只知道实现了 Display
/// - 必须返回单一具体类型
///
/// 为什么返回 String:
/// - String 实现了 Display
/// - 可以在 {} 中使用
/// - 拥有堆分配的数据，可以返回
///
/// 不能返回的情况:
/// ```rust
/// // 这个函数不能编译！
/// pub fn get_conditional(condition: bool) -> impl Display {
///     if condition {
///         String::from("yes")  // 返回 String
///     } else {
///         42  // 返回 i32 - 编译错误！
///     }
/// }
/// ```
///
/// 原因: impl Trait 在返回位置要求返回类型一致
pub fn create_displayable() -> impl Display {
    String::from("Hello, World!")
}

/// 返回不同类型的正确方式
///
/// 使用枚举包装不同类型
#[derive(Debug)]
enum Displayable {
    StringVal(String),
    IntVal(i32),
}

impl Display for Displayable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Displayable::StringVal(s) => write!(f, "{}", s),
            Displayable::IntVal(i) => write!(f, "{}", i),
        }
    }
}

/// 使用枚举返回不同类型
///
/// 这展示了如何绕过 impl Trait 的限制
pub fn get_conditional(condition: bool) -> impl Display {
    if condition {
        Displayable::StringVal(String::from("yes"))
    } else {
        Displayable::IntVal(42)
    }
}

// ============= 高级示例 =============

/// 返回迭代器的示例
///
/// 这是 impl Trait 最常见的用例之一
/// 迭代器类型通常很复杂，impl Trait 简化了签名
///
/// # 类型推断
///
/// 编译器知道具体的迭代器类型，但隐藏了这个细节
pub fn get_numbers() -> impl Iterator<Item = i32> {
    vec![1, 2, 3, 4, 5].into_iter()
}

/// 返回闭包的示例
///
/// 闭包的类型是匿名的，impl Trait 非常有用
pub fn create_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}

/// 链式迭代器示例
///
/// 展示了 impl Trait 如何简化复杂的迭代器类型
///
/// 不用 impl Trait 的话，返回类型会是:
/// std::iter::Map<std::iter::Filter<std::vec::IntoIter<i32>, [closure]>, [closure]>
pub fn process_numbers() -> impl Iterator<Item = i32> {
    vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        .into_iter()
        .filter(|x| x % 2 == 0)
        .map(|x| x * 2)
}

// ============= 对比示例 =============

/// 泛型版本 (等价于 impl Trait 参数)
///
/// 当你需要多次使用 T 时，必须用泛型语法
pub fn notify_generic<T: Display>(item1: &T, item2: &T) {
    println!("Items: {} and {}", item1, item2);
}

/// 多个参数使用 impl Trait
///
/// 注意: 这不要求两个参数是相同类型！
pub fn notify_different(item1: &impl Display, item2: &impl Display) {
    println!("Items: {} and {}", item1, item2);
}

// ============= 测试代码 =============

fn main() {
    println!("=== impl Trait 作为参数 ===");
    notify(&"Hello, World!");
    notify(&42);
    notify(&3.14159);

    println!("\n=== impl Trait 作为返回类型 ===");
    let displayable = create_displayable();
    println!("Got: {}", displayable);

    println!("\n=== 条件返回（使用枚举）===");
    let result = get_conditional(true);
    println!("Condition true: {}", result);

    let result = get_conditional(false);
    println!("Condition false: {}", result);

    println!("\n=== 返回迭代器 ===");
    for num in get_numbers().take(3) {
        println!("Number: {}", num);
    }

    println!("\n=== 返回闭包 ===");
    let double = create_multiplier(2);
    println!("2 * 5 = {}", double(5));
    println!("2 * 10 = {}", double(10));

    println!("\n=== 链式迭代器 ===");
    for num in process_numbers().take(3) {
        println!("Processed: {}", num);
    }

    println!("\n=== 泛型 vs impl Trait ===");
    notify_generic(&1, &2); // 必须是相同类型
    notify_different(&1, &"two"); // 可以是不同类型

    println!("\n=== 实际应用 ===");
    let text = "search text";
    let results = search(text, "text");
    for result in results {
        println!("Found: {}", result);
    }
}

/// 使用 impl Trait 返回搜索结果
///
/// 展示了实际应用中的 impl Trait
pub fn search<'a>(text: &'a str, query: &str) -> impl Iterator<Item = &'a str> {
    text.lines()
        .filter(move |line| line.contains(query))
}

// ============= 扩展知识 =============

/*
impl Trait 的完整规则:

参数位置:
1. impl Trait 完全等价于泛型参数
2. 可以有多个 impl Trait 参数
3. 不同 impl Trait 参数可以是不同类型

返回位置:
1. 必须返回单一具体类型
2. 不能在不同分支返回不同类型
3. 实际类型由函数体决定

何时使用 impl Trait:

✅ 适合使用:
- 返回复杂的迭代器类型
- 返回闭包类型
- 实现细节不重要时
- 简化函数签名

❌ 不适合使用:
- 需要返回不同类型时（用枚举）
- 需要在函数签名中引用具体类型时
- 需要多次使用类型参数时

对比表:
| 特性 | 泛型参数 | impl Trait |
|------|---------|-----------|
| 参数位置 | 完全等价 | 完全等价 |
| 返回位置 | 灵活 | 单一类型 |
| 类型可见性 | 可见 | 隐藏 |
| 复杂度 | 较高 | 较低 |

实际模式:
1. 迭代器: impl Iterator<Item = T>
2. 闭包: impl Fn(Arg) -> Ret
3. Future: impl Future<Output = T>
4. Stream: impl Stream<Item = T>
*/
