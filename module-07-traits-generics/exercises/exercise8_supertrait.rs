//! # 练习 8: Supertrait
//!
//! 难度: 中等
//! 时间: 15 分钟
//! 前置知识: trait 继承
//! 学习目标:
//!   - 定义 supertrait (trait 继承)
//!   - 理解 trait 之间的依赖关系

use std::fmt::Display;

// TODO: 定义 OutlinePrint trait，要求类型必须先实现 Display
trait OutlinePrint: Display {
    fn outline_print(&self);
}

pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// TODO: 为 Point 实现 OutlinePrint trait
// outline_print 方法应该打印:
// <---------->
// | (x, y)  |
// <---------->
// 其中 (x, y) 是 Display 的输出
impl OutlinePrint for Point {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "<{}>", "-".repeat(len + 2));
        println!("| {} |", output);
        println!("{}", "<{}>", "-".repeat(len + 2));
    }
}

fn main() {
    let p = Point { x: 3, y: 4 };
    p.outline_print();
}
