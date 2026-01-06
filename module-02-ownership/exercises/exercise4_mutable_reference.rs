//! # 练习 4: 可变引用
//!
//! **难度**: 中等
//! **预计时间**: 15 分钟
//!
//! **前置知识**:
//! - 基本的引用概念 (`&T`)
//! - 可变变量声明 (`mut`)
//!
//! **学习目标**:
//! - 理解可变引用的语法 (`&mut T`)
//! - 学会如何修改引用指向的数据
//! - 掌握可变引用的借用规则

/// 向字符串追加 ", world"
///
/// # TODO
///
/// 1. 修改函数签名，使用可变引用 `&mut String`
/// 2. 实现函数体，向字符串追加内容
pub fn append_world(s: &mut String) {
    unimplemented!()
}

fn main() {
    let mut s = String::from("hello");

    // TODO: 传递可变引用
    append_world(&mut s);

    println!("{}", s); // 应输出 "hello, world"

    // 额外练习：修改数字
    let mut num = 5;
    double_value(&mut num);
    println!("双倍值: {}", num); // 应输出 10
}

/// 将数字翻倍
///
/// # TODO
/// 实现这个函数，使引用指向的值翻倍
fn double_value(n: &mut i32) {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_world() {
        let mut s = String::from("hello");
        append_world(&mut s);
        assert_eq!(s, "hello, world");
    }

    #[test]
    fn test_append_world_custom() {
        let mut s = String::from("rust");
        append_world(&mut s);
        assert_eq!(s, "rust, world");
    }

    #[test]
    fn test_double_value() {
        let mut n = 5;
        double_value(&mut n);
        assert_eq!(n, 10);
    }

    #[test]
    fn test_double_value_zero() {
        let mut n = 0;
        double_value(&mut n);
        assert_eq!(n, 0);
    }

    #[test]
    fn test_double_value_negative() {
        let mut n = -3;
        double_value(&mut n);
        assert_eq!(n, -6);
    }
}
