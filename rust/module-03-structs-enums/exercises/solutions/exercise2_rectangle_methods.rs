//! # 练习 2 解答: 结构体方法
//!
//! **提示**：请先尝试自己实现，再参考此解答！
//!
//! # 实现思路
//!
//! 1. `impl` 块：用于为结构体定义方法
//! 2. `&self`：借用 self，不获取所有权
//! 3. 方法：可以在 impl 块中定义多个方法
//! 4. 关联函数：不带 self 参数的函数（如 `new`）

/// 矩形结构体
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    /// 创建新矩形
    ///
    /// # 实现思路
    /// 这是一个关联函数，类似于其他语言中的静态方法
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    /// 计算面积
    ///
    /// # 实现思路
    /// 使用 `&self` 借用，调用者仍拥有矩形
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    /// 判断是否为正方形
    ///
    /// # 实现思路
    /// 比较宽度和高度是否相等
    pub fn is_square(&self) -> bool {
        self.width == self.height
    }

    /// 计算周长
    ///
    /// # 实现思路
    /// 周长 = 2 × (宽 + 高)
    pub fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    /// 缩放矩形
    ///
    /// # 实现思路
    /// 返回一个新的矩形，不修改原矩形
    pub fn scale(&self, factor: u32) -> Rectangle {
        Rectangle {
            width: self.width * factor,
            height: self.height * factor,
        }
    }

    /// 创建正方形（关联函数）
    pub fn square(size: u32) -> Self {
        Rectangle {
            width: size,
            height: size,
        }
    }

    /// 判断是否可以容纳另一个矩形
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
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

    let square = Rectangle::square(5);
    println!("\n正方形:");
    println!("边长: {}, 面积: {}", square.width, square.area());
    println!("是否为正方形: {}", square.is_square());
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

        assert_eq!(rect.width, 10);
        assert_eq!(rect.height, 20);
    }

    #[test]
    fn test_square() {
        let square = Rectangle::square(7);
        assert_eq!(square.width, 7);
        assert_eq!(square.height, 7);
        assert!(square.is_square());
    }

    #[test]
    fn test_can_hold() {
        let big = Rectangle::new(10, 10);
        let small = Rectangle::new(5, 5);

        assert!(big.can_hold(&small));
        assert!(!small.can_hold(&big));
    }

    #[test]
    fn test_multiple_impl_blocks() {
        // Rust 允许多个 impl 块
        impl Rectangle {
            pub fn diagonal(&self) -> f64 {
                ((self.width as f64).powi(2) + (self.height as f64).powi(2)).sqrt()
            }
        }

        let rect = Rectangle::new(3, 4);
        assert_eq!(rect.diagonal(), 5.0);
    }
}
