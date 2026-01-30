//! # 练习 1: 基础生命周期
//!
//! 难度: 简单
//! 时间: 15 分钟
//! 前置知识: 借用检查器基础
//! 学习目标:
//!   - 理解生命周期参数的用途
//!   - 使用生命周期标注函数签名

// TODO: 添加生命周期参数使函数编译通过
// 提示: 返回值的生命周期与两个参数中较短的一个相同
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(&string1, &string2);
        println!("The longest string is {}", result);
    }
}
