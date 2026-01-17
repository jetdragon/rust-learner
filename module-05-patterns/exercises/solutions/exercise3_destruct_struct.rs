//! # 练习 3 解答: 解构结构体
//!
//! **提示**：请先尝试自己实现，再参考此解答！
//!
//! # 实现思路
//!
//! 1. 解构结构体：使用模式匹配提取字段
//! 2. 元组匹配：匹配多个值的组合
//! 3. 守卫：使用 if 添加额外条件
//! 4. 通配符：使用 _ 忽略不需要的值

/// 二维点
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

/// 返回点所在的象限
pub fn quadrant(p: &Point) -> &'static str {
    match (p.x, p.y) {
        (0, 0) => "原点",
        (0, _) | (_, 0) => "坐标轴",
        (x, y) if x > 0 && y > 0 => "第一象限",
        (x, y) if x < 0 && y > 0 => "第二象限",
        (x, y) if x < 0 && y < 0 => "第三象限",
        _ => "第四象限",
    }
}

/// 计算两点间的曼哈顿距离
pub fn manhattan_distance(p1: &Point, p2: &Point) -> i32 {
    let Point { x: x1, y: y1 } = p1;
    let Point { x: x2, y: y2 } = p2;
    (x2 - x1).abs() + (y2 - y1).abs()
}

/// 计算两点间的欧几里得距离（平方）
pub fn distance_squared(p1: &Point, p2: &Point) -> i32 {
    let Point { x: x1, y: y1 } = p1;
    let Point { x: x2, y: y2 } = p2;
    (x2 - x1).pow(2) + (y2 - y1).pow(2)
}

fn main() {
    let points = vec![
        Point::new(1, 1),
        Point::new(-1, 1),
        Point::new(-1, -1),
        Point::new(1, -1),
        Point::new(0, 0),
        Point::new(0, 5),
    ];

    println!("=== 象限判断 ===\n");
    for p in &points {
        println!("Point({:?}, {:?}) -> {}", p.x, p.y, quadrant(p));
    }

    // 距离计算
    let p1 = Point::new(0, 0);
    let p2 = Point::new(3, 4);
    println!("\n曼哈顿距离: {}", manhattan_distance(&p1, &p2));
    println!("距离平方: {}", distance_squared(&p1, &p2));

    // 解构示例
    let Point { x, y } = Point::new(10, 20);
    println!("\n解构: x={}, y={}", x, y);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quadrant() {
        assert_eq!(quadrant(&Point { x: 1, y: 1 }), "第一象限");
        assert_eq!(quadrant(&Point { x: -1, y: 1 }), "第二象限");
        assert_eq!(quadrant(&Point { x: -1, y: -1 }), "第三象限");
        assert_eq!(quadrant(&Point { x: 1, y: -1 }), "第四象限");
        assert_eq!(quadrant(&Point { x: 0, y: 0 }), "原点");
        assert_eq!(quadrant(&Point { x: 0, y: 5 }), "坐标轴");
    }

    #[test]
    fn test_manhattan_distance() {
        let p1 = Point::new(0, 0);
        let p2 = Point::new(3, 4);
        assert_eq!(manhattan_distance(&p1, &p2), 7);
    }

    #[test]
    fn test_distance_squared() {
        let p1 = Point::new(0, 0);
        let p2 = Point::new(3, 4);
        assert_eq!(distance_squared(&p1, &p2), 25);
    }

    #[test]
    fn test_destructuring() {
        let p = Point { x: 10, y: 20 };
        let Point { x, y: y_coord } = p;
        assert_eq!(x, 10);
        assert_eq!(y_coord, 20);
    }
}
