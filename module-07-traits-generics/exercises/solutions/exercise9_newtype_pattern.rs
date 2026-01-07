//! # 练习 9: Newtype 模式 - 解答
//!
//! 难度: 中等
//! 时间: 15 分钟
//! 前置知识: tuple struct, trait
//! 学习目标:
//!   - 使用 newtype 模式实现外部 trait
//!   - 理解孤儿规则的限制

use std::fmt;

// ============= 孤儿规则说明 =============

/*
孤儿规则 (Orphan Rule):

你可能只为以下类型实现 trait:
1. 定义在当前 crate 中的 trait
2. 定义在当前 crate 中的类型

换句话说:
- 不能为外部类型实现外部 trait
- 例如: 不能为 Vec<T> 实现 Display
- 防止冲突: 两个 crate 可能为同一类型实现同一 trait

解决方法: Newtype 模式
- 创建一个新的 tuple struct 包装外部类型
- 为新类型实现所需的 trait
*/

/// Wrapper 结构体 - newtype 模式的例子
///
/// Newtype 模式:
/// - 使用 tuple struct 包装一个类型
/// - 可以实现任何 trait（因为是本地类型）
/// - 通过 Deref trait 可以获得原始类型的方法
///
/// 这里包装 Vec<String> 以实现 Display
pub struct Wrapper(Vec<String>);

// ============= 解决方案 =============

/// 为 Wrapper 实现 Display trait
///
/// 实现说明:
/// - 格式: [item1, item2, item3]
/// - 访问内部数据: self.0
/// - 使用 join() 连接字符串
///
/// 为什么能工作:
/// - Wrapper 是本地定义的类型
/// - Display 是标准库的 trait
/// - 孤儿规则允许这样做
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

/// 为 Wrapper 实现 Debug trait
///
/// 同样使用 newtype 模式
/// 提供不同的调试格式
impl fmt::Debug for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Wrapper({:?})", self.0)
    }
}

// ============= 扩展示例 =============

/// 为 Wrapper 实现 Clone trait
///
/// 展示了 newtype 模式可以添加任何行为
impl Clone for Wrapper {
    fn clone(&self) -> Self {
        Wrapper(self.0.clone())
    }
}

/// 实现 PartialEq 以支持比较
impl PartialEq for Wrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

/// 实现 Eq
impl Eq for Wrapper {}

/// 更复杂的 newtype 模式示例
///
/// 为外部类型添加额外功能
pub struct Celsius(i32);

impl Celsius {
    pub fn new(temp: i32) -> Self {
        Celsius(temp)
    }

    pub fn to_fahrenheit(&self) -> i32 {
        self.0 * 9 / 5 + 32
    }
}

impl fmt::Display for Celsius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}°C", self.0)
    }
}

/// 使用 Deref trait 提供透明访问
///
/// 这使 Wrapper 可以像 Vec<String> 一样使用
use std::ops::{Deref, DerefMut};

impl Deref for Wrapper {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Wrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// 类型别名 vs Newtype 模式
///
/// 类型别名只是别名，不是新类型
type StringVec = Vec<String>;

// newtype 创建了真正的类型
struct StringVecWrapper(Vec<String>);

/// 使用泛型的 newtype 模式
///
/// 创建可重用的包装器
pub struct SmartPointer<T> {
    value: T,
}

impl<T> SmartPointer<T> {
    pub fn new(value: T) -> Self {
        SmartPointer { value }
    }
}

impl<T: fmt::Display> fmt::Display for SmartPointer<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SmartPointer({})", self.value)
    }
}

// ============= 测试代码 =============

