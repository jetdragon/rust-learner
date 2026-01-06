//! # 练习 6 解答: @ 绑定
//!
//! **提示**：请先尝试自己实现，再参考此解答！
//!
//! # 实现思路
//!
//! 1. @ 绑定：在匹配的同时创建变量绑定
//! 2. 语法：`pattern @ variable`
//! 3. 优势：既匹配模式又保留值
//! 4. 用途：在范围匹配后使用实际值

/// 年龄分类
pub fn age_category(age: u32) -> String {
    match age {
        a @ 0..=12 => format!("儿童: {}岁", a),
        a @ 13..=17 => format!("少年: {}岁", a),
        a @ 18..=64 => format!("成人: {}岁", a),
        a => format!("老人: {}岁", a),
    }
}

/// 使用 @ 绑定简化模式匹配
pub fn score_description(score: i32) -> String {
    match score {
        s @ 90..=100 => format!("优秀: {}分", s),
        s @ 60..=89 => format!("及格: {}分", s),
        s @ 0..=59 => format!("不及格: {}分", s),
        _ => "无效分数".to_string(),
    }
}

/// 嵌套的 @ 绑定
pub fn analyze_point(x: i32, y: i32) -> String {
    match (x, y) {
        (0, 0) => "原点".to_string(),
        p @ (_, _) if p.0.abs() == p.1.abs() => format!("在对角线上: {:?}", p),
        (x, y) => format!("普通点: ({}, {})", x, y),
    }
}

fn main() {
    let ages = [5, 10, 15, 20, 30, 70];

    println!("=== 年龄分类 ===\n");
    for age in ages {
        println!("{}岁: {}", age, age_category(age));
    }

    // 分数描述
    println!("\n=== 分数描述 ===");
    for score in [95, 75, 45, -5] {
        println!("{}分: {}", score, score_description(score));
    }

    // 点分析
    println!("\n=== 点分析 ===");
    for &(x, y) in &[(0, 0), (3, 3), (2, 5)] {
        println!("({}, {}): {}", x, y, analyze_point(x, y));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_age_category() {
        assert_eq!(age_category(10), "儿童: 10岁");
        assert_eq!(age_category(15), "少年: 15岁");
        assert_eq!(age_category(30), "成人: 30岁");
        assert_eq!(age_category(70), "老人: 70岁");
    }

    #[test]
    fn test_score_description() {
        assert_eq!(score_description(85), "优秀: 85分");
        assert_eq!(score_description(60), "及格: 60分");
        assert_eq!(score_description(40), "不及格: 40分");
    }

    #[test]
    fn test_analyze_point() {
        assert_eq!(analyze_point(0, 0), "原点");
        assert!(analyze_point(3, 3).contains("对角线"));
        assert!(analyze_point(2, 5).contains("普通点"));
    }

    #[test]
    fn test_at_binding_preserves_value() {
        // @ 绑定保留匹配的值
        let result = match 25 {
            n @ 20..=30 => n * 2, // 可以使用 n 的值
            _ => 0,
        };
        assert_eq!(result, 50);
    }

    #[test]
    fn test_nested_at_binding() {
        // 嵌套结构中的 @ 绑定
        let point = Some((10, 20));
        match point {
            Some(p @ (x, y)) => {
                assert_eq!(p, (10, 20));
                assert_eq!(x, 10);
                assert_eq!(y, 20);
            }
            None => panic!("不应该到这"),
        }
    }
}
