//! # 练习 3 解答: 元组解构
//!
//! **提示**：请先尝试自己实现，再参考此解答！
//!
//! # 实现思路
//!
//! 1. 通过 `.0`, `.1`, `.2` 访问元组的三个元素
//! 2. 使用 `.powi(2)` 计算平方（整数指数）
//! 3. 使用 `.sqrt()` 计算平方根
//! 4. 可以通过元组解构让代码更清晰：`let (x, y, z) = coordinates;`

/// 计算三维坐标到原点的距离
pub fn distance_to_origin(coordinates: (f64, f64, f64)) -> f64 {
    let (x, y, z) = coordinates;
    (x.powi(2) + y.powi(2) + z.powi(2)).sqrt()
}

// 不使用解构的版本：
// pub fn distance_to_origin(coordinates: (f64, f64, f64)) -> f64 {
//     let x_squared = coordinates.0.powi(2);
//     let y_squared = coordinates.1.powi(2);
//     let z_squared = coordinates.2.powi(2);
//     (x_squared + y_squared + z_squared).sqrt()
// }

/// 计算两点之间的距离
pub fn distance_between_points(
    point1: (f64, f64, f64),
    point2: (f64, f64, f64),
) -> f64 {
    let (x1, y1, z1) = point1;
    let (x2, y2, z2) = point2;

    let dx = x2 - x1;
    let dy = y2 - y1;
    let dz = z2 - z1;

    (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_origin() {
        assert_eq!(distance_to_origin((0.0, 0.0, 0.0)), 0.0);
    }

    #[test]
    fn test_on_axis() {
        assert_eq!(distance_to_origin((3.0, 0.0, 0.0)), 3.0);
    }

    #[test]
    fn test_3_4_5_triangle() {
        assert_eq!(distance_to_origin((3.0, 4.0, 0.0)), 5.0);
    }

    #[test]
    fn test_diagonal() {
        let result = distance_to_origin((1.0, 1.0, 1.0));
        assert!((result - 1.73205).abs() < 0.001);
    }

    #[test]
    fn test_distance_between_points() {
        assert_eq!(distance_between_points((0.0, 0.0, 0.0), (1.0, 0.0, 0.0)), 1.0);
        assert_eq!(distance_between_points((1.0, 2.0, 3.0), (4.0, 6.0, 8.0)), (3.0_f64.powi(2) + 4.0_f64.powi(2) + 5.0_f64.powi(2)).sqrt());
    }
}
