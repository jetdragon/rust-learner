//! # 模式匹配 (Pattern Matching)
//!
//! 本模块提供模式匹配的实用类型和函数示例。
//!
//! ## 模块内容
//!
//! - 枚举类型：`Grade`, `TrafficLight`, `List`
//! - 结构体类型：`Point`
//! - 模式匹配函数：匹配各种模式和类型
//!
//! ## 示例
//!
//! ```
//! use module_04_patterns::grade;
//!
//! assert_eq!(grade(95, 100), "优秀");
//! ```

// ========== 枚举定义 ==========

/// 成绩等级
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grade {
    /// 优秀 (90-100)
    Excellent,
    /// 良好 (80-89)
    Good,
    /// 及格 (60-79)
    Pass,
    /// 不及格 (0-59)
    Fail,
}

/// 交通信号灯
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrafficLight {
    /// 红灯
    Red,
    /// 黄灯
    Yellow,
    /// 绿灯
    Green,
}

/// 链表
#[derive(Debug)]
pub enum List {
    /// 包含一个值和指向下一个节点的指针
    Cons(i32, Box<List>),
    /// 链表末尾
    Nil,
}

/// 点坐标
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    /// 创建新点
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// 判断点是否在坐标轴上
    pub fn is_on_axis(&self) -> bool {
        matches!(self, Point { x: 0, .. } | Point { y: 0, .. })
    }

    /// 判断点是否是原点
    pub fn is_origin(&self) -> bool {
        matches!(self, Point { x: 0, y: 0 })
    }
}

/// 月份
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Month {
    January = 1,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

/// 方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

/// 形状
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

// ========== 模式匹配函数 ==========

/// 根据分数返回评级
///
/// # 示例
///
/// ```
/// use module_04_patterns::grade;
///
/// assert_eq!(grade(95, 100), "优秀");
/// assert_eq!(grade(85, 100), "良好");
/// assert_eq!(grade(70, 100), "及格");
/// assert_eq!(grade(50, 100), "不及格");
/// ```
pub fn grade(score: u32, max_score: u32) -> &'static str {
    // 计算百分比
    let percentage = if max_score == 0 {
        0
    } else {
        (score * 100) / max_score
    };

    match percentage {
        90..=100 => "优秀",
        80..=89 => "良好",
        60..=79 => "及格",
        0..=59 => "不及格",
        _ => "无效分数",
    }
}

/// 根据成绩枚举返回评级
pub fn grade_from_enum(g: Grade) -> &'static str {
    match g {
        Grade::Excellent => "优秀",
        Grade::Good => "良好",
        Grade::Pass => "及格",
        Grade::Fail => "不及格",
    }
}

/// 返回交通灯的动作
pub fn traffic_action(light: TrafficLight) -> &'static str {
    match light {
        TrafficLight::Red => "停止",
        TrafficLight::Yellow => "准备",
        TrafficLight::Green => "通行",
    }
}

/// 返回交通灯的下一个状态
pub fn next_light(light: TrafficLight) -> TrafficLight {
    match light {
        TrafficLight::Red => TrafficLight::Green,
        TrafficLight::Yellow => TrafficLight::Red,
        TrafficLight::Green => TrafficLight::Yellow,
    }
}

/// 安全获取切片中的第二个元素
///
/// # 示例
///
/// ```
/// use module_04_patterns::second;
///
/// assert_eq!(second(&vec![1, 2, 3]), Some(&2));
/// assert_eq!(second(&vec![1]), None);
/// ```
pub fn second<T>(slice: &[T]) -> Option<&T> {
    match slice.get(1) {
        Some(value) => Some(value),
        None => None,
    }
}

/// 使用 if let 简化的版本
pub fn second_if_let<T>(slice: &[T]) -> Option<&T> {
    if let Some(value) = slice.get(1) {
        Some(value)
    } else {
        None
    }
}

/// 判断点所在的象限
pub fn quadrant(p: &Point) -> &'static str {
    match p {
        Point { x: 0, y: 0 } => "原点",
        Point { x: 0, .. } | Point { y: 0, .. } => "坐标轴",
        Point { x, y } if *x > 0 && *y > 0 => "第一象限",
        Point { x, y } if *x < 0 && *y > 0 => "第二象限",
        Point { x, y } if *x < 0 && *y < 0 => "第三象限",
        Point { x, y } if *x > 0 && *y < 0 => "第四象限",
        _ => "未知",
    }
}

/// 描述数字的特征
pub fn describe_number(n: i32) -> &'static str {
    match n {
        0 => "零",
        x if x > 0 && x % 2 == 0 => "正偶数",
        x if x > 0 && x % 2 != 0 => "正奇数",
        x if x < 0 && x % 2 == 0 => "负偶数",
        x if x < 0 && x % 2 != 0 => "负奇数",
        _ => "其他",
    }
}

/// 根据年龄返回类别
pub fn age_category(age: u32) -> String {
    match age {
        child @ 0..=12 => format!("儿童: {}岁", child),
        teen @ 13..=17 => format!("少年: {}岁", teen),
        adult @ 18..=64 => format!("成人: {}岁", adult),
        senior @ 65.. => format!("老人: {}岁", senior),
    }
}

