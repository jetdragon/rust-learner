//! # 练习 4: 泛型结构体 - 解答
//!
//! 难度: 简单
//! 时间: 10 分钟
//! 前置知识: struct 基础
//! 学习目标:
//!   - 定义泛型结构体
//!   - 为泛型结构体实现方法

use std::fmt;

/// Point 结构体 - 泛型点
///
/// 使用泛型 T 表示坐标类型
/// - 可以是 i32, f32, f64 等
/// - x 和 y 必须是相同类型
///
/// # 类型参数
///
/// * `T` - 坐标类型，必须实现 Debug 用于打印
#[derive(Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

// ============= 解决方案 =============

/// 为所有 Point<T> 实现方法
///
/// impl<T> 表示这是一个泛型实现块
/// T 可以是任何类型
impl<T> Point<T> {
    /// 创建新的 Point
    ///
    /// 这是一个关联函数（类似静态方法）
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    /// 返回 x 的引用
    ///
    /// 实现说明:
    /// - 返回 &T 而不是 T，避免所有权转移
    /// - 这是 getter 方法的标准模式
    pub fn x(&self) -> &T {
        &self.x
    }

    /// 返回 y 的引用
    pub fn y(&self) -> &T {
        &self.y
    }

    /// 混合 x 和 y 的值（需要 T 支持 Clone）
    ///
    /// 这个方法展示了 trait bound 在方法中的使用
    pub fn mix(&self) -> (T, T)
    where
        T: Clone,
    {
        (self.x.clone(), self.y.clone())
    }
}

/// 为特定类型 Point<f32> 实现特定方法
///
/// 这是 Rust 的"特化"（specialization）功能
/// - 只为 f32 类型的 Point 提供 distance_from_origin
/// - 其他类型如 Point<i32> 没有这个方法
///
/// 为什么需要这个:
/// - 计算距离需要浮点数运算
/// - 整数类型无法直接计算平方根
///
/// # 示例
///
/// ```
/// let p = Point::new(1.0, 2.0);
/// assert_eq!(p.distance_from_origin(), 2.236068);
/// ```
impl Point<f32> {
    /// 计算到原点的距离
    ///
    /// 数学公式: sqrt(x² + y²)
    ///
    /// 实现细节:
    /// - powi(2) 计算平方（整数指数）
    /// - sqrt() 计算平方根
    /// - 这是 f32 特有的方法
    pub fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

/// 另一个特化示例: Point<T> where T: Display
///
/// 为实现了 Display 的类型提供额外方法
impl<T: fmt::Display> Point<T> {
    /// 格式化点信息
    pub fn formatted(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

// ============= 测试代码 =============

fn main() {
    println!("=== 整数点 ===");
    let integer_point = Point::new(5, 10);
    println!("Integer point: {:?}", integer_point);
    println!("x = {}", integer_point.x());
    println!("y = {}", integer_point.y());
    println!("Formatted: {}", integer_point.formatted());

    println!("\n=== 浮点点 ===");
    let float_point = Point::new(1.0, 4.0);
    println!("Float point: {:?}", float_point);
    println!("Distance from origin: {}", float_point.distance_from_origin());
    println!("Formatted: {}", float_point.formatted());

    println!("\n=== 不同类型的点 ===");
    let float_point2 = Point::new(3.14, 2.71);
    println!("Float point 2: {:?}", float_point2);
    println!("Distance: {}", float_point2.distance_from_origin());

    println!("\n=== 方法链式调用 ===");
    let p = Point::new(3, 4);
    println!("Point (3, 4): {:?}", p);
    println!("Formatted: {}", p.formatted());
}

// ============= 扩展示例 =============

/// 多个类型参数的结构体
///
/// 展示了如何使用多个泛型参数
#[derive(Debug)]
pub struct Point2<T, U> {
    pub x: T,
    pub y: U,
}

impl<T, U> Point2<T, U> {
    pub fn new(x: T, y: U) -> Self {
        Point2 { x, y }
    }

    /// 混合两个不同类型的点
    ///
    /// 这个方法返回一个新点，x 来自 self，y 来自 other
    pub fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

/// 为特定类型组合实现方法
impl Point2<f32, f32> {
    /// 仅当两个坐标都是 f32 时才可用
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_accessors() {
        let p = Point::new(5, 10);
        assert_eq!(*p.x(), 5);
        assert_eq!(*p.y(), 10);
    }

    #[test]
    fn test_distance_calculation() {
        let p = Point::new(3.0, 4.0);
        assert!((p.distance_from_origin() - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_mixup() {
        let p1 = Point2::new(5, 10.4);
        let p2 = Point2::new("Hello", 'c');
        let p3 = p1.mixup(p2);
        assert_eq!(p3.x, 5);
        assert_eq!(p3.y, 'c');
    }
}

// ============= 扩展知识 =============

/*
泛型结构体的关键点:

1. 声明泛型:
   struct Name<T> { field: T }

2. 多个泛型:
   struct Name<T, U> { field1: T, field2: U }

3. 为所有 T 实现方法:
   impl<T> Struct<T> { ... }

4. 为特定 T 实现方法:
   impl Struct<f32> { ... }

5. 带 trait bound 的实现:
   impl<T: Trait> Struct<T> { ... }

方法解析:
1. Rust 首先查找特定类型的实现
2. 然后查找更通用的实现
3. 更具体的实现优先

性能考虑:
- 泛型在编译时单态化（monomorphization）
- 每个具体类型生成独立的代码
- 运行时性能与手写特定类型相同
- 缺点是可能增加二进制大小

最佳实践:
1. 使用泛型避免代码重复
2. 为常用类型提供优化实现
3. 使用 trait bound 约束泛型
4. 考虑使用 where 子句提高可读性
*/
