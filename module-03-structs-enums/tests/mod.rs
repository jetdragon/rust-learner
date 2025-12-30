// 03-结构体与枚举 模块测试

use module_03_structs_enums::*;

// Book 结构体定义
struct Book {
    title: String,
    author: String,
    pages: u32,
    available: bool,
}

// ========== 练习 1: 定义结构体 ==========
#[test]
fn test_book_struct() {
    let book = Book {
        title: String::from("Rust Programming"),
        author: String::from("Steve Klabnik"),
        pages: 300,
        available: true,
    };
    assert_eq!(book.title, "Rust Programming");
    assert!(book.available);
}

// ========== 练习 2: 结构体方法 ==========
#[test]
fn test_rectangle_methods() {
    let rect = Rectangle::new(10, 20);
    assert_eq!(rect.area(), 200);
    assert!(!rect.is_square());

    let square = Rectangle::square(5);
    assert_eq!(square.area(), 25);
    assert!(square.is_square());
}

// ========== 练习 3: 定义枚举 ==========
#[test]
fn test_traffic_light() {
    assert_eq!(TrafficLight::Red.duration(), 30);
    assert_eq!(TrafficLight::Yellow.duration(), 3);
    assert_eq!(TrafficLight::Green.duration(), 25);

    assert_eq!(TrafficLight::Red.next(), TrafficLight::Green);
    assert_eq!(TrafficLight::Green.next(), TrafficLight::Yellow);
    assert_eq!(TrafficLight::Yellow.next(), TrafficLight::Red);
}

// ========== 练习 4: 带数据的枚举 ==========
#[test]
fn test_shape_area() {
    let circle = Shape::Circle { radius: 2.0 };
    assert!((circle.area() - 12.566).abs() < 0.01);

    let rect = Shape::Rectangle { width: 10.0, height: 5.0 };
    assert_eq!(rect.area(), 50.0);

    let triangle = Shape::Triangle { base: 6.0, height: 4.0 };
    assert_eq!(triangle.area(), 12.0);
}

// ========== 练习 5: Option 类型使用 ==========
#[test]
fn test_find_second_largest() {
    assert_eq!(find_second_largest(&[1, 2, 3, 4, 5]), Some(4));
    assert_eq!(find_second_largest(&[5, 5, 4, 3]), Some(4));
    assert_eq!(find_second_largest(&[1]), None);
    assert_eq!(find_second_largest(&[]), None);
    assert_eq!(find_second_largest(&[1, 1, 1]), None);
}

// ========== 练习 6: Result 类型使用 ==========
#[test]
fn test_safe_divide() {
    assert_eq!(safe_divide(10.0, 2.0), Ok(5.0));
    assert_eq!(safe_divide(10.0, 0.0), Err("除数不能为零".to_string()));
    assert_eq!(safe_divide(-10.0, 2.0), Ok(-5.0));
}

// ========== 练习 7: match 穷尽性 ==========
#[test]
fn test_color_match() {
    fn color_name(color: &Color) -> &str {
        match color {
            Color(255, 0, 0) => "红色",
            Color(0, 0, 255) => "蓝色",
            Color(0, 255, 0) => "绿色",
            Color(255, 255, 0) => "黄色",
            _ => "其他颜色",
        }
    }

    assert_eq!(color_name(&Color(255, 0, 0)), "红色");
    assert_eq!(color_name(&Color(255, 255, 255)), "其他颜色");
}

// ========== Value 类型测试 ==========
#[test]
fn test_value_conversions() {
    let v: Value = 42.into();
    assert!(matches!(v, Value::Integer(42)));

    let v: Value = 3.14.into();
    assert!(matches!(v, Value::Float(_)));

    let v: Value = String::from("hello").into();
    assert!(matches!(v, Value::Text(_)));

    let v: Value = true.into();
    assert!(matches!(v, Value::Boolean(true)));
}

#[test]
fn test_value_display() {
    assert_eq!(format!("{}", Value::Integer(42)), "42");
    assert_eq!(format!("{}", Value::Float(3.14)), "3.14");
    assert_eq!(format!("{}", Value::Text("hello".to_string())), "hello");
    assert_eq!(format!("{}", Value::Boolean(true)), "true");
    assert_eq!(format!("{}", Value::Null), "NULL");
}

// ========== 用户结构体测试 ==========
#[test]
fn test_user_creation() {
    let user = User::new(String::from("alice"), String::from("alice@example.com"));
    assert_eq!(user.username, "alice");
    assert_eq!(user.sign_in_count, 0);
    assert!(user.active);
    assert_eq!(user.status(), "活跃");
}

#[test]
fn test_user_methods() {
    let mut user = User::new(String::from("bob"), String::from("bob@example.com"));
    user.increment_sign_in_count();
    assert_eq!(user.sign_in_count, 1);
    user.increment_sign_in_count();
    assert_eq!(user.sign_in_count, 2);
}

// ========== 元组结构体测试 ==========
#[test]
fn test_color_struct() {
    let white = Color::white();
    assert_eq!(white.to_hex(), "#FFFFFF");

    let black = Color::black();
    assert_eq!(black.to_hex(), "#000000");
}

#[test]
fn test_point_struct() {
    let p1 = Point::origin();
    let p2 = Point(3, 4);
    assert_eq!(p1.distance_to(&p2), 5.0);

    let p3 = Point(0, 0);
    let p4 = Point(6, 8);
    assert_eq!(p3.distance_to(&p4), 10.0);
}

// ========== 单元结构体测试 ==========
#[test]
fn test_always_equal() {
    let a = AlwaysEqual;
    let b = AlwaysEqual;
    assert!(a.equals(&b));
}

// ========== IpAddr 测试 ==========
#[test]
fn test_ip_addr() {
    let v4 = IpAddr::V4(String::from("127.0.0.1"));
    assert!(v4.is_v4());
    assert_eq!(v4.address(), "127.0.0.1");

    let v6 = IpAddr::V6(String::from("::1"));
    assert!(!v6.is_v4());
    assert_eq!(v6.address(), "::1");
}

// ========== 综合测试 ==========
#[test]
fn test_comprehensive_structs_and_enums() {
    // 创建用户
    let mut user = User::new("alice".to_string(), "alice@example.com".to_string());
    user.increment_sign_in_count();

    // 创建矩形
    let rect = Rectangle::new(10, 20);
    assert!(rect.can_hold(&Rectangle::square(5)));

    // 使用枚举
    let shape = Shape::Circle { radius: 1.0 };
    assert!((shape.area() - 3.14159).abs() < 0.01);

    // Option 和 Result
    let result = safe_divide(rect.area() as f64, 2.0);
    assert_eq!(result, Ok(100.0));

    let second_largest = find_second_largest(&[10, 20, 30, 40, 50]);
    assert_eq!(second_largest, Some(40));
}

// ========== 模式匹配测试 ==========
#[test]
fn test_pattern_matching() {
    let v: Value = 42.into();
    match v {
        Value::Integer(n) => assert_eq!(n, 42),
        _ => panic!("应该是整数"),
    }

    let light = TrafficLight::Red;
    match light {
        TrafficLight::Red => assert_eq!(light.duration(), 30),
        _ => panic!("应该是红灯"),
    }
}

// ========== if let 测试 ==========
#[test]
fn test_if_let() {
    let some_value: Option<i32> = Some(7);

    let mut captured = false;
    if let Some(i) = some_value {
        assert_eq!(i, 7);
        captured = true;
    }
    assert!(captured);

    let none_value: Option<i32> = None;
    if let Some(_) = none_value {
        panic!("不应该有值");
    }
}
