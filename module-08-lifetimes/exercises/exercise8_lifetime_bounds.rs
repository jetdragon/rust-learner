//! # 练习 8: 生命周期约束
//!
//! 难度: 中等
//! 时间: 15 分钟
//! 前置知识: 生命周期子类型
//! 学习目标:
//!   - 使用生命周期约束 (T: 'a)
//!   - 理解生命周期之间的关系

// TODO: 添加生命周期约束
pub fn holds_ref<'a, T>(t: &'a T) -> bool
where
    T: 'a, // T 必须活得至少和 'a 一样长
{
    true
}

pub struct BoxHolder<'a, T: ?Sized> {
    pub value: &'a T,
}

impl<'a, T: ?Sized> BoxHolder<'a, T> {
    pub fn new(value: &'a T) -> Self {
        BoxHolder { value }
    }
}

// TODO: 为什么这个函数需要生命周期约束？
pub fn get_first<'a, T: ?Sized>(items: &'a [&T]) -> Option<&'a T> {
    items.first().copied()
}

fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let first = get_first(&numbers[..]);
    println!("First: {:?}", first);

    let strings: &[&str] = &["hello", "world"];
    let first_str = get_first(strings);
    println!("First str: {:?}", first_str);
}
