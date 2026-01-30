// 04-模式匹配 模块集成测试

use module_04_patterns::*;

// ========== 练习 1: 基本 match 表达式 ==========
#[test]
fn test_grade_function() {
    assert_eq!(grade(95, 100), "优秀");
    assert_eq!(grade(85, 100), "良好");
    assert_eq!(grade(70, 100), "及格");
    assert_eq!(grade(50, 100), "不及格");
    assert_eq!(grade(105, 100), "无效分数");
}

// ========== 练习 2: 匹配 Option ==========
#[test]
fn test_second_function() {
    assert_eq!(second(&vec![1, 2, 3]), Some(&2));
    assert_eq!(second(&vec![1]), None);
    assert_eq!(second(&Vec::<i32>::new()), None);

    // 测试 if let 版本
    assert_eq!(second_if_let(&vec![1, 2, 3]), Some(&2));
    assert_eq!(second_if_let(&vec![1]), None);
}

// ========== 练习 3: 解构结构体 ==========
#[test]
fn test_quadrant_function() {
    assert_eq!(quadrant(&Point { x: 1, y: 1 }), "第一象限");
    assert_eq!(quadrant(&Point { x: -1, y: 1 }), "第二象限");
    assert_eq!(quadrant(&Point { x: -1, y: -1 }), "第三象限");
    assert_eq!(quadrant(&Point { x: 1, y: -1 }), "第四象限");
    assert_eq!(quadrant(&Point { x: 0, y: 0 }), "原点");
    assert_eq!(quadrant(&Point { x: 0, y: 5 }), "坐标轴");
}

// ========== 练习 4: 匹配枚举 ==========
#[test]
fn test_traffic_light_functions() {
    assert_eq!(traffic_action(TrafficLight::Red), "停止");
    assert_eq!(traffic_action(TrafficLight::Yellow), "准备");
    assert_eq!(traffic_action(TrafficLight::Green), "通行");

    assert_eq!(next_light(TrafficLight::Red), TrafficLight::Green);
    assert_eq!(next_light(TrafficLight::Green), TrafficLight::Yellow);
    assert_eq!(next_light(TrafficLight::Yellow), TrafficLight::Red);
}

#[test]
fn test_grade_enum() {
    assert_eq!(grade_from_enum(Grade::Excellent), "优秀");
    assert_eq!(grade_from_enum(Grade::Good), "良好");
    assert_eq!(grade_from_enum(Grade::Pass), "及格");
    assert_eq!(grade_from_enum(Grade::Fail), "不及格");
}

// ========== 练习 5: 使用守卫 ==========
#[test]
fn test_describe_number() {
    assert_eq!(describe_number(4), "正偶数");
    assert_eq!(describe_number(3), "正奇数");
    assert_eq!(describe_number(-4), "负偶数");
    assert_eq!(describe_number(-3), "负奇数");
    assert_eq!(describe_number(0), "零");
}

// ========== 练习 6: @ 绑定 ==========
#[test]
fn test_age_category() {
    assert_eq!(age_category(10), "儿童: 10岁");
    assert_eq!(age_category(15), "少年: 15岁");
    assert_eq!(age_category(30), "成人: 30岁");
    assert_eq!(age_category(70), "老人: 70岁");
}

#[test]
fn test_match_number() {
    assert_eq!(match_number(0), "零");
    assert_eq!(match_number(2), "小数字");
    assert_eq!(match_number(5), "中等数字: 5");
    assert_eq!(match_number(50), "大数字: 50");
    assert_eq!(match_number(200), "超大数字");
}

// ========== 练习 7: 忽略值 ==========
#[test]
fn test_sum_first_three() {
    assert_eq!(sum_first_three((1, 2, 3, 4, 5)), 6);
    assert_eq!(sum_first_three((10, 20, 30, 40, 50)), 60);
    assert_eq!(sum_first_three_alt((1, 2, 3, 4, 5)), 6);
}

// ========== 练习 8: 多个模式 ==========
#[test]
fn test_season_function() {
    assert_eq!(season(Month::January), "冬季");
    assert_eq!(season(Month::February), "冬季");
    assert_eq!(season(Month::December), "冬季");
    assert_eq!(season(Month::March), "春季");
    assert_eq!(season(Month::April), "春季");
    assert_eq!(season(Month::May), "春季");
    assert_eq!(season(Month::June), "夏季");
    assert_eq!(season(Month::July), "夏季");
    assert_eq!(season(Month::August), "夏季");
    assert_eq!(season(Month::September), "秋季");
    assert_eq!(season(Month::October), "秋季");
    assert_eq!(season(Month::November), "秋季");
}

// ========== 练习 9: 方向反转 ==========
#[test]
fn test_opposite_direction() {
    assert_eq!(opposite_direction(Direction::North), Direction::South);
    assert_eq!(opposite_direction(Direction::South), Direction::North);
    assert_eq!(opposite_direction(Direction::East), Direction::West);
    assert_eq!(opposite_direction(Direction::West), Direction::East);
}

// ========== 练习 10: 解构枚举 ==========
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

