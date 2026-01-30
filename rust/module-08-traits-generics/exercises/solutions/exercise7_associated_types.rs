//! # 练习 7: 关联类型 - 解答
//!
//! 难度: 中等
//! 时间: 15 分钟
//! 前置知识: trait, 泛型
//! 学习目标:
//!   - 在 trait 中定义关联类型
//!   - 理解关联类型 vs 泛型参数的区别

/// Iterator trait - 标准库中关联类型的经典例子
///
/// 为什么使用关联类型而不是泛型:
/// - 每个 Iterator 实现只有一个特定的 Item 类型
/// - 不需要在每次使用时指定类型
/// - 使代码更简洁
pub trait Iterator {
    /// 关联类型 - 迭代器产生值的类型
    ///
    /// 这与泛型参数的区别:
    /// - trait Iterator<Item=i32> vs trait Iterator
    /// - 每个实现只选择一次 Item 类型
    /// - 编译器可以自动推断
    type Item;

    /// 返回下一个值
    ///
    /// 返回 Self::Item 而不是泛型参数
    fn next(&mut self) -> Option<Self::Item>;
}

// ============= 解决方案 =============

/// Counter 结构体 - 计数器
///
/// 这是一个简单的迭代器示例
pub struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { count: 0, max: 5 }
    }

    pub fn with_max(max: u32) -> Counter {
        Counter { count: 0, max }
    }
}

/// 为 Counter 实现 Iterator trait
///
/// 实现说明:
/// - type Item = u32; 指定关联类型
/// - next() 返回 Option<u32>
/// - 关联类型让迭代器的使用更简洁
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            let result = Some(self.count);
            self.count += 1;
            result
        } else {
            None
        }
    }
}

/// CharCounter 结构体 - 字符计数器
///
/// 这个迭代器产生 char 类型，展示了不同的关联类型
pub struct CharCounter {
    chars: Vec<char>,
    index: usize,
}

impl CharCounter {
    pub fn new(s: &str) -> CharCounter {
        CharCounter {
            chars: s.chars().collect(),
            index: 0,
        }
    }
}

/// 为 CharCounter 实现 Iterator trait
///
/// 实现说明:
/// - type Item = char; 不同的关联类型
/// - 同一个 trait，不同的类型参数
/// - 这是关联类型的优势
impl Iterator for CharCounter {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.chars.len() {
            let result = Some(self.chars[self.index]);
            self.index += 1;
            result
        } else {
            None
        }
    }
}

// ============= 对比示例 =============

/// 如果使用泛型而不是关联类型:
///
/// 这展示了为什么关联类型更简洁
trait IteratorGeneric<Item> {
    fn next(&mut self) -> Option<Item>;
}

/// 使用泛型版本:
struct CounterGeneric {
    count: u32,
}

impl IteratorGeneric<u32> for CounterGeneric {
    fn next(&mut self) -> Option<u32> {
        // ...
        None
    }
}

// 问题:
// 1. 每次使用都需要指定类型
// 2. 可以为同一类型实现多次（可能冲突）
// 3. 代码更冗长

// ============= 高级示例 =============

/// 使用多个关联类型的 trait
///
/// 展示了关联类型的灵活性
trait Container {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;

    fn iter(&self) -> Self::IntoIter;
}

/// 实现多个关联类型
struct MyVec<T> {
    items: Vec<T>,
}

impl<T> Container for MyVec<T>
where
    T: Clone,
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn iter(&self) -> Self::IntoIter {
        self.items.clone().into_iter()
    }
}

// ============= 测试代码 =============

fn main() {
    println!("=== Counter (u32) ===");
    let mut counter = Counter::new();
    if let Some(v) = counter.next() {
        println!("Counter value: {}", v);
    }
    if let Some(v) = counter.next() {
        println!("Counter value: {}", v);
    }

    println!("\n=== 使用迭代器适配器 ===");
    let counter2 = Counter::with_max(10);
    let sum: u32 = counter2.filter(|x| x % 2 == 0).sum();
    println!("Sum of evens: {}", sum);

    println!("\n=== CharCounter (char) ===");
    let mut char_counter = CharCounter::new("hello");
    if let Some(c) = char_counter.next() {
        println!("Char: {}", c);
    }
    if let Some(c) = char_counter.next() {
        println!("Char: {}", c);
    }

    println!("\n=== 收集所有字符 ===");
    let char_counter2 = CharCounter::new("world");
    let chars: Vec<char> = char_counter2.collect();
    println!("Characters: {:?}", chars);

    println!("\n=== 关联类型 vs 泛型 ===");
    demonstrate_associated_vs_generic();

    println!("\n=== 实际应用 ===");
    practical_example();
}

/// 对比关联类型和泛型
fn demonstrate_associated_vs_generic() {
    // 关联类型 - 不需要指定类型
    let counter = Counter::new();
    let doubled: Vec<_> = counter.map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);

    // 如果是泛型版本，需要写:
    // let doubled: Vec<_> = counter.map::<u32, _>(|x| x * 2).collect();
    // 但编译器能自动推断，所以实际上也差不多
}

/// 实际应用示例
fn practical_example() {
    // 使用标准库迭代器
    let numbers = vec![1, 2, 3, 4, 5];

    // 类型推断自动工作
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);

    // 过滤偶数
    let evens: Vec<_> = numbers.into_iter().filter(|x| x % 2 == 0).collect();
    println!("Evens: {:?}", evens);
}

// ============= 扩展知识 =============

/*
关联类型 vs 泛型参数:

关联类型 (type Item):
- 每个 impl 选择一次类型
- 类型由实现决定
- 更简洁的语法
- 适用于"每个实现一个类型"的场景

泛型参数:
- 每次使用时指定类型
- 类型由调用者决定
- 更灵活但更冗长
- 适用于"多种类型"的场景

选择指南:

✅ 使用关联类型:
1. 实现决定类型（如 Iterator）
2. 类型与实现紧密相关
3. 只需要一个类型实现

✅ 使用泛型参数:
1. 调用者决定类型（如 From）
2. 需要多种类型实现
3. 灵活性更重要

常见使用关联类型的 trait:

1. Iterator:
   type Item;

2. Iterator 的扩展:
   type IntoIter;

3. Future (async):
   type Output;

4. Stream (async):
   type Item;

5. 范型容器:
   type Item;
   type IntoIter;

组合使用:
trait MyTrait {
    type Item;  // 关联类型

    fn process<T>(&self, item: T) -> Self::Item;  // 泛型方法
}
*/
