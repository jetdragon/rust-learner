//! # 练习 3: Trait Bound - 解答
//!
//! 难度: 中等
//! 时间: 15 分钟
//! 前置知识: 泛型基础
//! 学习目标:
//!   - 使用 trait bound 约束泛型
//!   - 理解 where 子句

use std::fmt::Display;

// ============= 解决方案 =============

/// Trait Bound 示例 1: 简单约束
///
/// 使用 <T: Display> 语法
/// - T 必须实现 Display trait
/// - 这样我们可以在函数内使用 {} 打印 T
///
/// # 替代语法
/// ```rust
/// // 使用 impl Trait 语法（更简洁）
/// pub fn notify(item: &impl Display) {
///     println!("Breaking news! {}", item);
/// }
/// ```
pub fn notify<T: Display>(item: &T) {
    println!("Breaking news! {}", item);
}

/// Trait Bound 示例 2: 多个约束
///
/// 使用 where 子句使函数签名更清晰
/// - T 必须实现 PartialOrd (可以比较)
/// - T 必须实现 Display (可以打印)
///
/// # 对比语法
/// ```rust
/// // 传统的 trait bound 语法
/// pub fn compare_and_print<T>(a: &T, b: &T) -> String
/// where
///     T: PartialOrd + Display,
/// {
///     // ...
/// }
///
/// // 等价于:
/// pub fn compare_and_print<T: PartialOrd + Display>(a: &T, b: &T) -> String {
///     // ...
/// }
/// ```
///
/// where 子句的优势:
/// 1. 使函数签名更清晰
/// 2. 复杂约束时更易读
/// 3. 避免行过长
pub fn compare_and_print<T>(a: &T, b: &T) -> String
where
    T: PartialOrd + Display,
{
    if a > b {
        format!("{} is greater than {}", a, b)
    } else {
        format!("{} is not greater than {}", a, b)
    }
}

/// 更复杂的 trait bound 示例
///
/// 展示了多种 trait bound 语法
pub fn process_items<T, U>(item1: &T, item2: &U) -> String
where
    T: Display + Clone,
    U: Display + PartialEq,
{
    format!("Item1: {}, Item2: {}", item1, item2)
}

// ============= 测试代码 =============

fn main() {
    println!("=== 简单 Trait Bound ===");
    notify(&"Hello");
    notify(&42);
    notify(&3.14);

    println!("\n=== 多个 Trait Bound ===");
    println!("{}", compare_and_print(&5, &3));
    println!("{}", compare_and_print(&"hello", &"world"));
    println!("{}", compare_and_print(&10, &20));

    println!("\n=== 复杂 Trait Bound ===");
    println!("{}", process_items(&"test", &"test"));

    // 实际应用示例
    println!("\n=== 实际应用 ===");
    let numbers = vec![1, 2, 3, 4, 5];
    println!("Largest: {}", largest(&numbers));

    let words = vec!["apple", "banana", "cherry"];
    println!("Longest: {}", largest(&words));
}

/// 返回集合中最大的元素
///
/// 这个函数展示了 trait bound 的实际应用
/// - T 必须可以比较 (PartialOrd)
/// - T 必须可以复制 (Copy)，因为我们需要返回它
///
/// # 类型参数
///
/// * `T` - 必须实现 PartialOrd 和 Copy
///
/// # 示例
///
/// ```
/// let nums = vec![1, 2, 3];
/// assert_eq!(largest(&nums), &3);
/// ```
pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// ============= 高级示例 =============

/// 使用生命周期和 trait bound
///
/// 展示了如何结合使用生命周期和 trait bound
pub fn longest_with_announcement<'a, T>(
    x: &'a T,
    y: &'a T,
    ann: T,
) -> &'a T
where
    T: Display + PartialOrd,
{
    println!("Announcement! {}", ann);
    if x > y {
        x
    } else {
        y
    }
}

/// 返回 impl Trait 的示例
///
/// 这是 Rust 1.26+ 引入的特性
/// 允许返回实现了特定 trait 的类型
///
/// 注意: 返回的必须是具体类型，不能在不同分支返回不同类型
pub fn create_displayable(value: bool) -> impl Display {
    if value {
        String::from("yes")
    } else {
        String::from("no")
    }
    // 这个函数不能写成:
    // if value { String::from("yes") } else { 42 }
    // 因为返回类型必须统一
}

// ============= 扩展知识 =============

/*
Trait Bound 语法总结:

1. 简单语法:
   T: Trait

2. 多个 trait:
   T: Trait1 + Trait2

3. where 子句 (推荐用于复杂情况):
   fn f<T>(t: T) where T: Trait

4. impl Trait 语法 (简洁但有限制):
   fn f(t: impl Trait)

5. 返回 impl Trait:
   fn f() -> impl Trait

常见的标准库 Trait Bound:
- Display: 可以用 {} 格式化
- Debug: 可以用 {:?} 格式化
- Clone: 可以显式克隆
- Copy: 可以隐式复制
- PartialOrd: 可以部分比较
- Ord: 完全排序
- PartialEq: 可以部分相等比较
- Eq: 完全相等比较
- Hash: 可以计算哈希值
- Iterator: 迭代器
- Send: 可以在线程间传递所有权
- Sync: 可以在线程间共享引用

选择何时使用哪种语法:
1. 参数类型: impl Trait 更简洁
2. 复杂约束: where 子句更清晰
3. 返回类型: impl Trait（但有限制）
4. 需要指定具体类型: 使用泛型参数
*/
