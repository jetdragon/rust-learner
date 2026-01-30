//! # 练习 2: 结构体方法
//!
//! **难度**: 入门
//! **预计时间**: 15 分钟
//!
//! **前置知识**:
//! - 结构体基础定义
//! - 基本的函数定义
//!
//! **学习目标**:
//! - 学习如何为结构体实现方法
//! - 理解 `impl` 块的用法
//! - 掌握 `&self` 和 `&mut self` 的区别

/// 矩形结构体
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    /// 创建新矩形
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    /// TODO: 实现计算面积的方法
    ///
    /// 返回矩形的面积（宽 × 高）
    pub fn area(&self) -> u32 {
        unimplemented!()
    }

    /// TODO: 实现判断是否为正方形的方法
    ///
    /// 返回 true 如果宽高相等
    pub fn is_square(&self) -> bool {
        unimplemented!()
    }

    /// TODO: 实现计算周长的方法
    ///
    /// 返回矩形的周长（2 × (宽 + 高)）
    pub fn perimeter(&self) -> u32 {
        unimplemented!()
    }

    /// TODO: 实现缩放方法
    ///
    /// 返回一个新的矩形，宽高都乘以 factor
    pub fn scale(&self, factor: u32) -> Rectangle {
        unimplemented!()
    }
}

fn main() {
    let rect = Rectangle::new(10, 20);

    println!("=== 矩形信息 ===");
    println!("宽: {}, 高: {}", rect.width, rect.height);
    println!("面积: {}", rect.area());
    println!("周长: {}", rect.perimeter());
    println!("是否为正方形: {}", rect.is_square());

    let scaled = rect.scale(2);
    println!("\n缩放 2 倍后:");
    println!("宽: {}, 高: {}", scaled.width, scaled.height);
    println!("面积: {}", scaled.area());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.area(), 200);
    }

    #[test]
    fn test_area_square() {
        let rect = Rectangle::new(5, 5);
        assert_eq!(rect.area(), 25);
    }

    #[test]
    fn test_is_square_true() {
        let rect = Rectangle::new(5, 5);
        assert!(rect.is_square());
    }

    #[test]
    fn test_is_square_false() {
        let rect = Rectangle::new(10, 20);
        assert!(!rect.is_square());
    }

    #[test]
    fn test_perimeter() {
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.perimeter(), 60);
    }

    #[test]
    fn test_scale() {
        let rect = Rectangle::new(10, 20);
        let scaled = rect.scale(2);

        assert_eq!(scaled.width, 20);
        assert_eq!(scaled.height, 40);
    }

    #[test]
    fn test_scale_by_three() {
        let rect = Rectangle::new(5, 7);
        let scaled = rect.scale(3);

        assert_eq!(scaled.width, 15);
        assert_eq!(scaled.height, 21);
        assert_eq!(scaled.area(), 315);
    }

    #[test]
    fn test_original_unchanged() {
        let rect = Rectangle::new(10, 20);
        let _scaled = rect.scale(2);

        // 原矩形不应改变
        assert_eq!(rect.width, 10);
        assert_eq!(rect.height, 20);
    }
}