/// 计算元组前三个元素的和
pub fn sum_first_three(tuple: (i32, i32, i32, i32, i32)) -> i32 {
    let (a, b, c, _, _) = tuple;
    a + b + c
}

/// 使用 .. 忽略剩余元素的版本
pub fn sum_first_three_alt(tuple: (i32, i32, i32, i32, i32)) -> i32 {
    let (a, b, c, ..) = tuple;
    a + b + c
}

/// 使用 @ 绑定和多个模式
pub fn match_number(n: i32) -> String {
    match n {
        0 => "零".to_string(),
        1..=3 => "小数字".to_string(),
        matched @ 4..=10 => format!("中等数字: {}", matched),
        matched @ 11..=100 => format!("大数字: {}", matched),
        _ => "超大数字".to_string(),
    }
}

/// 使用模式匹配判断月份季节
pub fn season(month: Month) -> &'static str {
    match month {
        Month::December | Month::January | Month::February => "冬季",
        Month::March | Month::April | Month::May => "春季",
        Month::June | Month::July | Month::August => "夏季",
        Month::September | Month::October | Month::November => "秋季",
    }
}

/// 反转方向
pub fn opposite_direction(dir: Direction) -> Direction {
    match dir {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::East => Direction::West,
        Direction::West => Direction::East,
    }
}

/// 计算形状面积
pub fn shape_area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
        Shape::Rectangle { width, height } => width * height,
        Shape::Triangle { base, height } => 0.5 * base * height,
    }
}

/// 检查列表中是否包含某个值
pub fn list_contains(list: &List, value: i32) -> bool {
    match list {
        List::Nil => false,
        List::Cons(item, rest) => {
            if *item == value {
                true
            } else {
                list_contains(rest, value)
            }
        }
    }
}

/// 使用 while let 的版本
pub fn list_contains_while(list: &List, value: i32) -> bool {
    let mut current = list;
    while let List::Cons(item, rest) = current {
        if *item == value {
            return true;
        }
        current = rest;
    }
    false
}

/// 计算链表长度
pub fn list_length(list: &List) -> usize {
    match list {
        List::Nil => 0,
        List::Cons(_, rest) => 1 + list_length(rest),
    }
}

/// 使用 matches! 宏检查值
pub fn is_small_option(opt: Option<i32>) -> bool {
    matches!(opt, Some(0..=9))
}

/// 检查点是否在特定象限
pub fn is_in_quadrant(p: &Point, q: u32) -> bool {
    matches!(
        (p.x, p.y),
        (x, y) if q == 1 && x > 0 && y > 0 ||
                  q == 2 && x < 0 && y > 0 ||
                  q == 3 && x < 0 && y < 0 ||
                  q == 4 && x > 0 && y < 0
    )
}

/// 嵌套解构示例
#[derive(Debug, Clone)]
pub struct Employee {
    pub name: String,
    pub role: Role,
}

/// 员工角色
#[derive(Debug, Clone)]
pub enum Role {
    Manager { department: String },
    Engineer { level: u32, language: String },
    Intern,
}

/// 描述员工
pub fn describe_employee(emp: &Employee) -> String {
    match emp {
        Employee {
            name,
            role: Role::Manager { department },
        } => format!("{} 是 {} 部门经理", name, department),
        Employee {
            name,
            role: Role::Engineer { level, language },
        } => format!("{} 是 {} 工程师，使用 {}", name, level, language),
        Employee {
            name,
            role: Role::Intern,
        } => format!("{} 是实习生", name),
    }
}

// ========== 测试 ==========

#[cfg(test)]
mod tests {
    use super::*;

    // 测试 grade 函数
    #[test]
    fn test_grade() {
        assert_eq!(grade(95, 100), "优秀");
        assert_eq!(grade(85, 100), "良好");
        assert_eq!(grade(70, 100), "及格");
        assert_eq!(grade(50, 100), "不及格");
        assert_eq!(grade(105, 100), "无效分数"); // 超过100%是无效分数
    }

    #[test]
    fn test_grade_from_enum() {
        assert_eq!(grade_from_enum(Grade::Excellent), "优秀");
        assert_eq!(grade_from_enum(Grade::Good), "良好");
        assert_eq!(grade_from_enum(Grade::Pass), "及格");
        assert_eq!(grade_from_enum(Grade::Fail), "不及格");
    }

    #[test]
    fn test_traffic_light() {
        assert_eq!(traffic_action(TrafficLight::Red), "停止");
        assert_eq!(traffic_action(TrafficLight::Yellow), "准备");
        assert_eq!(traffic_action(TrafficLight::Green), "通行");

        assert_eq!(next_light(TrafficLight::Red), TrafficLight::Green);
        assert_eq!(next_light(TrafficLight::Green), TrafficLight::Yellow);
        assert_eq!(next_light(TrafficLight::Yellow), TrafficLight::Red);
    }

