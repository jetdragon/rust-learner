//! # 练习 8: Supertrait - 解答
//!
//! 难度: 中等
//! 时间: 15 分钟
//! 前置知识: trait 继承
//! 学习目标:
//!   - 定义 supertrait (trait 继承)
//!   - 理解 trait 之间的依赖关系

use std::fmt::Display;

// ============= 解决方案 =============

/// OutlinePrint trait - 要求类型必须先实现 Display
///
/// Supertrait 语法:
/// trait OutlinePrint: Display { }
///
/// 这表示:
/// - 实现 OutlinePrint 的类型必须先实现 Display
/// - 在 OutlinePrint 的方法中可以使用 Display 的方法
/// - 类似于其他语言中的"接口继承"
///
/// 为什么需要这个:
/// - outline_print 需要 to_string() 方法
/// - to_string() 来自 Display trait
/// - 通过 supertrait 确保 Display 已实现
trait OutlinePrint: Display {
    /// 打印带边框的轮廓
    fn outline_print(&self);
}

/// Point 结构体 - 二维点
///
/// 实现了 Display，所以可以实现 OutlinePrint
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// 为 Point 实现 Display
///
/// 这是 OutlinePrint 的前置条件
impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// 为 Point 实现 OutlinePrint trait
///
/// 实现说明:
/// - 可以调用 self.to_string() 因为 Display 是 supertrait
/// - 打印格式化的边框
/// - 边框宽度根据内容自动调整
///
/// 输出示例:
/// <---------->
/// | (3, 4)  |
/// <---------->
impl OutlinePrint for Point {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "<{}>", "-".repeat(len + 2));
        println!("| {} |", output);
        println!("{}", "<{}>", "-".repeat(len + 2));
    }
}

// ============= 扩展示例 =============

/// 多个 supertrait
///
/// 可以要求实现多个 trait
trait Printable: Display + Clone {
    fn print_twice(&self) {
        println!("{}", self);
        println!("{}", self);
    }
}

/// 带 where 子句的 supertrait
///
/// 更复杂的约束可以用 where 表达
trait AdvancedTrait: Display
where
    Self: Clone,
{
    fn describe(&self) -> String {
        format!("Value: {}", self)
    }
}

/// Supertrait 链
///
/// trait 可以继承多个 trait，形成继承链
trait UltimateTrait: OutlinePrint + Clone {
    fn ultimate_print(&self) {
        println!("Ultimate:");
        self.outline_print();
    }
}

/// 实现复杂 supertrait 链的类型
#[derive(Clone)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Display for Point3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl OutlinePrint for Point3D {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "<{}>", "-".repeat(len + 2));
        println!("| {} |", output);
        println!("{}", "<{}>", "-".repeat(len + 2));
    }
}

impl UltimateTrait for Point3D {}

// ============= 对比示例 =============

/// 不使用 supertrait 的替代方案
///
/// 使用 trait bound 达到相同效果
trait OutlinePrintAlternative {
    fn outline_print_alt(&self)
    where
        Self: Display;
}

// 区别:
// 1. Supertrait: 要求在实现 trait 时必须已实现 Display
// 2. Trait bound: 只在调用该方法时需要 Display
// 3. Supertrait 更严格但更清晰

// ============= 测试代码 =============

fn main() {
    println!("=== 基础 Supertrait 示例 ===");
    let p = Point { x: 3, y: 4 };
    p.outline_print();

    println!("\n=== 不同的点 ===");
    let p2 = Point { x: -1, y: 2 };
    p2.outline_print();

    println!("\n=== 多个 Supertrait ===");
    let p3 = Point { x: 10, y: 20 };
    // p3 实现 Display，所以可以使用 Printable
    // 注意: Printable 需要 Display + Clone
    // Point 没有实现 Clone，所以这个不会编译
    // p3.print_twice();

    println!("\n=== Supertrait 链 ===");
    let p3d = Point3D { x: 1, y: 2, z: 3 };
    p3d.ultimate_print();

    println!("\n=== 实际应用 ===");
    practical_example();
}

/// 实际应用示例
fn practical_example() {
    // 创建可配置的日志器
    trait Loggable: Display {
        fn log(&self) {
            println!("[LOG] {}", self);
        }
    }

    struct Message {
        content: String,
    }

    impl Display for Message {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.content)
        }
    }

    impl Loggable for Message {}

    let msg = Message {
        content: String::from("System started"),
    };
    msg.log();
}

// ============= 扩展知识 =============

/*
Supertrait 的完整语法:

1. 单个 supertrait:
   trait MyTrait: Display { }

2. 多个 supertrait:
   trait MyTrait: Display + Debug + Clone { }

3. 带 where 子句:
   trait MyTrait: Display
   where
       Self: Sized,
   { }

4. Supertrait 链:
   trait C: B { }
   trait B: A { }
   trait A { }

使用场景:

✅ 适合使用 supertrait:
1. 需要依赖其他 trait 的功能
2. 建立清晰的 trait 层次
3. 确保 trait 实现者有特定能力

❌ 不适合使用:
1. 简单的 trait 组合（使用组合）
2. 可选的功能（使用独立 trait）

Supertrait vs 组合:

Supertrait:
trait MyTrait: Display + Debug { }

组合:
fn my_function<T>(item: T)
where
    T: Display + Debug,
{ }

区别:
- Supertrait 是"继承"关系
- 组合是"约束"关系
- Supertrait 在 trait 定义时
- 组合在使用时

常见 Supertrait 示例:

1. Error trait:
   trait Error: Debug + Display { }

2. Iterator 扩展:
   trait MyIterator: Iterator { }

3. 字符串转换:
   trait ToString: Display { }

设计模式:

1. 模板方法:
   trait Processable: Display {
       fn process(&self) {
           println!("Processing: {}", self);
           self.do_process();
       }
       fn do_process(&self);
   }

2. 能力检查:
   trait Serializable: Display {
       fn serialize(&self) -> String {
           self.to_string()
       }
   }

3. 特化:
   trait AdvancedTrait: BasicTrait {
       fn advanced_method(&self);
   }
*/