// ========== 链表操作测试 ==========
#[test]
fn test_list_operations() {
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );

    assert!(list_contains(&list, 2));
    assert!(!list_contains(&list, 5));
    assert!(list_contains_while(&list, 3));
    assert!(!list_contains_while(&list, 10));

    assert_eq!(list_length(&list), 3);
    assert_eq!(list_length(&List::Nil), 0);
}

// ========== Point 方法测试 ==========
#[test]
fn test_point_methods() {
    let origin = Point::new(0, 0);
    assert!(origin.is_origin());
    assert!(origin.is_on_axis());

    let on_x = Point::new(5, 0);
    assert!(!on_x.is_origin());
    assert!(on_x.is_on_axis());

    let on_y = Point::new(0, 5);
    assert!(!on_y.is_origin());
    assert!(on_y.is_on_axis());

    let normal = Point::new(3, 4);
    assert!(!normal.is_origin());
    assert!(!normal.is_on_axis());
}

// ========== 员工角色测试 ==========
#[test]
fn test_employee_descriptions() {
    let manager = Employee {
        name: "Alice".to_string(),
        role: Role::Manager {
            department: "工程".to_string(),
        },
    };
    assert!(describe_employee(&manager).contains("Alice"));
    assert!(describe_employee(&manager).contains("工程"));

    let engineer = Employee {
        name: "Bob".to_string(),
        role: Role::Engineer {
            level: 3,
            language: "Rust".to_string(),
        },
    };
    assert!(describe_employee(&engineer).contains("Bob"));
    assert!(describe_employee(&engineer).contains("3"));
    assert!(describe_employee(&engineer).contains("Rust"));

    let intern = Employee {
        name: "Charlie".to_string(),
        role: Role::Intern,
    };
    assert!(describe_employee(&intern).contains("Charlie"));
    assert!(describe_employee(&intern).contains("实习生"));
}

// ========== matches! 宏测试 ==========
#[test]
fn test_is_small_option() {
    assert!(is_small_option(Some(5)));
    assert!(is_small_option(Some(0)));
    assert!(is_small_option(Some(9)));
    assert!(!is_small_option(Some(10)));
    assert!(!is_small_option(None));
}

#[test]
fn test_matches_macro_in_quadrant() {
    assert!(is_in_quadrant(&Point { x: 1, y: 1 }, 1));
    assert!(is_in_quadrant(&Point { x: -1, y: 1 }, 2));
    assert!(is_in_quadrant(&Point { x: -1, y: -1 }, 3));
    assert!(is_in_quadrant(&Point { x: 1, y: -1 }, 4));

    // 测试不在象限的情况
    assert!(!is_in_quadrant(&Point { x: 1, y: 1 }, 2));
    assert!(!is_in_quadrant(&Point { x: 0, y: 0 }, 1));
}

// ========== 综合测试 ==========
#[test]
fn test_comprehensive_pattern_matching() {
    // 测试 grade 函数
    assert_eq!(grade(90, 100), "优秀");

    // 测试象限
    let p = Point::new(3, 4);
    assert_eq!(quadrant(&p), "第一象限");

    // 测试交通灯
    let light = TrafficLight::Red;
    assert_eq!(next_light(light), TrafficLight::Green);

    // 测试数字描述
    assert_eq!(describe_number(42), "正偶数");

    // 测试链表
    let list = List::Cons(10, Box::new(List::Nil));
    assert!(list_contains(&list, 10));
    assert_eq!(list_length(&list), 1);

    // 测试形状
    let shape = Shape::Rectangle {
        width: 5.0,
        height: 4.0,
    };
    assert_eq!(shape_area(&shape), 20.0);
}

// ========== 边界条件测试 ==========
#[test]
fn test_edge_cases() {
    // 测试年龄边界
    assert_eq!(age_category(0), "儿童: 0岁");
    assert_eq!(age_category(12), "儿童: 12岁");
    assert_eq!(age_category(13), "少年: 13岁");
    assert_eq!(age_category(17), "少年: 17岁");
    assert_eq!(age_category(18), "成人: 18岁");
    assert_eq!(age_category(64), "成人: 64岁");
    assert_eq!(age_category(65), "老人: 65岁");
    assert_eq!(age_category(100), "老人: 100岁");

    // 测试数字分类边界
    assert_eq!(describe_number(i32::MIN), "负偶数");
    assert_eq!(describe_number(i32::MAX), "正奇数");

    // 测试空链表
    assert_eq!(list_length(&List::Nil), 0);
    assert!(!list_contains(&List::Nil, 5));
}

// ========== 复杂模式测试 ==========
#[test]
fn test_complex_patterns() {
    // 测试季节覆盖所有月份
    let all_months = [
        Month::January,
        Month::February,
        Month::March,
        Month::April,
        Month::May,
        Month::June,
        Month::July,
        Month::August,
        Month::September,
        Month::October,
        Month::November,
        Month::December,
    ];

    for month in all_months {
        let s = season(month);
        assert!(["冬季", "春季", "夏季", "秋季"].contains(&s));
    }

    // 测试所有方向可逆
    for dir in [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ] {
        assert_eq!(opposite_direction(opposite_direction(dir)), dir);
    }
}