fn main() {
    println!("=== 基础 Newtype 模式 ===");
    let w = Wrapper(vec![
        String::from("hello"),
        String::from("world"),
        String::from("rust"),
    ]);
    println!("Display: {}", w);
    println!("Debug: {:?}", w);

    println!("\n=== 使用 Deref 访问内部方法 ===");
    let mut w2 = Wrapper(vec![
        String::from("one"),
        String::from("two"),
    ]);
    println!("Length: {}", w2.len());
    w2.push(String::from("three"));
    println!("After push: {}", w2);

    println!("\n=== Celsius Newtype ===");
    let temp = Celsius::new(25);
    println!("Temperature: {}", temp);
    println!("In Fahrenheit: {}°F", temp.to_fahrenheit());

    println!("\n=== 泛型 Newtype ===");
    let sp = SmartPointer::new(42);
    println!("Smart pointer: {}", sp);

    let sp2 = SmartPointer::new("Hello");
    println!("Smart pointer: {}", sp2);

    println!("\n=== 实际应用示例 ===");
    practical_examples();
}

/// 实际应用示例
fn practical_examples() {
    // 1. 单位类型 - 防止混淆单位
    struct Meters(f64);
    struct Feet(f64);

    impl Meters {
        pub fn to_feet(&self) -> Feet {
            Feet(self.0 * 3.28084)
        }
    }

    let distance = Meters(100.0);
    let in_feet = distance.to_feet();
    println!("100 meters is {} feet", in_feet.0);

    // 2. 验证包装器 - 确保数据有效性
    pub struct NonEmptyString(String);

    impl NonEmptyString {
        pub fn new(s: String) -> Option<Self> {
            if s.is_empty() {
                None
            } else {
                Some(NonEmptyString(s))
            }
        }

        pub fn get(&self) -> &str {
            &self.0
        }
    }

    let valid = NonEmptyString::new(String::from("Hello"));
    let invalid = NonEmptyString::new(String::from(""));

    println!("Valid string: {:?}", valid.map(|s| s.get()));
    println!("Invalid string: {:?}", invalid);

    // 3. 状态包装器 - 类型级别的状态机
    pub struct VerifiedEmail(String);

    impl VerifiedEmail {
        pub fn send_email(&self, message: &str) {
            println!("Sending '{}' to {}", message, self.0);
        }
    }

    // 必须经过验证才能发送
    // 这在编译时强制执行
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let w = Wrapper(vec![String::from("a"), String::from("b")]);
        assert_eq!(format!("{}", w), "[a, b]");
    }

    #[test]
    fn test_equality() {
        let w1 = Wrapper(vec![String::from("test")]);
        let w2 = Wrapper(vec![String::from("test")]);
        assert_eq!(w1, w2);
    }

    #[test]
    fn test_celsius_conversion() {
        let c = Celsius::new(0);
        assert_eq!(c.to_fahrenheit(), 32);

        let c = Celsius::new(100);
        assert_eq!(c.to_fahrenheit(), 212);
    }
}

// ============= 扩展知识 =============

/*
Newtype 模式的优势:

1. 绕过孤儿规则
   - 为外部类型实现外部 trait

2. 类型安全
   - 防止混淆不同单位的值
   - 例如: Meters vs Feet

3. 封装
   - 隐藏实现细节
   - 提供验证逻辑

4. 文档
   - 类型名称传达意图
   - 例如: NonEmptyString vs String

5. 优化
   - 零成本抽象
   - 编译器会优化掉包装层

Newtype 模式的常见用法:

1. 实现 trait:
   struct MyVec(Vec<i32>);

2. 添加语义:
   struct UserId(i32);
   struct TransactionId(i32);

3. 类型安全:
   struct Celsius(i32);
   struct Fahrenheit(i32);

4. 验证:
   struct NonEmptyVec(Vec<T>);
   struct SortedVec(Vec<T>);

5. 状态:
   struct Validated<T>(T);
   struct Unvalidated<T>(T);

与 Deref 结合:
- 自动获得内部类型的方法
- 但要注意: 类型转换不会自动发生
- 显式比隐式更好

最佳实践:
1. 使用 newtype 时重新实现关键 trait
2. 考虑是否需要 Deref/DerefMut
3. 为新类型提供有用的方法
4. 使用有意义的类型名
*/
