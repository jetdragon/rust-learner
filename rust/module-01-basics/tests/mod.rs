// 01-基础入门 模块测试

// ========== 练习 1: 温度转换 ==========
#[test]
fn test_temperature() {
    // 测试正常情况
    let temp: f64 = 0.0;
    assert_eq!(temp * 9.0 / 5.0 + 32.0, 32.0);

    let temp: f64 = 100.0;
    assert_eq!(temp * 9.0 / 5.0 + 32.0, 212.0);

    // 测试负数
    let temp: f64 = -40.0;
    assert_eq!(temp * 9.0 / 5.0 + 32.0, -40.0);
}

// ========== 练习 2: 数组操作 ==========
#[test]
fn test_average() {
    let numbers = [1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    let avg = sum as f64 / numbers.len() as f64;
    assert_eq!(avg, 3.0);

    // 测试空数组
    let empty: [i32; 0] = [];
    let avg_empty = if empty.is_empty() {
        0.0
    } else {
        empty.iter().sum::<i32>() as f64 / empty.len() as f64
    };
    assert_eq!(avg_empty, 0.0);

    // 测试负数
    let neg_numbers = [-5, 0, 5];
    let sum_neg: i32 = neg_numbers.iter().sum();
    let avg_neg = sum_neg as f64 / neg_numbers.len() as f64;
    assert_eq!(avg_neg, 0.0);
}

// ========== 练习 3: 元组解构 ==========
#[test]
fn test_distance() {
    // 测试原点
    let point: (f64, f64, f64) = (0.0, 0.0, 0.0);
    let distance = (point.0.powi(2) + point.1.powi(2) + point.2.powi(2)).sqrt();
    assert_eq!(distance, 0.0);

    // 测试单位点
    let point: (f64, f64, f64) = (1.0, 0.0, 0.0);
    let distance = (point.0.powi(2) + point.1.powi(2) + point.2.powi(2)).sqrt();
    assert_eq!(distance, 1.0);

    // 测试 3-4-5 三角形
    let point: (f64, f64, f64) = (3.0, 4.0, 0.0);
    let distance = (point.0.powi(2) + point.1.powi(2) + point.2.powi(2)).sqrt();
    assert_eq!(distance, 5.0);
}

// ========== 练习 5: 函数式编程 ==========
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn is_positive(n: i32) -> bool {
    n > 0
}

fn check<F>(number: i32, condition: F) -> bool
where
    F: Fn(i32) -> bool,
{
    condition(number)
}

#[test]
fn test_functional() {
    assert!(check(4, is_even));
    assert!(!check(3, is_even));
    assert!(check(5, is_positive));
    assert!(!check(-5, is_positive));

    // 使用闭包
    assert!(check(10, |n| n > 5));
    assert!(!check(3, |n| n > 5));
}

// ========== 综合练习: BMI 计算器 ==========
fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / height_m.powi(2)
}

fn bmi_category(bmi: f64) -> &'static str {
    if bmi < 18.5 {
        "体重过轻"
    } else if bmi < 25.0 {
        "正常"
    } else if bmi < 30.0 {
        "超重"
    } else {
        "肥胖"
    }
}

#[test]
fn test_bmi() {
    // 测试正常 BMI
    let bmi = calculate_bmi(70.0, 1.75);
    assert!((bmi - 22.86).abs() < 0.01);
    assert_eq!(bmi_category(bmi), "正常");

    // 测试体重过轻
    let bmi = calculate_bmi(50.0, 1.75);
    assert!((bmi - 16.33).abs() < 0.01);
    assert_eq!(bmi_category(bmi), "体重过轻");

    // 测试超重
    let bmi = calculate_bmi(80.0, 1.70);
    assert!((bmi - 27.68).abs() < 0.01);
    assert_eq!(bmi_category(bmi), "超重");

    // 测试肥胖
    let bmi = calculate_bmi(100.0, 1.70);
    assert!((bmi - 34.60).abs() < 0.01);
    assert_eq!(bmi_category(bmi), "肥胖");
}
