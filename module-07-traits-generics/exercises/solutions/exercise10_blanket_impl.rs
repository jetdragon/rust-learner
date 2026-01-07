//! # 练习 10: Blanket 实现 - 解答
//!
//! 难度: 中等
//! 时间: 15 分钟
//! 前置知识: trait, 泛型
//! 学习目标:
//!   - 理解 blanket 实现的概念
//!   - 使用条件为满足 trait bound 的类型实现 trait

use std::fmt::Display;

// ============= 解决方案 =============

/// Printable trait - 展示 blanket 实现
///
/// Blanket 实现:
/// - 为满足条件的所有类型实现 trait
/// - 使用 impl<T: Trait> Printable for T 语法
/// - 非常强大但需要谨慎使用
///
/// 这个 trait 有默认实现，可以被覆盖
pub trait Printable {
    /// 打印值
    fn print(&self) {
        println!("{}", self);
    }

    /// 打印两次
    fn print_twice(&self) {
        println!("{}", self);
        println!("{}", self);
    }

    /// 带前缀打印
    fn print_with_prefix(&self, prefix: &str) {
        println!("{}: {}", prefix, self);
    }
}

/// Blanket 实现
///
/// 为所有实现 Display 的类型实现 Printable
///
/// 语法解释:
/// - impl<T: Display> - 对于所有满足 Display 的类型 T
/// - Printable for T - 为 T 实现 Printable
/// - 结果: 任何实现了 Display 的类型自动获得 Printable
///
/// 为什么这是"blanket":
/// - 像"毯子"一样覆盖所有满足条件的类型
/// - 一个实现覆盖无限多个类型
impl<T: Display> Printable for T {}

// ============= 高级示例 =============

/// 使用 where 子句的 blanket 实现
///
/// 更复杂的条件可以用 where 表达
trait SuperPrintable
where
    Self: Display + Clone,
{
    fn super_print(&self) {
        println!("Super: {}", self);
    }

    fn print_cloned(&self) {
        let cloned = self.clone();
        println!("Cloned: {}", cloned);
    }
}

/// 为所有满足条件的类型实现
impl<T> SuperPrintable for T
where
    T: Display + Clone,
{
}

/// 为标准库类型添加 trait 的例子
///
/// 注意: 这是可以的，因为我们在实现我们自己的 trait
/// 如果要为外部类型实现外部 trait，需要 newtype 模式
trait DebugPrint {
    fn debug_print(&self);
}

/// 为所有 Display 类型实现
impl<T: Display> DebugPrint for T {
    fn debug_print(&self) {
        println!("[DEBUG] {}", self);
    }
}

// ============= 实际应用示例 =============

/// 为引用类型实现 trait
///
/// 展示了更复杂的 blanket 实现
trait RefPrintable {
    fn ref_print(&self);
}

/// 为所有 &T 实现，其中 T: Display
impl<'a, T: Display> RefPrintable for &'a T {
    fn ref_print(&self) {
        println!("Ref: {}", self);
    }
}

/// 为元组实现 trait
///
/// 为特定元组类型实现 trait
trait PairPrintable {
    fn print_pair(&self);
}

/// 为包含两个 Display 类型的元组实现
impl<T1: Display, T2: Display> PairPrintable for (T1, T2) {
    fn print_pair(&self) {
        println!("Pair: {} and {}", self.0, self.1);
    }
}

/// 为 Result 类型实现
///
/// 非常实用的 blanket 实现
trait ResultPrintable {
    fn print_result(&self);
}

impl<T: Display, E: Display> ResultPrintable for Result<T, E> {
    fn print_result(&self) {
        match self {
            Ok(v) => println!("Success: {}", v),
            Err(e) => println!("Error: {}", e),
        }
    }
}

// ============= 测试代码 =============

