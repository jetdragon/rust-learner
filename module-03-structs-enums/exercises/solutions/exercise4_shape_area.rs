//! # 练习 4 解答: 带数据的枚举
//!
//! **提示**：请先尝试自己实现，再参考此解答！
//!
//! # 实现思路
//!
//! 1. 结构化变体：枚举变体可以携带数据
//! 2. 解构：在 match 中解构枚举数据
//! 3. 命名字段：使用类似结构体的语法
//! 4. 方法：在 impl 块中为枚举实现方法

/// 几何图形枚举
pub enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    /// 计算图形面积
    ///
    /// # 实现思路
    /// 解构每个变体的数据并计算
    pub fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }

    /// 返回图形的描述
    ///
    /// # 实现思路
    /// 使用 match 解构并格式化输出
    pub fn describe(&self) -> String {
        match self {
            Shape::Circle { radius } => format!("圆形（半径: {}）", radius),
            Shape::Rectangle { width, height } => format!("矩形（宽: {}, 高: {}）", width, height),
            Shape::Triangle { base, height } => format!("三角形（底: {}, 高: {}）", base, height),
        }
    }

    /// 创建圆形
    pub fn circle(radius: f64) -> Self {
        Shape::Circle { radius }
    }

    /// 创建矩形
    pub fn rectangle(width: f64, height: f64) -> Self {
        Shape::Rectangle { width, height }
    }

    /// 创建三角形
    pub fn triangle(base: f64, height: f64) -> Self {
        Shape::Triangle { base, height }
    }

    /// 判断是否为圆形
    pub fn is_circle(&self) -> bool {
        matches!(self, Shape::Circle { .. })
    }

    /// 获取图形名称
    pub fn name(&self) -> &str {
        match self {
            Shape::Circle { .. } => "圆形",
            Shape::Rectangle { .. } => "矩形",
            Shape::Triangle { .. } => "三角形",
        }
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
        println!("类型: {}", shape.name());
        println!("面积: {:.2}\n", shape.area());
    }

    // 演示使用 `..` 忽略字段
    let circle = Shape::circle(3.0);
    if let Shape::Circle { .. } = circle {
        println!("这是一个圆形");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PI: f64 = std::f64::consts::PI;

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
        assert!(desc.contains("圆形"));
        assert!(desc.contains("3"));
    }

    #[test]
    fn test_rectangle_describe() {
        let rect = Shape::rectangle(5.0, 8.0);
        let desc = rect.describe();
        assert!(desc.contains("矩形"));
    }

    #[test]
    fn test_triangle_describe() {
        let triangle = Shape::triangle(4.0, 5.0);
        let desc = triangle.describe();
        assert!(desc.contains("三角形"));
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

    #[test]
    fn test_is_circle() {
        assert!(Shape::circle(5.0).is_circle());
        assert!(!Shape::rectangle(5.0, 5.0).is_circle());
        assert!(!Shape::triangle(5.0, 5.0).is_circle());
    }

    #[test]
    fn test_name() {
        assert_eq!(Shape::circle(1.0).name(), "圆形");
        assert_eq!(Shape::rectangle(1.0, 1.0).name(), "矩形");
        assert_eq!(Shape::triangle(1.0, 1.0).name(), "三角形");
    }

    #[test]
    fn test_enum_with_data() {
        // 演示如何匹配并使用字段数据
        let shape = Shape::Circle { radius: 10.0 };

        match shape {
            Shape::Circle { radius } => {
                assert_eq!(radius, 10.0);
            }
            _ => panic!("不是圆形"),
        }
    }

    #[test]
    fn test_wildcard_pattern() {
        // 使用 .. 忽略某些字段
        let rect = Shape::Rectangle { width: 10.0, height: 20.0 };

        match rect {
            Shape::Rectangle { width, .. } => {
                assert_eq!(width, 10.0);
            }
            _ => panic!("不是矩形"),
        }
    }
}
