//! # 练习 4: 带数据的枚举
//!
//! **难度**: 中等
//! **预计时间**: 20 分钟
//!
//! **前置知识**:
//! - 枚举基础定义
//! - match 表达式
//! - 基本的数学运算
//!
//! **学习目标**:
//! - 学习如何定义带数据的枚举变体
//! - 理解结构化变体
//! - 掌握解构枚举数据

/// 几何图形枚举
///
/// # TODO
///
/// 定义 Shape 枚举，包含以下变体：
/// - Circle { radius: f64 }
/// - Rectangle { width: f64, height: f64 }
/// - Triangle { base: f64, height: f64 }
pub enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    /// TODO: 计算图形面积
    ///
    /// - 圆形面积：π * r²
    /// - 矩形面积：宽 × 高
    /// - 三角形面积：0.5 × 底 × 高
    pub fn area(&self) -> f64 {
        unimplemented!()
    }

    /// TODO: 返回图形的描述
    pub fn describe(&self) -> String {
        unimplemented!()
    }

    /// 创建圆形
    pub fn circle(radius: f64) -> Self {
        unimplemented!()
    }

    /// 创建矩形
    pub fn rectangle(width: f64, height: f64) -> Self {
        unimplemented!()
    }

    /// 创建三角形
    pub fn triangle(base: f64, height: f64) -> Self {
        unimplemented!()
    }
}

fn main() {
    let shapes = vec![
        Shape::circle(5.0),
        Shape::rectangle(10.0, 20.0),
        Shape::triangle(8.0, 6.0),
    ];

    println!("=== 几何图形面积计算 ===\n");

    for shape in shapes {
        println!("{}", shape.describe());
        println!("面积: {:.2}\n", shape.area());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PI: f64 = 3.14159;

    #[test]
    fn test_circle_area() {
        let circle = Shape::circle(5.0);
        let expected = PI * 5.0 * 5.0;
        assert!((circle.area() - expected).abs() < 0.01);
    }

    #[test]
    fn test_rectangle_area() {
        let rect = Shape::rectangle(10.0, 20.0);
        assert_eq!(rect.area(), 200.0);
    }

    #[test]
    fn test_triangle_area() {
        let triangle = Shape::triangle(8.0, 6.0);
        assert_eq!(triangle.area(), 24.0);
    }

    #[test]
    fn test_circle_describe() {
        let circle = Shape::circle(3.0);
        let desc = circle.describe();
        assert!(desc.contains("圆形") || desc.contains("Circle"));
        assert!(desc.contains("3"));
    }

    #[test]
    fn test_rectangle_describe() {
        let rect = Shape::rectangle(5.0, 8.0);
        let desc = rect.describe();
        assert!(desc.contains("矩形") || desc.contains("Rectangle"));
    }

    #[test]
    fn test_triangle_describe() {
        let triangle = Shape::triangle(4.0, 5.0);
        let desc = triangle.describe();
        assert!(desc.contains("三角形") || desc.contains("Triangle"));
    }

    #[test]
    fn test_circle_zero_radius() {
        let circle = Shape::circle(0.0);
        assert_eq!(circle.area(), 0.0);
    }

    #[test]
    fn test_triangle_small() {
        let triangle = Shape::triangle(2.0, 3.0);
        assert_eq!(triangle.area(), 3.0);
    }

    #[test]
    fn test_rectangle_square() {
        let square = Shape::rectangle(4.0, 4.0);
        assert_eq!(square.area(), 16.0);
    }
}