fn main() {
    println!("=== 基础 Blanket 实现 ===");

    // 字符串实现了 Display，所以自动有 Printable
    let message = "Hello, World!";
    message.print();
    println!();

    message.print_twice();
    println!();

    message.print_with_prefix("INFO");
    println!();

    // 数字也自动有 Printable
    let number = 42;
    number.print();
    println!();

    let pi = 3.14159;
    pi.print_with_prefix("PI");
    println!();

    println!("=== SuperPrintable 示例 ===");
    let vec = vec![1, 2, 3];
    vec.super_print();
    vec.print_cloned();
    println!();

    println!("=== DebugPrint 示例 ===");
    "test".debug_print();
    123.debug_print();
    println!();

    println!("=== 引用类型 ===");
    let value = 42;
    let reference = &value;
    reference.ref_print();
    println!();

    println!("=== 元组类型 ===");
    let pair = ("Alice", 30);
    pair.print_pair();

    let pair2 = (3.14, "Pi");
    pair2.print_pair();
    println!();

    println!("=== Result 类型 ===");
    let ok_result: Result<i32, &str> = Ok(100);
    ok_result.print_result();

    let err_result: Result<i32, &str> = Err("Something went wrong");
    err_result.print_result();
    println!();

    println!("=== 自定义类型 ===");
    demonstrate_custom_type();
}

/// 展示自定义类型自动获得 trait
fn demonstrate_custom_type() {
    #[derive(Debug, Clone)]
    struct Person {
        name: String,
        age: u32,
    }

    // 实现 Display 使 Person 自动获得其他 trait
    impl Display for Person {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} ({} years old)", self.name, self.age)
        }
    }

    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // Person 自动有 Printable, DebugPrint, SuperPrintable
    person.print();
    person.super_print();
    person.print_cloned();
    person.debug_print();
}

// ============= 标准库中的 Blanket 实现 =============

/*
标准库中著名的 blanket 实现:

1. ToString:
   impl<T: Display> ToString for T

   任何实现了 Display 的类型自动有 to_string()

2. From:
   impl<T> From<T> for T
   impl<T> From<&T> for &T where T: ?Sized

3. Iterator 适配器:
   impl<I: Iterator> Iterator for Rev<I>
   impl<I: Iterator> Iterator for Take<I>

这些使得 Rust 的 trait 系统非常强大和灵活
*/

mod standard_library_examples {
    use std::fmt::Display;

    // 模拟 ToString 的实现
    trait MyToString {
        fn my_to_string(&self) -> String;
    }

    impl<T: Display> MyToString for T {
        fn my_to_string(&self) -> String {
            format!("{}", self)
        }
    }

    // 使用示例
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_blanket_to_string() {
            assert_eq!(42.my_to_string(), "42");
            assert_eq!("hello".my_to_string(), "hello");
        }
    }
}

// ============= 扩展知识 =============

/*
Blanket 实现规则:

✅ 可以做:
1. 为你的 trait 实现你的 trait
2. 为你的 trait 实现外部 trait
3. 为所有满足条件的类型实现你的 trait

❌ 不能做:
1. 为外部 trait 实现你的 trait（孤儿规则）
2. 为特定外部类型实现外部 trait（孤儿规则）

语法总结:

1. 简单条件:
   impl<T: Display> MyTrait for T {}

2. 多个条件:
   impl<T: Display + Clone> MyTrait for T {}

3. 使用 where:
   impl<T> MyTrait for T
   where
       T: Display + Clone,
   {}

4. 带生命周期:
   impl<'a, T: Display> MyTrait for &'a T {}

5. 复杂组合:
   impl<T, U> MyTrait for (T, U)
   where
       T: Display,
       U: Display,
   {}

设计考虑:

优点:
1. 一次实现，无限使用
2. 非常强大的抽象
3. 减少重复代码

缺点:
1. 可能导致意外的实现
2. 难以预测哪些类型有 trait
3. 可能产生命名冲突

最佳实践:
1. 谨慎使用 blanket 实现
2. 确保语义正确
3. 提供清晰的文档
4. 考虑使用 newtype 模式替代

实际模式:

1. 扩展 trait:
   trait MyTrait: Display { }

   等价于:
   impl<T: Display> MyTrait for T {}

2. 默认实现:
   trait MyTrait {
       fn method(&self) { ... }
   }

   impl<T: Display> MyTrait for T {}

3. 条件实现:
   impl<T> MyTrait for Vec<T>
   where
       T: Display,
   {}
*/
