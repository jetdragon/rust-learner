//! # 练习 6: @ 绑定
//!
//! **难度**: 中等
//! **预计时间**: 15 分钟

/// TODO: 年龄分类
///
/// 使用 @ 绑定，同时匹配范围和保留值：
/// - 0-12: 返回 "儿童: {age}岁"
/// - 13-17: 返回 "少年: {age}岁"
/// - 18-64: 返回 "成人: {age}岁"
/// - 65+: 返回 "老人: {age}岁"
pub fn age_category(age: u32) -> String {
    match age {
        a @ 0..=12 => format!("儿童: {}岁", a),
        a @ 13..=17 => format!("少年: {}岁", a),
        a @ 18..=64 => format!("成人: {}岁", a),
        a => format!("老人: {}岁", a),
    }
}

/// TODO: 使用 @ 绑定简化模式匹配
pub fn score_description(score: i32) -> String {
    unimplemented!()
}

fn main() {
    let ages = [5, 10, 15, 20, 30, 70];

    println!("=== 年龄分类 ===\n");
    for age in ages {
        println!("{}岁: {}", age, age_category(age));
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
}
