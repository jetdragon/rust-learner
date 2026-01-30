//! 练习 3: 元组解构
//!
//! # 目标
//! 学习元组操作、解构和浮点数运算
//!
//! # 任务
//! 编写一个函数，计算三维坐标到原点的距离
//!
//! # 难度
//! 中等
//!
//! # 预计时间
//! 15 分钟
//!
//! # 前置知识
//! - 元组类型
//! - 元素访问 (.0, .1, .2)
//! - 数学函数 (powi, sqrt)

/// 计算三维坐标到原点的距离
///
/// # 参数
/// * `coordinates` - 包含三个 f64 的元组 (x, y, z)
///
/// # 返回值
/// 到原点的距离
///
/// # 公式
/// 距离 = √(x² + y² + z²)
///
/// # 示例
/// ```rust
/// use module_01_basics::exercises::distance_to_origin;
///
/// assert_eq!(distance_to_origin((0.0, 0.0, 0.0)), 0.0);
/// assert_eq!(distance_to_origin((3.0, 4.0, 0.0)), 5.0);
/// ```
pub fn distance_to_origin(coordinates: (f64, f64, f64)) -> f64 {
    // TODO: 计算到原点的距离
    // 提示:
    // 1. 通过 .0, .1, .2 访问元组元素
    // 2. 使用 .powi(2) 计算平方
    // 3. 使用 .sqrt() 计算平方根
    unimplemented!()
}

/// 计算两点之间的距离（可选的额外挑战）
///
/// # 参数
/// * `point1` - 第一个点的坐标
/// * `point2` - 第二个点的坐标
///
/// # 返回值
/// 两点之间的距离
///
/// # 公式
/// 距离 = √((x2-x1)² + (y2-y1)² + (z2-z1)²)
pub fn distance_between_points(
    point1: (f64, f64, f64),
    point2: (f64, f64, f64),
) -> f64 {
    // TODO (可选): 实现两点间距离计算
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_origin() {
        // 原点到自身的距离为 0
        assert_eq!(distance_to_origin((0.0, 0.0, 0.0)), 0.0);
    }

    #[test]
    fn test_on_axis() {
        // (3, 0, 0) 到原点的距离是 3
        assert_eq!(distance_to_origin((3.0, 0.0, 0.0)), 3.0);
    }

    #[test]
    fn test_3_4_5_triangle() {
        // (3, 4, 0) 到原点的距离是 5 (3-4-5 三角形)
        assert_eq!(distance_to_origin((3.0, 4.0, 0.0)), 5.0);
    }

    #[test]
    fn test_diagonal() {
        // (1, 1, 1) 到原点的距离是 √3
        let result = distance_to_origin((1.0, 1.0, 1.0));
        assert!((result - 1.73205).abs() < 0.001);
    }

    #[test]
    fn test_distance_between_points() {
        // 测试两点间距离
        let result = distance_between_points((0.0, 0.0, 0.0), (1.0, 0.0, 0.0));
        assert_eq!(result, 1.0);
    }
}
