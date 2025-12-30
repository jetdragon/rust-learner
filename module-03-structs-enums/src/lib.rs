//! # 结构体与枚举 (Structs & Enums)
//!
//! 本模块演示 Rust 的自定义类型系统，包括：
//! - 结构体的定义和使用
//! - 枚举的定义和使用
//! - 方法语法
//! - Option 和 Result 类型
//!
//! ## 示例
//!
//! ```
//! use module_03_structs_enums::{Rectangle, TrafficLight, Value};
//!
//! let rect = Rectangle { width: 10, height: 20 };
//! println!("面积: {}", rect.area());
//!
//! let light = TrafficLight::Red;
//! println!("时长: {}秒", light.duration());
//!
//! let v: Value = 42.into();
//! println!("值: {}", v);
//! ```

// ==================== 结构体 ====================

/// 矩形结构体
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    /// 创建新的矩形
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// 计算矩形面积
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    /// 判断是否为正方形
    pub fn is_square(&self) -> bool {
        self.width == self.height
    }

    /// 创建正方形
    pub fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    /// 返回能容纳 self 的另一个矩形
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// ==================== 枚举 ====================

/// 交通信号灯
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    /// 返回信号灯持续时长（秒）
    pub fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 3,
            TrafficLight::Green => 25,
        }
    }

    /// 返回下一个信号灯
    pub fn next(&self) -> Self {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Green => TrafficLight::Yellow,
            TrafficLight::Yellow => TrafficLight::Red,
        }
    }
}

/// IP 地址类型
#[derive(Debug, Clone, PartialEq)]
pub enum IpAddr {
    V4(String),
    V6(String),
}

impl IpAddr {
    /// 返回 IP 地址的字符串表示
    pub fn address(&self) -> &str {
        match self {
            IpAddr::V4(addr) | IpAddr::V6(addr) => addr,
        }
    }

    /// 判断是否为 IPv4
    pub fn is_v4(&self) -> bool {
        matches!(self, IpAddr::V4(_))
    }
}

/// 几何图形
#[derive(Debug, Clone)]
pub enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    /// 计算图形面积
    pub fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }
}

// ==================== 通用值类型 ====================

/// 通用的值类型，可以存储不同类型的数据
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Text(String),
    Boolean(bool),
    Null,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Text(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Null => write!(f, "NULL"),
        }
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Value::Integer(v)
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::Float(v)
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Value::Text(v)
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Value::Text(v.to_string())
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Value::Boolean(v)
    }
}

// ==================== 辅助函数 ====================

/// 安全除法，返回 Result
pub fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("除数不能为零".to_string())
    } else {
        Ok(a / b)
    }
}

/// 找出数组中第二大的值
pub fn find_second_largest(numbers: &[i32]) -> Option<i32> {
    if numbers.len() < 2 {
        return None;
    }

    let mut largest = i32::MIN;
    let mut second_largest = i32::MIN;

    for &num in numbers {
        if num > largest {
            second_largest = largest;
            largest = num;
        } else if num > second_largest && num != largest {
            second_largest = num;
        }
    }

    if second_largest == i32::MIN {
        None
    } else {
        Some(second_largest)
    }
}

// ==================== 用户结构体示例 ====================

/// 用户结构体
#[derive(Debug, Clone)]
pub struct User {
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
    pub active: bool,
}

impl User {
    /// 创建新用户
    pub fn new(username: String, email: String) -> Self {
        Self {
            username,
            email,
            sign_in_count: 0,
            active: true,
        }
    }

    /// 增加登录次数
    pub fn increment_sign_in_count(&mut self) {
        self.sign_in_count += 1;
    }

    /// 返回用户状态
    pub fn status(&self) -> &str {
        if self.active {
            "活跃"
        } else {
            "未激活"
        }
    }
}

// ==================== 元组结构体 ====================

/// 颜色 (RGB)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    /// 创建白色
    pub fn white() -> Self {
        Color(255, 255, 255)
    }

    /// 创建黑色
    pub fn black() -> Self {
        Color(0, 0, 0)
    }

    /// 转换为十六进制字符串
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }
}

/// 二维点
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point(pub i32, pub i32);

impl Point {
    /// 创建原点
    pub fn origin() -> Self {
        Point(0, 0)
    }

    /// 计算到另一个点的距离
    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = (self.0 - other.0) as f64;
        let dy = (self.1 - other.1) as f64;
        (dx * dx + dy * dy).sqrt()
    }
}

// ==================== 单元结构体 ====================

