//! 练习 2: 函数与所有权
//!
//! # 目标
//! 理解所有权如何在函数调用时转移
//!
//! # 任务
//! 编写函数演示 String 移动和 i32 拷贝的区别
//!
//! # 难度
//! 简单
//!
//! # 预计时间
//! 10 分钟
//!
//! # 前置知识
//! - 函数参数的所有权转移
//! - Copy trait
//! - 基本类型与集合类型的区别

/// 接收 String 的所有权并打印
///
/// # 参数
/// * `s` - String 的所有权转移到函数内
///
/// # 注意
/// 调用此函数后，原 String 变量将不再有效
pub fn take_ownership(s: String) {
    println!("拥有: {}", s);
} // s 在这里离开作用域并被释放

/// 接收 i32 的拷贝并打印
///
/// # 参数
/// * `n` - i32 的拷贝
///
/// # 注意
/// 调用此函数后，原变量仍然有效（因为 i32 实现了 Copy）
pub fn make_copy(n: i32) {
    println!("拷贝: {}", n);
}

/// 返回 String 的所有权
///
/// # 参数
/// * `s` - String 的所有权
///
/// # 返回值
/// String 的所有权返回给调用者
pub fn give_ownership(s: String) -> String {
    println!("传递所有权: {}", s);
    s // 所有权返回给调用者
}

/// 接收 i32 并返回加倍后的值
///
/// # 参数
/// * `n` - i32 的拷贝
///
/// # 返回值
/// 加倍后的 i32
pub fn double_and_return(n: i32) -> i32 {
    n * 2
}

fn main() {
    let s = String::from("hello");
    let x = 5;

    // 演示 String 的移动
    take_ownership(s);
    // 下面的行会编译错误，因为 s 的所有权已移动
    // println!("{}", s);

    // 演示 i32 的拷贝
    make_copy(x);
    // x 仍然有效，因为 i32 实现了 Copy
    println!("x 仍然是: {}", x);

    // 演示所有权的返回
    let s2 = give_ownership(String::from("world"));
    println!("s2: {}", s2);

    // 演示拷贝和返回
    let y = double_and_return(x);
    println!("y: {}, x: {}", y, x);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take_ownership() {
        let s = String::from("test");
        take_ownership(s);
        // s 在这里不再有效，不能使用
    }

    #[test]
    fn test_make_copy() {
        let x = 42;
        make_copy(x);
        assert_eq!(x, 42); // x 仍然有效
    }

    #[test]
    fn test_give_ownership() {
        let s = give_ownership(String::from("return"));
        assert_eq!(s, "return");
    }

    #[test]
    fn test_double_and_return() {
        let x = 10;
        let result = double_and_return(x);
        assert_eq!(result, 20);
        assert_eq!(x, 10); // x 仍然有效
    }
}
