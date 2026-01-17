//! # 练习 4: 泛型结构体
//!
//! 难度: 简单
//! 时间: 10 分钟
//! 前置知识: struct 基础
//! 学习目标:
//!   - 定义泛型结构体
//!   - 为泛型结构体实现方法

#[derive(Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    // TODO: 实现 x 方法返回 x 的引用
    pub fn x(&self) -> &T {
        &self.x
    }
}

// TODO: 为 Point<f32> 实现特定方法 distance_from_origin
// 提示: 使用 (x^2 + y^2).sqrt()
impl Point<f32> {
    pub fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let integer_point = Point::new(5, 10);
    let float_point = Point::new(1.0, 4.0);

    println!("Integer point: {:?}, x={}", integer_point, integer_point.x());
    println!("Float point: {:?}", float_point);
}