    #[test]
    fn test_second() {
        assert_eq!(second(&vec![1, 2, 3]), Some(&2));
        assert_eq!(second(&vec![1]), None);
        assert_eq!(second(&Vec::<i32>::new()), None);
        assert_eq!(second_if_let(&vec![1, 2, 3]), Some(&2));
    }

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
    fn test_describe_number() {
        assert_eq!(describe_number(4), "正偶数");
        assert_eq!(describe_number(3), "正奇数");
        assert_eq!(describe_number(-4), "负偶数");
        assert_eq!(describe_number(-3), "负奇数");
        assert_eq!(describe_number(0), "零");
    }

    #[test]
    fn test_age_category() {
        assert_eq!(age_category(10), "儿童: 10岁");
        assert_eq!(age_category(15), "少年: 15岁");
        assert_eq!(age_category(30), "成人: 30岁");
        assert_eq!(age_category(70), "老人: 70岁");
    }

    #[test]
    fn test_sum_first_three() {
        assert_eq!(sum_first_three((1, 2, 3, 4, 5)), 6);
        assert_eq!(sum_first_three((10, 20, 30, 40, 50)), 60);
        assert_eq!(sum_first_three_alt((1, 2, 3, 4, 5)), 6);
    }

    #[test]
    fn test_match_number() {
        assert_eq!(match_number(0), "零");
        assert_eq!(match_number(2), "小数字");
        assert_eq!(match_number(5), "中等数字: 5");
        assert_eq!(match_number(50), "大数字: 50");
        assert_eq!(match_number(200), "超大数字");
    }

    #[test]
    fn test_season() {
        assert_eq!(season(Month::January), "冬季");
        assert_eq!(season(Month::April), "春季");
        assert_eq!(season(Month::July), "夏季");
        assert_eq!(season(Month::October), "秋季");
    }

    #[test]
    fn test_direction() {
        assert_eq!(opposite_direction(Direction::North), Direction::South);
        assert_eq!(opposite_direction(Direction::South), Direction::North);
        assert_eq!(opposite_direction(Direction::East), Direction::West);
        assert_eq!(opposite_direction(Direction::West), Direction::East);
    }

    #[test]
    fn test_shape_area() {
        let circle = Shape::Circle { radius: 1.0 };
        assert!((shape_area(&circle) - 3.14159).abs() < 0.01);

        let rect = Shape::Rectangle {
            width: 10.0,
            height: 5.0,
        };
        assert_eq!(shape_area(&rect), 50.0);

        let triangle = Shape::Triangle {
            base: 6.0,
            height: 4.0,
        };
        assert_eq!(shape_area(&triangle), 12.0);
    }

    #[test]
    fn test_list_operations() {
        let list = List::Cons(
            1,
            Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
        );

        assert!(list_contains(&list, 2));
        assert!(!list_contains(&list, 5));
        assert!(list_contains_while(&list, 3));

        assert_eq!(list_length(&list), 3);
        assert_eq!(list_length(&List::Nil), 0);
    }

    #[test]
    fn test_is_small_option() {
        assert!(is_small_option(Some(5)));
        assert!(is_small_option(Some(0)));
        assert!(is_small_option(Some(9)));
        assert!(!is_small_option(Some(10)));
        assert!(!is_small_option(None));
    }

    #[test]
    fn test_is_in_quadrant() {
        assert!(is_in_quadrant(&Point { x: 1, y: 1 }, 1));
        assert!(is_in_quadrant(&Point { x: -1, y: 1 }, 2));
        assert!(is_in_quadrant(&Point { x: -1, y: -1 }, 3));
        assert!(is_in_quadrant(&Point { x: 1, y: -1 }, 4));
    }

    #[test]
    fn test_point_methods() {
        let origin = Point::new(0, 0);
        assert!(origin.is_origin());
        assert!(origin.is_on_axis());

        let on_x = Point::new(5, 0);
        assert!(!on_x.is_origin());
        assert!(on_x.is_on_axis());

        let normal = Point::new(3, 4);
        assert!(!normal.is_origin());
        assert!(!normal.is_on_axis());
    }

    #[test]
    fn test_employee() {
        let manager = Employee {
            name: "Alice".to_string(),
            role: Role::Manager {
                department: "工程".to_string(),
            },
        };
        assert_eq!(describe_employee(&manager), "Alice 是 工程 部门经理");

        let engineer = Employee {
            name: "Bob".to_string(),
            role: Role::Engineer {
                level: 3,
                language: "Rust".to_string(),
            },
        };
        assert_eq!(describe_employee(&engineer), "Bob 是 3 工程师，使用 Rust");

        let intern = Employee {
            name: "Charlie".to_string(),
            role: Role::Intern,
        };
        assert_eq!(describe_employee(&intern), "Charlie 是实习生");
    }

    #[test]
    fn test_matches_macro() {
        let point = Point { x: 0, y: 5 };
        assert!(matches!(point, Point { x: 0, .. }));
        assert!(matches!(point, Point { y: 5, .. }));

        let opt = Some(5);
        assert!(matches!(opt, Some(0..=10)));
        assert!(!matches!(opt, Some(11..)));
    }
}
