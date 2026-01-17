//! # 练习 3: 解构结构体
//!
//! **难度**: 入门
//! **预计时间**: 15 分钟

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

/// TODO: 返回点所在的象限
///
/// 返回点所在的象限：
/// - (+, +): "第一象限"
/// - (-, +): "第二象限"
/// - (-, -): "第三象限"
/// - (+, -): "第四象限"
/// - (0, 0): "原点"
/// - 在轴上: "坐标轴"
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

/// TODO: 计算两点间的曼哈顿距离
pub fn manhattan_distance(p1: &Point, p2: &Point) -> i32 {
    unimplemented!()
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
}