/// 单元结构体示例
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlwaysEqual;

impl AlwaysEqual {
    /// 比较两个值是否相等（总是返回 true）
    pub fn equals(&self, _other: &AlwaysEqual) -> bool {
        true
    }
}

// ==================== 测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    // Rectangle 测试
    #[test]
    fn test_rectangle_area() {
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.area(), 200);
    }

    #[test]
    fn test_rectangle_is_square() {
        assert!(Rectangle::square(10).is_square());
        assert!(!Rectangle::new(10, 20).is_square());
    }

    #[test]
    fn test_rectangle_can_hold() {
        let larger = Rectangle::new(10, 10);
        let smaller = Rectangle::new(5, 5);
        assert!(larger.can_hold(&smaller));
    }

    // TrafficLight 测试
    #[test]
    fn test_traffic_light_duration() {
        assert_eq!(TrafficLight::Red.duration(), 30);
        assert_eq!(TrafficLight::Yellow.duration(), 3);
        assert_eq!(TrafficLight::Green.duration(), 25);
    }

    #[test]
    fn test_traffic_light_next() {
        assert_eq!(TrafficLight::Red.next(), TrafficLight::Green);
        assert_eq!(TrafficLight::Green.next(), TrafficLight::Yellow);
        assert_eq!(TrafficLight::Yellow.next(), TrafficLight::Red);
    }

    // IpAddr 测试
    #[test]
    fn test_ip_addr() {
        let v4 = IpAddr::V4("127.0.0.1".to_string());
        assert!(v4.is_v4());
        assert_eq!(v4.address(), "127.0.0.1");
    }

    // Shape 测试
    #[test]
    fn test_shape_area() {
        let circle = Shape::Circle { radius: 1.0 };
        assert!((circle.area() - std::f64::consts::PI).abs() < 0.001);

        let rect = Shape::Rectangle { width: 10.0, height: 5.0 };
        assert_eq!(rect.area(), 50.0);
    }

    // Value 测试
    #[test]
    fn test_value_from() {
        let v: Value = 42.into();
        assert!(matches!(v, Value::Integer(42)));

        let v: Value = 3.14.into();
        assert!(matches!(v, Value::Float(_)));

        let v: Value = "hello".into();
        assert!(matches!(v, Value::Text(_)));

        let v: Value = true.into();
        assert!(matches!(v, Value::Boolean(true)));
    }

    #[test]
    fn test_value_display() {
        assert_eq!(format!("{}", Value::Integer(42)), "42");
        assert_eq!(format!("{}", Value::Text("hello".to_string())), "hello");
        assert_eq!(format!("{}", Value::Boolean(true)), "true");
        assert_eq!(format!("{}", Value::Null), "NULL");
    }

    // safe_divide 测试
    #[test]
    fn test_safe_divide_success() {
        assert_eq!(safe_divide(10.0, 2.0), Ok(5.0));
    }

    #[test]
    fn test_safe_divide_error() {
        assert_eq!(safe_divide(10.0, 0.0), Err("除数不能为零".to_string()));
    }

    // find_second_largest 测试
    #[test]
    fn test_find_second_largest() {
        assert_eq!(find_second_largest(&[1, 2, 3, 4, 5]), Some(4));
        assert_eq!(find_second_largest(&[5, 5, 4, 3]), Some(4));
        assert_eq!(find_second_largest(&[1]), None);
        assert_eq!(find_second_largest(&[]), None);
    }

    // User 测试
    #[test]
    fn test_user_new() {
        let user = User::new("alice".to_string(), "alice@example.com".to_string());
        assert_eq!(user.username, "alice");
        assert_eq!(user.sign_in_count, 0);
        assert!(user.active);
    }

    #[test]
    fn test_user_increment() {
        let mut user = User::new("bob".to_string(), "bob@example.com".to_string());
        user.increment_sign_in_count();
        assert_eq!(user.sign_in_count, 1);
    }

    // Color 测试
    #[test]
    fn test_color() {
        assert_eq!(Color::white().to_hex(), "#FFFFFF");
        assert_eq!(Color::black().to_hex(), "#000000");
    }

    // Point 测试
    #[test]
    fn test_point_distance() {
        let p1 = Point::origin();
        let p2 = Point(3, 4);
        assert_eq!(p1.distance_to(&p2), 5.0);
    }

    // AlwaysEqual 测试
    #[test]
    fn test_always_equal() {
        let a = AlwaysEqual;
        let b = AlwaysEqual;
        assert!(a.equals(&b));
    }
}
